use std::error::Error;
use std::time::Duration;
use local_ip_address::local_ip;
use tokio::net::{TcpListener, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /* Variables */
    // Get the local IP address and create the server address
    let lan_ip = local_ip().expect("Failed to get local IP address");
    let server_port = "8080";
    let server_ip = format!("{}",lan_ip);

    // Broadcast port
    let broadcast_port = "5000";

    // Broadcast interval
    let interval = 10;

    tokio::spawn(async move {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", broadcast_port))
            .await
            .expect("Failed to bind UDP socket");

        socket.set_broadcast(true)
            .expect("Failed to set broadcast");

        let broadcast_address = format!("255.255.255.255:{}", broadcast_port);

        loop {
            // Send the server IP address to the broadcast address
            socket.send_to(server_ip.as_bytes(), &broadcast_address)
                .await
                .expect("Failed to send broadcast");
            println!("Broadcast server IP address: {}", server_ip);
            sleep(Duration::from_secs(interval)).await;
        }
    });

    let tcp_listener = TcpListener::bind(format!("0.0.0.0:{}", &server_port))
        .await?;
    println!("Server listening on port {}", &server_port);

    loop {
        let (mut socket, addr) = tcp_listener.accept()
            .await?;
        println!("New TCP connection from: {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Connection closed");
                        break;
                    }
                    Ok(n) => {
                        println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
                        socket.write_all(b"Message received\n")
                            .await
                            .unwrap();
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        return;
                    }
                }
            }
        });
    }
}
