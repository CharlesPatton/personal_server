/*
main file for the personal server



author: Charles Patton
*/
use std::{net::{TcpListener, TcpStream}};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0u8; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    
    stream.write(b"HTTP/1.1 200 OK\r\n\r\nClient accepted: OK!").unwrap();
}

fn main() -> std::io::Result<()> {
    let socket = TcpListener::bind("127.0.0.1:8080")?;

    for stream in socket.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
