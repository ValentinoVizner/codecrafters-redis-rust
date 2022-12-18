use std::{net::{TcpListener, TcpStream}, thread};
use std::io::prelude::*;

fn handle_redis_requests(mut stream: TcpStream) {
    let remote_addr = stream.peer_addr().unwrap();
    println!("accepted new connection from {}", remote_addr);
    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_size) => match stream.write(b"+PONG\r\n") {
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

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_redis_requests(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }
    Ok(())
}