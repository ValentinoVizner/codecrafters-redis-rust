use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let ping = b"*1\r\n$4\r\nping\r\n";

    stream.read_exact(&mut buffer).unwrap();

    if buffer.starts_with(ping) {
        stream.write_all(b"+PONG\r\n").unwrap();
        stream.flush().unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_request(stream);
    }
    Ok(())
}