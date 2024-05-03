

mod build;
mod r#user;


use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use prost::Message;
use user::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];  // Buffer to hold incoming data

            // Read data from the socket
            match socket.read(&mut buf).await {
                Ok(size) if size > 0 => {
                    // Attempt to deserialize the buffer into a User protobuf message
                    match User::decode(&buf[..size]) {
                        Ok(user) => {
                            println!("Received user: {:?}", user);
                            // You can now handle the user object, e.g., save to a database, etc.
                        },
                        Err(e) => println!("Failed to decode user: {}", e),
                    }
                },
                Ok(_) => println!("No data received"),
                Err(e) => println!("Failed to read from socket: {}", e),
            }
        });
    }
}
