use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    println!("TETE!");

    let listener = TcpListener::bind("0.0.0.0:852").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Received connection from {:?}", stream.peer_addr().unwrap());
        thread::spawn(|| {
            handle_connection(stream);
        });
    }


}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}