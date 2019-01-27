
use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Read;

fn handle_client(mut _stream: TcpStream) {
//     let mut buffer = Vec::new();
    let mut v = String::new();
    
    _stream.read_to_string(&mut v);
    
    println!("{:?}", v);
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("25.64.163.196:34254")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}