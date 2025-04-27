# Chat-RS: A simple IP-to-IP chat application built in Rust to help teach network programming

![graphics/logo.png](graphics/logo.png)

Date: 27 April 2025

Oliver Bonham-Carter

Email: obonhamcarter at allegheny.edu

This README.md file provides a comprehensive explanation of the project, including how to use it, examples, and instructions for adding more characters. It also includes sections for contributing and licensing.

_Chat-RS_ is a cool IP-to-IP chat application written in Rust that uses socket programming to allow users to send and receive text via IP addresses. The program is designed to be simple and easy to use, making it a interesting starting point for those who would like to begin learning about network programming in Rust (using the `TcpListener`, `TcpStream`, `IpAddr` and `UdpSocket` crates, in addition to others).

The application consists of a server and a client. The server listens for incoming connections, while the client connects to the server to send messages. That makes for a cool conversation starter, right?!

## Table of Contents

- [Chat-RS: A simple IP-to-IP chat application built in Rust to help teach network programming](#chat-rs-a-simple-ip-to-ip-chat-application-built-in-rust-to-help-teach-network-programming)
  - [Table of Contents](#table-of-contents)
- [IP-to-IP Chat Application](#ip-to-ip-chat-application)
  - [Features](#features)
- [Modes: Server and Client](#modes-server-and-client)
  - [Executing the Code](#executing-the-code)
  - [Example Usage](#example-usage)
    - [**Machine1** (`server`) and **Machine2** (`client`) communcation](#machine1-server-and-machine2-client-communcation)
  - [Notes](#notes)
  - [License](#license)
  - [Contributing](#contributing)
  - [A Work In Progress](#a-work-in-progress)

# IP-to-IP Chat Application

This is a simple IP-to-IP chat application built in Rust. It allows users to send messages directly to each other over a network using TCP sockets. The application consists of a server and a client. The server listens for incoming connections, while the client connects to the server to send messages.

## Features

- Direct IP-to-IP communication
- Simple command-line interface
- Message sending and receiving
- Exit command to close the client

The application was written in Rust has two modes: server and client. Use the following commands to run it:

# Modes: Server and Client

In networking, a `server` is a program that provides the services that are requested by a `client` that initiates the requests. Here, the `server` listens for incoming connections from `clients`. The `server` is also responsible for accepting connections and handling incoming messages.

The `server` uses the `TcpListener` crate to listen for incoming connections on a specified IP address and port. When a `client` connects, the `server` accepts the connection and starts a new thread to handle communication with that `client`. The `server` uses the `TcpStream` to send and receive its messages over the network.

While the chat-space may appear jumbled in this project due multiple `clients` talking at the same time, the `server` can actually handle several `clients` at the same time by spawning new threads for each connection. Each of the thread listens individually for incoming messages from the `client` and then prints them to the console.

## Executing the Code

**First step: open the server**
Start the server by specifying the mode as `server` and specifying the IP address with a port to use to listen for incoming connections.

``` bash
cargo run -- server 127.0.0.1:8080
```

Note: Replace `127.0.0.1:8080` with the desired IP and port.

The server will wait for incoming connections and display received messages. (Seems a little sad to me that the server just sits around waiting for whatever it finds, but I guess it is a server after all!)

**Second step: open the client**
Start the client by specifying the mode as client and the server's IP address with the port:

``` bash
cargo run -- client 127.0.0.1:8080
```

Note: Replace `127.0.0.1:8080` with the server's IP and port.

Enter messages in the terminal to send them to the `server`. Type exit to close the client.

## Example Usage

To chat across two separate machines on the same wifi network, follow the below instructions. 

### **Machine1** (`server`) and **Machine2** (`client`) communcation

Imagine that there are two machines involved,
  + **Machine1** (`server`, IP: `192.168.40.69`)
  + **Machine2** (`client`, IP: `192.168.40.121`)
  
Notice that the IP addresses are different! The `server` will be running on **Machine1**, and the `client` will be running on **Machine2**.

In order to have the machines communicate, the `client` onf**Machine1** must be using the IP address of the `server` of **Machine2** who is listening.

1. On **Machine1** (`Server`):
   - Open a terminal and run the server:

     ```bash
     cargo run -- server 192.168.40.69:8080
     ```

This creates a server that listens on IP address `192.168.40.69` and port `8080`. The server will wait for incoming connections and display received messages.

2. **On Machine2 (Client)**:
   - Open a terminal and run the client using the IP address of the server:
     ```bash
     cargo run client 192.168.40.69:8080
     ```

This connects the `client` to the `server` at IP address. You can now start sending messages from the `client` (Machine2) to the `server` (Machine1). when you are finished on the `client`, type `exit` to close the connection.

In order for **Machine2** to send messages back to **Machine1**, the `client` must be using the IP address of the `server` of **Machine1** who is listening.

1. On **Machine2** (`Server`):
   - Open a terminal and run the server:

     ```bash
     cargo run -- server 192.168.40.121:8080
     ```

This creates a server that listens on IP address `192.168.40.69` and port `8080`. The server will wait for incoming connections and display received messages.

2. **On Machine1 (Client)**:
   - Open a terminal and run the client using the IP address of the server:
     ```bash
     cargo run client 192.168.40.121:8080
     ```

## Notes

- Ensure the server and client are on the same network.

- Use a valid IP address and port that are not blocked by firewalls.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing

Contributions are welcome! If you have ideas for improvements or want to add more features, feel free to open an issue or submit a pull request.

## A Work In Progress

Check back often to see the evolution of the project! **Chat-RS** is a work-in-progress. Updates will come periodically.

If you would like to contribute to this project, please do! For instance, if you see some low-hanging fruit or tasks that could add value to the project, I would love to have your insight.

Otherwise, please create an issue for bugs or errors. Since I am a teaching faculty member at Allegheny College, I may not have all the time necessary to quickly fix bugs. I welcome the Open Source Community to further the development of this project. Much thanks in advance.

If you appreciate this project, please consider clicking the project's Star button. :-)
