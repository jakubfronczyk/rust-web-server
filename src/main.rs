use tokio::{io::AsyncWriteExt, net::TcpListener};

use std::io;

async fn handle(mut socket: tokio::net::TcpStream) -> io::Result<()> {
    socket.write_all(b"Hello World form urst").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        handle(socket).await;
    }
}
