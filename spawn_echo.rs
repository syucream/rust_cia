use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

/*
 * read reveived data and echo it
 */
fn echo(stream: &mut TcpStream) -> Result<(), io::Error> {
    let mut buf = [0; 1024];

    stream.read(&mut buf)?;
    stream.write_all(&buf)?;

    return Ok(());
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    loop {
        match listener.accept() {
            Ok((mut stream, _)) => {
                let _ = thread::spawn(move || {
                    loop {
                        match echo(&mut stream) {
                            Ok(_) => { continue; }
                            Err(_) => { break; }
                        }
                    }
                });
            }
            Err(_) => { println!("listen error :("); }
        }
    }
}
