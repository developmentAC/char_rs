// Import necessary modules for command-line arguments, networking, threading, and input/output
use colored::Colorize;
use std::env;
use std::io::Read;
use std::io::{self, Write};
use std::net::{IpAddr, TcpListener, TcpStream, UdpSocket};
use std::thread;

mod toml_extract; // Extract and print the version information according to the toml file

// Print colored text to the console
fn colour_print(text: &str, colour: &str) {
    match colour {
        "flush_green" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            io::stdout().flush().unwrap();
            print!(" {}", text.bright_green().bold());
            io::stdout().flush().unwrap();
        }
        "green" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_green().bold());
        }
        "green_noLineFeed" => {
            print!("\x1b[2K\r");
            print!("{}", text.bright_green().bold());
        }
        "red" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_red().bold());
        }
        "cyan" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_cyan().bold());
        }
        "purple" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_purple().bold());
        }
        "purple_noLineFeed" => {
            print!("\x1b[2K\r");
            print!("{}", text.bright_purple().bold());
        }
        "blue" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_blue().bold());
        }
        "yellow" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_yellow().bold());
        }
        "yellow_noLineFeed" => {
            print!("\x1b[2K\r");
            print!("{}", text.bright_yellow().bold());
        }
        _ => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_yellow().bold());
        }
    }
}

// Function to determine the local IP address of the machine
// This is useful for users to identify their IP address when setting up the server
fn get_local_ip() -> Option<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?; // Bind to an available port
    socket.connect("8.8.8.8:80").ok()?; // Connect to a public DNS server to determine the local IP address
    socket.local_addr().ok().map(|addr| addr.ip()) // Extract and return the local IP address
}

// Function to handle communication with a connected client
// This function runs in a separate thread for each client
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // Buffer to store incoming data
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed by the client
            Ok(_) => {
                let message = String::from_utf8_lossy(&buffer); // Convert bytes to a string
                // println!("  Received: {}", message); // Print the received message
                let msg = format!("\t Received: {message}" );
                colour_print(&msg, "yellow");
            


            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e); // Handle read errors
                break;
            }
        }
    }
}

// Function to start the server
// The server listens for incoming connections and spawns a new thread for each client
fn start_server(address: &str) {
    let listener = TcpListener::bind(address).expect("Could not bind :-("); // Bind to the specified address

    // println!("\t Server listening on {}", address);
    let msg = format!("\t Server listening on {address}" );
    colour_print(&msg, "green"); // print in green

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("\t New connection: {}", stream.peer_addr().unwrap()); // Log the new connection
                thread::spawn(|| handle_client(stream)); // Handle the client in a new thread
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e); // Handle connection errors
            }
        }
    }
}

// Function to start the client
// The client connects to the server and allows the user to send messages
fn start_client(address: &str) {
    let mut stream = TcpStream::connect(address).expect("  Could not connect to server"); // Connect to the server
    println!("\t Connected to server at {}", address);

    let stdin = io::stdin(); // Standard input for user input
    let mut input = String::new(); // Buffer to store user input

    loop {
        print!("\t Enter a message: "); // Prompt the user for input
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
        input.clear(); // Clear the input buffer
        stdin.read_line(&mut input).unwrap(); // Read user input

        if input.trim() == "exit" {
            // Exit the loop if the user types "exit"
            // stream.write_all(input.as_bytes()).expect("Failed to send message :-("); // Send the message to the server
            stream
                .write_all("Your correspondent has exited the connection ...".as_bytes())
                .expect("Failed to send message :-("); // Send the message to the server
            break;
        }

        stream
            .write_all(input.as_bytes())
            .expect("Failed to send message :-("); // Send the message to the server
    }
}

// Main function to parse command-line arguments and start the appropriate mode
fn main() {
    // Display version information from the toml file
    toml_extract::main();

    let args: Vec<String> = env::args().collect(); // Collect command-line arguments

    if args.len() < 3 {
        eprintln!("\t Modes: server, client");
        eprintln!("\t Usage: {} <mode> <address>", args[0]); // Print usage instructions
        if let Some(ip) = get_local_ip() {
            println!("\t Your local IP address is: {}", ip); // Display the local IP address
            colour_print("\n\t Sample commands to run the server and client:","cyan");
            let msg = format!("\t  {} server {ip}:8080",args[0]);
            colour_print(&msg, "green"); // Print the server command in green

            let msg = format!("\t  {} client {ip}:8080",args[0]);
            colour_print(&msg, "green"); // Print the server command in green

            // println!("\t Server command: {} server {ip}:8080",args[0]); // Provide an example usage
            // println!("\t Client command: {} client {ip}:8080",args[0]); // Provide an example usage
        } else {
            println!("\t Could not determine your local IP address. :-("); // Handle failure to determine IP
        }
        return;
    }

    let mode = &args[1]; // First argument specifies the mode (server or client)
    let my_address = &args[2]; // Second argument specifies the address (IP:port)

    match mode.as_str() {
        "server" => start_server(my_address), // Start the server if mode is "server"
        "client" => start_client(my_address), // Start the client if mode is "client"
        _ => eprintln!("  Invalid mode. Use 'server' or 'client'."), // Handle invalid mode
    }
}
