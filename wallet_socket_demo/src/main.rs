use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use wallet_socket_demo::generate_random_secret_key;

fn handle_client(mut stream: TcpStream) {
    // TODO: command pattern
    // DOTO: docker file
    let mut buffer = [0; 3];
    stream.read_exact(&mut buffer).unwrap();

    let command = String::from_utf8_lossy(&buffer[..]);
    dbg!(&command.trim());
    if command.trim() == "GEN" {
        let secret_key = generate_random_secret_key();
        stream.write_all(secret_key.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_client(stream);
    }
}
