# Rust TCP Chat Server and Client

This project was created for learning purposes to demonstrate how to build a simple chat server and client in Rust using TCP communication.

---

## Features

- **TCP Communication**: The server and client communicate using `TcpListener` and `TcpStream`.
- **Multi-client Support**: The server handles multiple clients, allowing them to send and receive messages.
- **Real-time Messaging**: Clients can send messages, and the server broadcasts them to all other connected clients.
- **Concurrency**: The server uses threads to handle each client's communication separately.
- **Interactive Client**: The client asks for user input to send messages and displays received messages from the server.

---

## Purpose

This project was created as part of a learning exercise to better understand:
- TCP networking in Rust
- Multi-threading
- Real-time communication

