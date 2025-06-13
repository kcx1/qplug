use serde_json::Value;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::select;
use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use crate::cli::subcommands::sync::qrc_api::JsonRpcCodec;

pub async fn write_server_messages(
    mut write_stream: OwnedWriteHalf,
    mut write_rx: mpsc::Receiver<JsonRpcCodec>,
    cancel_token: CancellationToken,
) -> anyhow::Result<()> {
    loop {
        select! {
            _exit =  tokio::signal::ctrl_c() => {
                // Wait for a termination signal to close the connection gracefully.
                println!("Received Ctrl+C, closing connection...");
                cancel_token.cancel();
                write_rx.close();
                write_stream.shutdown().await?;
                break;
            }
            _ = cancel_token.cancelled() => {
                write_rx.close();
                write_stream.shutdown().await?;
                return Ok(())
        }

            payload = write_rx.recv() => {
                    match payload {
                        Some(payload) => {

                            write_stream
                                .write_all(&payload.format()?.as_bytes())
                            .await?;
                        },
                        None => break
                    }
                }
        }
    }
    Ok(())
}

pub async fn read_server_messages(
    mut stream: OwnedReadHalf,
    read_tx: mpsc::Sender<Value>,
    cancel_token: CancellationToken,
) -> anyhow::Result<String> {
    // create mutable buffer 1kb
    let mut read_buf = [0; 1024];

    loop {
        select! {
            _exit =  tokio::signal::ctrl_c() => {
                // Wait for a termination signal to close the connection gracefully.
                println!("Received Ctrl+C, closing connection...");
                cancel_token.cancel();
                break;
            }
            _ = cancel_token.cancelled() => {
                return Ok("Reading Socket has closed.".into())
            }
                result = stream.read(&mut read_buf) => {
                match result {
                    Ok(0) => cancel_token.cancel(),
                    Ok(buf_len) => {

                        if buf_len > 0 {
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
                        } else {
                            return Ok("Reading Socket has closed.".into());
                        }
                    },
                    Err(e) => {
                        return Err(e.into());
                    }
                }


            }
        }
    }
    Ok("Reading Socket has closed.".into())
}
