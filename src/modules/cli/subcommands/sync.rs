#![allow(dead_code)]

use std::{io::stdin, path::PathBuf};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

use anyhow::{Context, Ok};
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

async fn write_server_messages(
    mut write_stream: OwnedWriteHalf,
    sender: tokio::sync::mpsc::Sender<JsonRpcCodec>,
) -> anyhow::Result<OwnedWriteHalf> {
    loop {
        let payload = match sender.recv().await {
            Ok(payload) => payload,
            Err(_) => break,
        };
    }
    println!("Writing to server");
    // self.connection = Some(self.connect().await?);
    write_stream.write_all(payload.format()?.as_bytes()).await?;
    Ok(write_stream)
}

async fn read_server_messages<T>(mut stream: OwnedReadHalf) -> anyhow::Result<String> {
    println!("Reading from server");
    loop {
        // Create a condition to disconnect
        let mut read_buf = [0; 1024];
        // create mutable buffer 1kb
        // read the buffer
        let buf_len = stream.read(&mut read_buf).await?;

        // convert the buffer into a string only using the length of data in the buffer
        let message = String::from_utf8_lossy(&read_buf[..buf_len]);

        if message.len() > 1 {
            println!("{message}");
        }
    }

    // Ok(message.into_owned())
}

#[tokio::main]
pub async fn sync(script: &PathBuf, config: &PathBuf) -> anyhow::Result<()> {
    let target = Target::from_config(config)?;
    println!("{}, {:?}", script.display(), target);

    // Create a QRS instance

    let qrs = QRS::new(QRS_PORT, target.cores[0].clone());

    // Connect to the stream
    let connection = qrs.connect().await?;
    let (read_stream, write_stream) = connection.into_split();
    tokio::spawn(async move {
        read_server_messages::<()>(read_stream).await.unwrap();
    });
    //Write Login Payload
    write_server_messages(write_stream, JsonRpcCodec::login(None, None)?).await?;
    // Write a get component controls payload
    // write_server_messages(
    //     write_stream,
    //     Payload::get_component_controls(target.component.name)?,
    // )
    // .await?;
    // write_server_messages(write_stream, JsonRpcCodec::noop()?).await?;
    //

    // Wait for a termination signal to close the connection gracefully.
    tokio::signal::ctrl_c().await?;
    println!("Received Ctrl+C, closing connection...");

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
