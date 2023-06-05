use std::{env, net::SocketAddr, str::FromStr, time::Duration};
use tokio_uring::net::{TcpListener, TcpStream};
use tokio_uring::start;
async fn server() {
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 8800))).unwrap();
    println!("Listener started, 10s sleep");
    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("Starting listener loop");
    loop {
        tokio::select! {
            conn = listener.accept() => {
                if let Ok((_stream, _addr)) = conn {
                    println!("Connection accepted")
                } else {
                    println!("Error accepting connection");
                }
            },
        }
    }
}

async fn client() {
    loop {
        match TcpStream::connect(SocketAddr::from_str("127.0.0.1:8800").unwrap()).await {
            Ok(_stream) => {
                println!("Connection accepted");
                loop {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            Err(e) => {
                eprintln!("Failed to reconnect; err = {:?}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 || (args[1] != "client" && args[1] != "server") {
        eprintln!("Usage: {} client|server", args[0]);
        return;
    }
    if args[1] == "client" {
        start(client());
    } else {
        start(server());
    }
}
