use std::io;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio::io::{AsyncReadExt};

const SERVER_ADDR: &str = "127.0.0.1:7778"; // Address of the EchoMambo-client server

#[tokio::main]
async fn main() {
    let arg = std::env::args().nth(1);
    let server_addr = arg.as_deref().unwrap_or(SERVER_ADDR);    

    // Connect to the server
    let mut stream = connect_to_server(&server_addr).await;   

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
async fn connect_to_server(server_addr: &str) -> TcpStream {

   if let Ok( stream) = TcpStream::connect(server_addr).await {
        stream
    } else {
        panic!("Failed to connect to server at {server_addr}");
    }
}
