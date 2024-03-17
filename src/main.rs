use std::{collections::HashMap, io};

use tokio::net::TcpListener;

mod http;
use http::*;

async fn handle(socket: tokio::net::TcpStream) -> Result<(), Error> {
    let mut connection = Connection::new(socket).await?;
    println!(
        "method:{:?}\nuri:{:?}\nversion:{:?}\nheaders:{:?}",
        connection.request.method,
        connection.request.uri,
        connection.request.version,
        connection.request.headers
    );
    let mut res_headers: HashMap<String, String> = HashMap::new();
    // res_headers.insert("Content-Length".to_string(), "0".to_string());
    connection
        .respond(Response {
            status: StatusCode::ok(),
            headers: res_headers,
            body: &String::from("What's up bro"),
        })
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = match TcpListener::bind("127.0.0.1:8080").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            return Err(e);
        }
    };

    println!("Server listening on port 8080");

    loop {
        let (socket, _) = listener.accept().await?;
        handle(socket).await;
    }
}
