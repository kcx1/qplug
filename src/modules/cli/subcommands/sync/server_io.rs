use serde_json::Value;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::select;
use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use crate::cli::subcommands::sync::qrc_api::QrcMessage;

pub async fn write_server_messages(
    mut write_stream: OwnedWriteHalf,
    mut write_rx: mpsc::Receiver<QrcMessage>,
    cancel_token: CancellationToken,
) -> anyhow::Result<()> {
    loop {
        select! {
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

// pub async fn read_server_messages(
//     mut stream: OwnedReadHalf,
//     read_tx: mpsc::Sender<Value>,
//     cancel_token: CancellationToken,
// ) -> anyhow::Result<String> {
//     // create mutable buffer 1kb
//     let mut read_buf = [0; 32768];
//
//     loop {
//         select! {
//             _ = cancel_token.cancelled() => {
//                 println!("Cancel Token has been cancelled");
//                 return Ok("Reading Socket has closed.".into())
//             }
//                 result = stream.read(&mut read_buf) => {
//                 println!("{result:?}");
//                 match result {
//                     Ok(buf_len) => {
//
//                         if buf_len > 0 {
//                             // convert the buffer into a string only using the length of data in the buffer
//                             let raw_message = String::from_utf8_lossy(&read_buf[..buf_len])
//                                 .to_string()
//                                 .replace("\0", "");
//                             let message: Value = serde_json::from_str(&raw_message)?;
//                                 println!("Message Thread has received data: {}", &message);
//                             if let Err(e) = read_tx.send(message.clone()).await {
//                                 println!(
//                                     "read_server_message Error: There was an error sending the message to the channel {:?}",
//                                     e
//                                 );
//                                 println!("Tried sending: {:?}", &message);
//                                 break;
//                             }
//                         }
//                     },
//                     Err(e) => {
//                         return Err(e.into());
//                     }
//                 }
//
//
//             }
//         }
//     }
//     Ok("Reading Socket has closed.".into())
// }

pub async fn read_server_messages(
    mut stream: OwnedReadHalf,
    read_tx: mpsc::Sender<Value>,
) -> anyhow::Result<String> {
    let mut buffer = [0; 512];
    let mut message_buffer = Vec::new();

    loop {
        match stream.read(&mut buffer).await {
            Ok(buf_len) => {
                if buf_len == 0 {
                    break; // Connection closed
                }
                // Push bytes into the message_buffer until we reach a null byte
                for &byte in &buffer[..buf_len] {
                    message_buffer.push(byte);
                    //QRC Messages are NULL byte delitmited; so check if the next byte is 0, which
                    //indicates the end of a frame
                    if byte == 0 {
                        // Construct our message based on the framed message_buffer
                        let raw_message =
                            String::from_utf8(message_buffer.clone())?.replace("\0", "");
                        let message: Value = serde_json::from_str(&raw_message)?;
                        if let Err(e) = read_tx.send(message).await {
                            println!(
                                "There was an error sending the message to the channel {:?}",
                                e
                            );
                            break;
                        }
                        // Clear the buffer for the next message
                        message_buffer.clear();
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        }
    }
    Ok("Reading Socket has closed.".into())
}
