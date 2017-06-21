use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

/*
 * read reveived data and (with waiting ... and) echo it
 */
fn echo(stream: &mut TcpStream) {
    let mut buf = [0; 1024];

    let _ = stream.read(&mut buf);
    thread::sleep(Duration::from_secs(5)); // wait ...
    let _ = stream.write_all(&buf);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => { echo(&mut stream); }
                Err(_) => { println!("listen error :("); }
            }
        }
    }
}
