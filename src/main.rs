use std::io::{self, prelude::*};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio::io::{AsyncReadExt};

const SERVER_ADDR: &str = "localhost:7777"; // Address of the echo server

#[tokio::main]
async fn main() {

    println!("Connecting to server at {SERVER_ADDR}...");
    
    // Connect to the server
    let mut stream = connect_to_server().await;   

    println!("Connected to echo server at {}", 
        stream.peer_addr().unwrap()
    );

    loop{
        println!("Enter message to send to server (or type 'exit' to quit): ");
        let mut message  =  String::new();
        
        io::stdin().read_line(&mut message).expect("Failed to read input from user");

        let message = message.trim().to_string();
        
        if message.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }
        
        stream.write_all(message.as_bytes())
        .await
        .expect("Failed to send message to server");

        println!("Sent message to server: {message}"); 

        let mut buffer = [0;1024];
        let bytes_read = match stream.read(&mut buffer).await {
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
async fn connect_to_server() -> TcpStream {

   if let Ok( stream) = TcpStream::connect(SERVER_ADDR).await {
        stream
    } else {
        panic!("Failed to connect to server at {SERVER_ADDR}");
    }
}
