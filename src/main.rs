use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn client() -> std::io::Result<()> {
    let mut client = TcpStream::connect("127.0.0.1:7878").unwrap_or_else(|err| {
        println!("Unable to connect to the server{}", err);
        panic!("connection failed ");
    });
    let mut buff = [0; 534];

    let mut inp = String::new();
    loop {
        println!("Enter the message you wanna send: ");
        io::stdin().read_line(&mut inp).unwrap();
        let mes = inp.trim().as_bytes();
        let res = client.write(mes);
        println!("CXIn {:?}", client);
        println!("{:?}", res);
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    loop {
        let mut input: String = String::new();
        println!("Enter the choice to select");
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        if input == "yes" {
            println!("The client side selected");
            match client() {
                Ok(_) => println!("Conncections established"),
                Err(_e) => println!("Error conections to server"),
            };
        } else if input == "ded" {
            println!("The server side selected");
            match serv() {
                Ok(_) => println!("started"),
                Err(_e) => eprintln!("Error connecting"),
            };
        } else {
            continue;
        }
    }
}

fn serv() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let mut clients: HashMap<String, TcpStream> = HashMap::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_users(&stream, &mut clients);
                println!("Connection established: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    let _ = handle_server(&stream);
                })
            }
            Err(e) => {
                println!("Connection failed{}", e);
                return Ok(());
            }
        };
    }
    Ok(())
}

fn handle_server(mut stream: &TcpStream) -> std::io::Result<()> {
    loop {
        let mut buff = [0; 534];
        match stream.read(&mut buff) {
            Ok(0) => {
                println!("Client removed");
                break;
            }
            Ok(bytes_read) => {
                let received_data = String::from_utf8_lossy(&buff[..bytes_read]);

                // Print the received data
                println!("Received data: {}", received_data);
            }
            Err(e) => println!("eroor{}", e),
        };
    }
    Ok(())
}

fn handle_users(stream: &TcpStream, clients: &mut HashMap<String, TcpStream>) {
    let addr = stream.peer_addr().unwrap().to_string();
    clients.insert(addr.clone(), stream.try_clone().unwrap());
    println!("The active clients: {:?}", clients);
}
