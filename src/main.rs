use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::time::{self, timeout, Duration};

static TIMEOUT_DURATION: Duration = Duration::from_secs(5);

// TCP Server
async fn tcp_server(port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(("127.0.0.1", port)).await?;
    println!("TCP Server listening on port {}", port);

    let timeout_future = time::sleep(TIMEOUT_DURATION);
    tokio::pin!(timeout_future);

    loop {
        tokio::select! {
            _ = &mut timeout_future => {
                println!("TCP server timed out.");
            return Err("TCP server timed out".into());
            }
            Ok((mut stream, src)) = listener.accept() => {
                println!("New connection from {}", src);
                let mut buf = [0; 1024];
                match stream.read(&mut buf).await {
                    Ok(n) => {
                        if n == 0 { continue; } // Connection was closed
                        let data = String::from_utf8_lossy(&buf[..n]);
                        println!("Received from TCP client {}: {}", src, data);
                        if data.starts_with("Hello from client") {
                            println!("Exiting TCP server");
                            return Ok(());
                        }
                        if let Err(e) = stream.write_all(&buf[..n]).await {
                            println!("Failed to write to client {}: {}", src, e);
                        }
                    }
                    Err(e) => println!("Failed to read from client {}: {}", e, src),
                }
            }
        }
    }
}

// UDP Server
async fn udp_server(port: u16) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind(("127.0.0.1", port)).await?;
    println!("UDP Server listening on port {}", port);
    let mut buf = [0; 1024];

    loop {
        match timeout(TIMEOUT_DURATION, socket.recv_from(&mut buf)).await {
            Ok(Ok((n, src))) => {
                let data = String::from_utf8_lossy(&buf[..n]);
                println!("Received from UDP client {}: {}", src, data);
                if data.starts_with("Hello from client") {
                    println!("Exiting UDP server");
                    return Ok(());
                }
            }
            Ok(Err(e)) => {
                println!("Failed to receive UDP datagram: {}", e);
                return Err("UDP server Error".into());
            }
            Err(_) => {
                println!("UDP server timed out");
                return Err("UDP server timed out".into());
            }
        }
    }
}

async fn test_tcp_connectivity(port: u16) {
    match timeout(TIMEOUT_DURATION, TcpStream::connect(("127.0.0.1", port))).await {
        Ok(Ok(mut stream)) => {
            println!("TCP port {} is open", port);
            let msg = b"Hello from client";
            if let Err(e) = stream.write_all(msg).await {
                eprintln!("Failed to write to TCP port {}: {}", port, e);
            }
        }
        Ok(Err(_)) => {
            println!("TCP port {} is closed", port);
        }
        Err(_) => {
            println!("Connection to TCP port {} timed out", port);
        }
    }
}

async fn test_udp_connectivity(port: u16) {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let msg = b"Hello from client";
    match socket.send_to(msg, ("127.0.0.1", port)).await {
        Ok(_) => println!("UDP datagram sent to port {}", port),
        Err(e) => println!("Failed to send UDP datagram to port {}: {}", port, e),
    }
}

// Client to test connectivity
async fn test_connectivity(port: u16) {
    tokio::join!(test_tcp_connectivity(port), test_udp_connectivity(port));
}

#[tokio::main]
async fn main() {
    let port = 8081; // Replace with the port you want to use

    let tcp_server = tokio::spawn(async move { tcp_server(port).await.unwrap() });
    let udp_server = tokio::spawn(async move { udp_server(port).await.unwrap() });

    tokio::time::sleep(Duration::from_secs(1)).await;

    // Test connectivity
    test_connectivity(port).await;

    // Wait for servers to finish
    udp_server.await.unwrap();
    tcp_server.await.unwrap();
}
