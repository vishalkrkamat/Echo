use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
fn handle(stream: TcpStream) -> std::io::Result<()> {
    Ok(())
}
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let _ = handle(stream);
        println!("Connection established");
    }
    Ok(())
}
