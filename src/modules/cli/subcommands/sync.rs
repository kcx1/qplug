#![allow(dead_code)]
use bytes::Buf;
use bytes::{BufMut, BytesMut};
use std::io;
use std::{io::stdin, path::PathBuf};
use tokio_util::codec::{Decoder, Encoder};
use tokio_util::codec::{Framed, FramedRead, FramedWrite};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct QsysCore {
    hostname: String,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Component {
    name: String,
    // id: String,
    // r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    cores: Vec<QsysCore>,
    component: Component,
}

impl Target {
    // The problem wiht this funciton is that it is an all or nothing match. If a config is found
    // EVERTHING is in the config. Otherwise the user is PROMPTED for everything. This needs to
    // blend the approach and fill in whatever is missing.
    fn new(config: Option<PathBuf>) -> anyhow::Result<Target> {
        match config {
            Some(config) => Ok(serde_json::from_reader(&mut std::fs::File::open(config)?)?),
            None => {
                // TODO: Create function to fill out the config.
                let mut name = String::new();
                stdin().read_line(&mut name)?;
                Ok(Target {
                    cores: vec![],
                    component: Component { name },
                })
            }
        }
    }

    fn from_config(config: &PathBuf) -> anyhow::Result<Target> {
        Ok(serde_json::from_reader(std::fs::File::open(config)?)?)
    }
}

static QRS_PORT: u16 = 1710;

struct QRS {
    port: u16,
    client: QsysCore,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcCodec {
    jsonrpc: String,
    method: String,
    params: Map<String, Value>,
}

impl JsonRpcCodec {
    fn new(method: String, params: Map<String, Value>) -> Self {
        let jsonrpc = String::from("2.0");
        JsonRpcCodec {
            jsonrpc,
            method,
            params,
        }
    }

    ///Login to the QSYS server. Provide username and password if needed
    fn login(username: Option<String>, password: Option<String>) -> anyhow::Result<JsonRpcCodec> {
        let mut parameters = Map::new();
        if let Some(username) = username {
            parameters.insert("User".to_string(), serde_json::Value::String(username));
        }
        if let Some(password) = password {
            parameters.insert("Password".to_string(), serde_json::Value::String(password));
        }
        Ok(JsonRpcCodec::new(String::from("Logon"), parameters))
    }

    ///Send a no op to the server. MUST be sent every 60 seconds to keep the session alive
    fn noop() -> anyhow::Result<JsonRpcCodec> {
        Ok(JsonRpcCodec::new(String::from("NoOp"), Map::new()))
    }

    fn get_component_controls(componet_name: String) -> anyhow::Result<JsonRpcCodec> {
        let mut parameters = Map::new();

        parameters.insert("Name".into(), componet_name.into());
        Ok(JsonRpcCodec::new(
            String::from("Component.GetControls"),
            parameters,
        ))
    }

    fn format(self) -> anyhow::Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}

impl Decoder for JsonRpcCodec {
    type Item = Value;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(pos) = buf.iter().position(|b| *b == b'\0') {
            let line = buf.split_to(pos);
            buf.advance(1); // skip null terminator

            serde_json::from_slice::<Value>(&line)
                .map(Some)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<Value> for JsonRpcCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Value, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let json = serde_json::to_string(&item)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        dst.put(json.as_bytes());
        dst.put_u8(0); // null terminator
        Ok(())
    }
}

impl QRS {
    ///Creates a new QRS instance
    fn new(port: u16, client: QsysCore) -> Self {
        QRS { port, client }
    }

    ///Connects to the QSYS server
    async fn connect(&self) -> anyhow::Result<TcpStream> {
        TcpStream::connect(format!("{}:{}", self.client.hostname, self.port))
            .await
            .context(format!(
                "Could Not connect to the server: {}:{}",
                self.client.hostname, self.port
            ))
    }

    // fn sync(&mut self, script: &PathBuf, target: &Target) -> anyhow::Result<()> {
    //     Ok(())
    // }
}

async fn write_server_messages<S>(
    mut writer: FramedWrite<S, JsonRpcCodec>,
    payload: JsonRpcCodec,
) -> anyhow::Result<FramedWrite<S, JsonRpcCodec>>
where
    S: tokio::io::AsyncWrite + Unpin,
{
    writer.send(payload).await?;
    Ok(writer)
}

async fn read_server_messages(socket: TcpStream, codec: JsonRpcCodec) -> anyhow::Result<()> {
    let mut reader = FramedRead::new(socket, codec);

    while let Some(frame) = reader.next().await {
        let msg = frame?;
        println!("Received: {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
pub async fn sync(script: &PathBuf, config: &PathBuf) -> anyhow::Result<()> {
    let target = Target::from_config(config)?;
    println!("{}, {:?}", script.display(), target);

    // Create a QRS instance

    let qrs = QRS::new(QRS_PORT, target.cores[0].clone());

    // Connect to the stream
    let (read_stream, write_stream) = qrs.connect().await?.into_split();
    tokio::spawn(async move {
        let msg = read_server_messages(read_stream).await;
        println!("{:?}", msg.unwrap());
    });
    //Write Login Payload
    write_server_messages(write_stream, JsonRpcCodec::login(None, None)?).await?;
    // Write a get component controls payload
    // write_server_messages(
    //     write_stream,
    //     Payload::get_component_controls(target.component.name)?,
    // )
    // .await?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//
//     use std::io::Read;
//
//     use crate::cli::subcommands::sync::QsysCore;
//     use crate::cli::subcommands::sync::QRS;
//     use crate::cli::subcommands::sync::QRS_PORT;

// #[test]
//     async fn test_login() -> anyhow::Result<()> {
//         let core = QsysCore {
//             hostname: "10.8.0.3".to_string(),
//             username: None,
//             password: None,
//         };
//         let mut stream = QRS::new(QRS_PORT, core);
//         stream.login(None, None).await?;
//         let mut stream_buf = String::new();
//         // loop {
//         //     match stream.connection {
//         //         Some(ref mut connection) => {
//         //             connection
//         //                 .read_to_string(&mut stream_buf)
//         //                 .expect("Failed to read from connection");
//         //
//         //             println!("{}", stream_buf);
//         //             break;
//         //         }
//         //         None => break,
//         //     };
//         // }
//         Ok(())
//     }
// }
