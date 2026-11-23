// #[tokio::main]
// async fn main() {
//     let (tx, mut rx) = tokio::sync::mpsc::channel(10);
//
//     tokio::spawn(async move {
//         for i in 0..10 {
//             println!("Send = {}", i);
//             tx.send(i).await.unwrap();
//         }
//     });
//
//     while let Some(value) = rx.recv().await {
//         println!("Got = {}", value);
//     }
// }

use std::env;
use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = {
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        format!("0.0.0.0:{port}")
    };
    let srv = TcpListener::bind(&addr).await?;
    let (tx, _) = broadcast::channel::<(SocketAddr, Vec<u8>)>(128);
    println!("Broadcast server listening on {addr}");

    loop {
        let (sock, peer_addr) = srv.accept().await?;
        println!("Client connected: {peer_addr}");

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let (mut reader, mut writer) = sock.into_split();

        let read_task = tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                let n = match reader.read(&mut buf).await {
                    Ok(0) => {
                        println!("Client disconnected: {peer_addr}");
                        break;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("[{peer_addr}] read error: {e}");
                        return;
                    }
                };

                let _ = tx.send((peer_addr, buf[..n].to_vec()));
            }
        });

        let write_task = tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok((sender_addr, msg)) => {
                        let prefix = format!("[{sender_addr}] ");
                        if let Err(e) = writer.write_all(prefix.as_bytes()).await {
                            eprintln!("[{peer_addr}] Write error: {e}");
                            break;
                        }
                        if let Err(e) = writer.write_all(&msg).await {
                            eprintln!("[{peer_addr}] Write error: {e}");
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        eprintln!("[{peer_addr}] Lagged, skipped {skipped} messages");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            let _ = writer.shutdown().await;
        });

        tokio::spawn(async move {
            let _ = read_task.await;
            write_task.abort();
        });
    }
}
