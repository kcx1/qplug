use std::path::PathBuf;
use std::time::Duration;
use tokio::net::TcpStream;

use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use anyhow::Context;

use async_watcher::notify::RecursiveMode;
use async_watcher::AsyncDebouncer;

use super::sync_config::{QsysCore, Target};
use crate::cli::subcommands::sync::qrc_api::{handle_server_reply, JsonRpcCodec, ServerReply};
use crate::cli::subcommands::sync::server_io::{read_server_messages, write_server_messages};
use crate::cli::subcommands::sync::sync_config::Component;

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
        println!(
            "\nConnecting to the core at {}:{} ...",
            self.client.hostname, self.port
        );
        let connection_timeout: u64 = 30;
        // Connect to the stream
        let connection = match tokio::time::timeout(
            Duration::from_secs(connection_timeout),
            TcpStream::connect(format!("{}:{}", self.client.hostname, self.port)),
        )
        .await
        .context(format!(
            "Unable to connect to the core at {}:{}",
            self.client.hostname, self.port
        )) {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("\nTimeout while connecting to QSYS: {:?}", e);
                std::process::exit(1)
            }
        }?;
        Ok(connection)
    }
}

async fn sync_code(
    file_path: &PathBuf,
    sock_write_channel: &mpsc::Sender<JsonRpcCodec>,
    cancel_token_file: &CancellationToken,
    component: &Component,
) {
    // Create the code path dynamically depending on how they were declared
    let code_path = match &component.script {
        Some(script) => match script.is_absolute() {
            true => script,
            false => &file_path.join(script),
        },
        None => &file_path.join(&component.name),
    };

    let mut lua_file = code_path.clone();
    lua_file.set_extension("lua");

    println!("{lua_file:?}");
    let sync_code = std::fs::read_to_string(lua_file).unwrap();

    if let Err(e) = sock_write_channel
        .send(
            JsonRpcCodec::update_code(component.name.as_ref(), sync_code)
                .await
                .unwrap(),
        )
        .await
    {
        eprintln!("Failed to send payload, {e}");
        cancel_token_file.cancel();
    }
}

#[tokio::main]
pub async fn sync(script: &PathBuf, config: &PathBuf, is_watch: bool) -> anyhow::Result<()> {
    if !script.is_dir() {
        anyhow::bail!("Please use a direcotry not a file")
    }

    let target = Target::from_config(config)?;

    let recursion_mode = match target.core.components.len() == 1 {
        true => RecursiveMode::NonRecursive,
        false => RecursiveMode::Recursive,
    };

    // Create a QRS instance
    let mut qsys_core = QsysServer::new(QRS_PORT, target.core.clone());

    // Create listener and sender
    let (read_thread, mut socket_read) = mpsc::channel(128);
    let (socket_write, write_thread) = mpsc::channel(128);

    // Connect to the stream
    let connection = qsys_core.connect().await?;
    let (read_stream, write_stream) = connection.into_split();

    //Create Cancel Tokens for tasks
    let cancel_token = CancellationToken::new();
    let cancel_token_read = cancel_token.clone();
    let cancel_token_write = cancel_token.clone();

    // Spawn Read Message Handler Task
    tokio::spawn(
        async move { read_server_messages(read_stream, read_thread, cancel_token_read).await },
    );

    // Spawn Write Message Handler Task
    tokio::spawn(async move {
        write_server_messages(write_stream, write_thread, cancel_token_write).await
    });

    // Run init
    for component in &target.core.components {
        sync_code(&script, &socket_write, &cancel_token, component).await;
    }

    if is_watch {
        println!("\nWatching file for changes...\n");

        // Spawn the notify loop
        let (mut debouncer, mut file_events) =
            AsyncDebouncer::new_with_channel(Duration::from_secs(1), Some(Duration::from_secs(1)))
                .await?;

        debouncer.watcher().watch(&script, recursion_mode)?;

        let cancel_token_file = cancel_token.clone();
        let path_string = String::from(script.to_string_lossy().to_string());
        let file_changed = socket_write.clone();
        tokio::spawn(async move {
            let file_path = PathBuf::from(path_string);

            while let Some(event) = file_events.recv().await {
                // Get updated file contents
                for component in &target.core.components {
                    sync_code(&file_path, &file_changed, &cancel_token_file, component).await;
                }
                // Currently not using the event
                println!("{event:?}")
            }
        });

        // let mut messages = vec![];
        let mut async_interval = tokio::time::interval(tokio::time::Duration::from_secs(3));

        loop {
            tokio::select! {

            _exit =  tokio::signal::ctrl_c() => {
                // Wait for a termination signal to close the connection gracefully.
                println!("Received Ctrl+C, closing connection...");
                cancel_token.cancel();
                break;
            }

            message_from_server = socket_read.recv() => {
                if let Some(msg) = message_from_server {
                        handle_server_reply(&ServerReply::new(msg)?);
                        // messages.push(&msg);
                    }
            }

            _noop = async_interval.tick() => {
                let payload = JsonRpcCodec::noop()?;
                println!("Payload: {:?}", payload);
                if let Err(e) = socket_write.send(payload).await {
                    eprintln!("Failed to send payload, {e}");
                    break;
                }
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
    }
    Ok(())
}
