use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 1024];

    // Read data from socket
    let n = socket.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    // Convert the bytes to a string and print it
    let received = String::from_utf8_lossy(&buf[..n]);
    println!("Received from client: {}", received.trim());

    // Send a response back to the client
    socket.write_all(b"Acknowledged").await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Failed to handle client: {}", e);
            }
        });
    }
}
