use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn client() -> std::io::Result<()> {
    let mut client = TcpStream::connect("127.0.0.1:7878").unwrap_or_else(|err| {
        println!("Unable to connect to the server{}", err);
        panic!("connection failed ");
    });
    let mut client_clone = client.try_clone()?;

    //spawn a thread to handle incoming message from the server
    thread::spawn(move || {
        let mut buff = [0; 1024];
        loop {
            match client_clone.read(&mut buff) {
                Ok(0) => {
                    println!("Disconnected");
                    break;
                }
                Ok(byes) => {
                    let message = String::from_utf8_lossy(&buff[..byes]);
                    println!("Server: {}", message);
                }
                Err(e) => {
                    println!("error occured{e}");
                    break;
                }
            }
        }
    });

    let mut inp = String::new();
    loop {
        println!("Enter the message you wanna send: ");
        inp.clear();
        io::stdin().read_line(&mut inp).unwrap();
        let mes = inp.trim().as_bytes();
        let res = client.write(mes);
        println!("CXIn {:?}", client);
        println!("{:?}", res);
    }
}

fn main() -> std::io::Result<()> {
    loop {
        let mut input: String = String::new();
        println!("Enter the choice to select");
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        if input == "connect" {
            println!("The client side selected");
            match client() {
                Ok(_) => println!("Conncections established"),
                Err(_e) => println!("Error conections to server"),
            };
        } else if input == "start" {
            println!("The server side selected");
            match serv() {
                Ok(_) => println!("started"),
                Err(_e) => eprintln!("Error starting the server"),
            };
        } else {
            continue;
        }
    }
}

fn serv() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let clients = Arc::new(Mutex::new(HashMap::new()));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr()?.to_string();
                let clients_clone = Arc::clone(&clients);
                {
                    // Add the new client to the hashmap
                    let mut clients_lock = clients_clone.lock().unwrap();
                    clients_lock.insert(addr.clone(), stream.try_clone()?);
                }
                println!("Connection established: {}", addr);

                thread::spawn(move || {
                    let _ = handle_server(stream, clients_clone, addr)
                        .unwrap_or_else(|e| println!("error handling client: {}", e));
                });
            }
            Err(e) => {
                println!("Connection failed{}", e);
                return Ok(());
            }
        };
    }
    Ok(())
}

fn handle_server(
    mut stream: TcpStream,
    clients: Arc<Mutex<HashMap<String, TcpStream>>>,
    addr: String,
) -> std::io::Result<()> {
    loop {
        let mut buff = [0; 2048];
        match stream.read(&mut buff) {
            Ok(0) => {
                // Remove client from the hashmap
                let mut clients_lock = clients.lock().unwrap();
                clients_lock.remove(&addr);
                break;
            }
            Ok(bytes_read) => {
                let message = String::from_utf8_lossy(&buff[..bytes_read]);
                println!("Message from {}: {}", addr, message);
                let client_lock = clients.lock().unwrap();
                for (client_addr, mut client_stream) in client_lock.iter() {
                    if client_addr != &addr {
                        if let Err(_e) = client_stream.write_all(message.as_bytes()) {
                            println!("Error sending the message");
                        }
                    }
                }
            }
            Err(e) => {
                println!("eroor{}", e);
                // Remove client from the hashmap
                let mut clients_lock = clients.lock().unwrap();
                clients_lock.remove(&addr);
                break;
            }
        };
    }
    Ok(())
}
