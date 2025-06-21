use std::path::PathBuf;
use std::time::Duration;
use tokio::net::TcpStream;

use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use anyhow::Context;

use async_watcher::notify::RecursiveMode;
use async_watcher::AsyncDebouncer;

use super::sync_config::{QsysCore, Target};
use crate::cli::subcommands::sync::qrc_api::{handle_server_reply, QrcMessage, ServerReply};
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
    sock_write_channel: &mpsc::Sender<QrcMessage>,
    cancel_token_file: &CancellationToken,
    component: &Component,
    verbose: bool,
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

    let sync_code = std::fs::read_to_string(&lua_file).unwrap();

    if let Err(e) = sock_write_channel
        .send(
            QrcMessage::update_code(component.name.as_ref(), sync_code)
                .await
                .unwrap(),
        )
        .await
    {
        eprintln!("Failed to send payload, {e}");
        cancel_token_file.cancel();
    } else {
        verbose_print(format!("Syncing {:?}", &lua_file), verbose);
    }
}

fn verbose_print<T>(msg: T, verbose: bool) -> ()
where
    T: std::fmt::Debug,
{
    if verbose {
        println!("\n\x1b[93m{:?}\x1b[0m\n", msg);
    }
}

#[tokio::main]
pub async fn sync(
    script: &PathBuf,
    config: &PathBuf,
    is_watch: bool,
    verbose: bool,
) -> anyhow::Result<String> {
    console_subscriber::init();

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
    let (read_thread, mut read_socket) = mpsc::channel(8192);
    let (write_socket, write_thread) = mpsc::channel(8192);

    // Connect to the stream
    let connection = qsys_core.connect().await?;
    let (read_stream, write_stream) = connection.into_split();

    //Create Cancel Tokens for tasks
    let cancel_token = CancellationToken::new();
    // let cancel_token_read = cancel_token.clone();
    let cancel_token_write = cancel_token.clone();

    // Spawn Read Message Handler Task
    let read_join =
        tokio::spawn(async move { read_server_messages(read_stream, read_thread).await });

    // Spawn Write Message Handler Task
    let write_join = tokio::spawn(async move {
        write_server_messages(write_stream, write_thread, cancel_token_write).await
    });

    // Run init
    for component in &target.core.components {
        sync_code(&script, &write_socket, &cancel_token, component, verbose).await;
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
        let file_changed = write_socket.clone();
        let _file_watch_join = tokio::spawn(async move {
            let file_path = PathBuf::from(path_string);

            while let Some(event) = file_events.recv().await {
                // Get updated file contents
                for component in &target.core.components {
                    sync_code(
                        &file_path,
                        &file_changed,
                        &cancel_token_file,
                        component,
                        verbose,
                    )
                    .await;
                }
                // Currently not using the event
                println!("\nSyncing File: {:?}\n", event.unwrap()[0].path)
            }
        });

        let read_handle = tokio::spawn(async move {
            loop {
                if let Some(msg) = read_socket.recv().await {
                    verbose_print(format!("Messge Received: {:?}", &msg), verbose);
                    handle_server_reply(ServerReply::new(msg).unwrap());
                    // messages.push(&msg);
                }
            }
        });

        let noop = tokio::spawn(async move {
            let mut async_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

            loop {
                async_interval.tick().await;
                let payload = QrcMessage::noop().unwrap();
                verbose_print(&payload, verbose);
                if let Err(e) = write_socket.send(payload).await {
                    eprintln!("Failed to send payload, {e}");
                    break;
                }
            }
        });
        // let mut messages = vec![];
        let _ctl_join = tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            println!("\n\nShutting Down...");
            write_join.abort();
            read_join.abort();
            _file_watch_join.abort();
            read_handle.abort();
            noop.abort();
            cancel_token.cancel();
            return;
        });

        _ctl_join.await?

        // _component_controls  {
        //     // let payload = JsonRpcCodec::get_component_controls(&target.component.name).unwrap();
        //     // println!("Payload: {:?}", payload);
        //     // if let Err(_e) = write_tx.send(payload).await {
        //     //     eprintln!("Failed to send payload");
        //     //     break;
        //     // }
        // }
    }
    Ok("Ok".into())
}
