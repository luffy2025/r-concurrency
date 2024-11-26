use anyhow::Result;
use std::io::ErrorKind;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::info;

const BUF_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {:?}", raddr);
        tokio::spawn(async move {
            process_redis_conn(stream, raddr).await?;
            Ok::<_, anyhow::Error>(())
        });
    }
}

async fn process_redis_conn(mut stream: TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => {
                info!("Connection<{}> closed", raddr);
                break;
            }
            Ok(n) => {
                info!("Read {} size", n);
                let line = String::from_utf8_lossy(&buf);
                info!("Received: {:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(e) => match e.kind() {
                ErrorKind::WouldBlock => {
                    continue;
                }
                _ => {
                    return Err(e.into());
                }
            },
        }
    }
    Ok(())
}
