use std::io::{self, prelude::*};
use std::net::TcpStream;

const SERVER_ADDR: &str = "localhost:7777"; // Address of the echo server

fn main() {

    println!("Connecting to server at {SERVER_ADDR}...");
    
    // Connect to the server
    let mut stream = connect_to_server();   

    println!("Connected to echo server at {}", 
        stream.peer_addr().unwrap()
    );

    loop{
        print!("Enter message to send to server (or type 'exit' to quit): ");
        let mut message  =  String::new();
        io::stdin().read_line(&mut message).expect("Failed to read input from user");
        let _ = stream.write_all(message.as_bytes()).expect("Failed to send message to server");
        let _ = stream.flush().expect("Failed to flush stream");

        println!("Sent message to server: {message}"); 

        let mut buffer = [0;1024];
        let bytes_read = match stream.read(&mut buffer) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Failed to read response from server: {}", e);
                return;             
            }
        };
        
        if bytes_read == 0 {
            eprintln!("Server closed the connection unexpectedly.");
            return;
        }
        else{
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Echo: {response}");
        }
        
    }
}

//=== Function to connect to the server ===//
fn connect_to_server() -> TcpStream {

   if let Ok( stream) = TcpStream::connect(SERVER_ADDR) {
        stream
    } else {
        panic!("Failed to connect to server at {SERVER_ADDR}");
    }
}
