use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_requests(mut stream: TcpStream) {
    let remote_addr = stream.peer_addr().unwrap();
    println!("accepted new connection from {}", remote_addr);
    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer).await {
            Ok(_size) => match stream.write("+PONG\r\n".as_bytes()).await {
                Ok(_) => continue,
                Err(e) => {
                    println!("error writing to the stream: {}", e);
                    break;
                }
            }
            Err(e) => {
                println!("error reading from the stream: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        // accept connections and process them serially
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, _)) => {
                tokio::spawn( async move{
                    handle_requests(stream).await;
                });
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
}