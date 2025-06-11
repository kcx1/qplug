use std::path::PathBuf;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use anyhow::Context;

use async_watcher::notify::RecursiveMode;
use async_watcher::AsyncDebouncer;

use serde_json::Value;

use super::config::{QsysCore, Target};
use crate::cli::subcommands::sync::qrc_api::{handle_server_reply, JsonRpcCodec, ServerReply};

static QRS_PORT: u16 = 1710;

struct QsysServer {
    port: u16,
    client: QsysCore,
}

impl QsysServer {
    ///Creates a new QRS instance
    fn new(port: u16, client: QsysCore) -> Self {
        QsysServer { port, client }
    }

    ///Connects to the QSYS server
    async fn connect(&mut self) -> anyhow::Result<TcpStream> {
        let connection_timeout: u64 = 30;
        // Connect to the stream
        let connection = match tokio::time::timeout(
            Duration::from_secs(connection_timeout),
            TcpStream::connect(format!("{}:{}", self.client.hostname, self.port)),
        )
        .await
        .context("Unable to connect to QSYS")
        {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("Timeout while connecting to QSYS: {:?}", e);
                std::process::exit(1)
            }
        }?;
        Ok(connection)
    }
}

async fn write_server_messages(
    mut write_stream: OwnedWriteHalf,
    mut write_rx: mpsc::Receiver<JsonRpcCodec>,
) -> anyhow::Result<()> {
    loop {
        match write_rx.recv().await {
            Some(payload) => {
                write_stream
                    .write_all(&payload.format()?.as_bytes())
                    .await?;
            }
            None => break,
        };
    }
    Ok(())
}

async fn read_server_messages(
    mut stream: OwnedReadHalf,
    read_tx: mpsc::Sender<Value>,
) -> anyhow::Result<String> {
    // create mutable buffer 1kb
    let mut read_buf = [0; 1024];
    // read the buffer

    loop {
        match stream.read(&mut read_buf).await {
            Ok(buf_len) => {
                // convert the buffer into a string only using the length of data in the buffer
                let raw_message = String::from_utf8_lossy(&read_buf[..buf_len])
                    .to_string()
                    .replace("\0", "");
                let message: Value = serde_json::from_str(&raw_message)?;
                if let Err(e) = read_tx.send(message).await {
                    println!(
                        "There was an error sending the message to the channel {:?}",
                        e
                    );
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error sending message: {}", e);
                break;
            }
        }
    }

    Ok("Reading Socket has closed.".into())
    // Ok(message.into_owned())
}

#[tokio::main]
pub async fn sync(script: &PathBuf, config: &PathBuf) -> anyhow::Result<()> {
    let target = Target::from_config(config)?;

    // Create a QRS instance
    let mut qsys_core = QsysServer::new(QRS_PORT, target.core.clone());

    // Create listener and sender
    let (read_tx, mut read_rx) = mpsc::channel(128);
    let (write_tx, write_rx) = mpsc::channel(128);

    // Connect to the stream
    let connection = qsys_core.connect().await?;
    let (read_stream, write_stream) = connection.into_split();

    // Spawn Read Message Handler Task
    tokio::spawn(async move { read_server_messages(read_stream, read_tx).await });

    // Spawn Write Message Handler Task
    tokio::spawn(async move { write_server_messages(write_stream, write_rx).await });

    // Spawn the notify loop
    let (mut debouncer, mut file_events) =
        AsyncDebouncer::new_with_channel(Duration::from_secs(1), Some(Duration::from_secs(1)))
            .await?;

    debouncer
        .watcher()
        .watch(&script, RecursiveMode::NonRecursive)?;

    // let mut messages = vec![];
    let mut async_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    // let mut interval = time::interval(std::time::Duration::from_secs(30));

    // let sync_code = std::fs::read_to_string(script)?;
    // if let Err(e) = write_tx
    //     .send(JsonRpcCodec::update_code(&target.component.name, sync_code).await?)
    //     .await
    // {
    //     eprintln!("Failed to send payload, {e}");
    // }

    loop {
        tokio::select! {
        message = read_rx.recv() => {
            if let Some(msg) = message {
                    handle_server_reply(&ServerReply::new(msg)?);
                    // messages.push(&msg);
                }
        }

        _noop = async_interval.tick() => {
            let payload = JsonRpcCodec::noop()?;
            println!("Payload: {:?}", payload);
            if let Err(e) = write_tx.send(payload).await {
                eprintln!("Failed to send payload, {e}");
                break;
            }
        }

        _exit =  tokio::signal::ctrl_c() => {
            // Wait for a termination signal to close the connection gracefully.
            println!("Received Ctrl+C, closing connection...");
            break;
        }

        _optional_break = tokio::time::sleep(tokio::time::Duration::from_secs(8)) => {
            break;
        }

        file_event = file_events.recv() => {
                println!("File event: {file_event:?}");
        }


            // _component_controls  {
            //     // let payload = JsonRpcCodec::get_component_controls(&target.component.name).unwrap();
            //     // println!("Payload: {:?}", payload);
            //     // if let Err(_e) = write_tx.send(payload).await {
            //     //     eprintln!("Failed to send payload");
            //     //     break;
            //     // }
            // }

        }
    }
    Ok(())
}
