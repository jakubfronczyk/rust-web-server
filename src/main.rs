use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use tokio::io::AsyncReadExt;
use tokio::{io::AsyncWriteExt, net::TcpListener};

type RequestParseResult = Result<Request, Error>;

enum Error {
    ParsingError,
    Utf8Error(std::string::FromUtf8Error),
    IOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(internal_err: std::io::Error) -> Self {
        Error::IOError(internal_err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(internal_err: std::string::FromUtf8Error) -> Self {
        Error::Utf8Error(internal_err)
    }
}
enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    OPTION,
    DELETE,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        if s == "GET" {
            Method::GET
        } else if s == "POST" {
            Method::POST
        } else {
            Method::GET
        }
    }
}
enum Version {
    HTTP1_1,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        if s == "HTTP/1.1" {
            Version::HTTP1_1
        } else {
            Version::HTTP1_1
        }
    }
}

struct Request {
    method: Method,
    uri: String,
    version: Version,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    path_params: HashMap<String, String>,
    reader: tokio::net::TcpStream,
}

impl Request {
    pub async fn new(mut reader: tokio::net::TcpStream) -> RequestParseResult {
        // Initialize variables to store request data
        let mut first_line: String = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut buffer: Vec<u8> = Vec::new();

        // Read data from the stream until the end of the request headers
        loop {
            let b = reader.read_u8().await?; // Read a byte from the stream
            buffer.push(b); // Store the byte in the buffer

            // Check if we have reached the end of a header line
            if b as char == '\n' {
                // If this is the first line, parse it as the request line
                if first_line.is_empty() {
                    first_line = String::from_utf8(buffer.clone())?; // Parse the first line
                    buffer.clear(); // Clear the buffer for the next line
                } else {
                    // If it's not the first line, parse it as a header and add it to the headers map
                    if buffer.len() == 2 && buffer[0] as char == '\r' {
                        break; // If we encounter an empty line, we've reached the end of the headers
                    }
                    let header_line = String::from_utf8(buffer.clone())?; // Parse the header line
                    let mut iter = header_line.split(":");
                    let key = match iter.next() {
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };
                    let value = match iter.next() {
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };
                    headers.insert(key.to_string(), value.to_string()); // Insert the header into the map
                }
            }
        }

        // Parse the request line into method, URI, and HTTP version
        let mut first_line_iter = first_line.split(" "); // Split the first line by space
        let uri_iter_next_unwrap = first_line_iter.next().unwrap().to_string(); // Get the URI part of the first line
        let mut uri_iter = uri_iter_next_unwrap.split("?"); // Split the URI part by question mark to separate URI and query parameters
        let uri = match uri_iter.next() {
            // Get the URI from the split
            Some(u) => u, // If URI exists, assign it to 'uri'
            None => return Err(Error::ParsingError), // If URI doesn't exist, return parsing error
        };
        let mut query_params: HashMap<String, String> = HashMap::new(); // Create a HashMap to store query parameters
        match uri_iter.next() {
            // Check if there are query parameters
            Some(q) => {
                // If query parameters exist
                for kv in q.split("&") {
                    // Split query parameters by '&'
                    let mut iter = kv.split("="); // Split each key-value pair by '='
                    let key = match iter.next() {
                        // Get the key part of the key-value pair
                        Some(k) => k, // If key exists, assign it to 'key'
                        None => return Err(Error::ParsingError), // If key doesn't exist, return parsing error
                    };
                    let value = match iter.next() {
                        // Get the value part of the key-value pair
                        Some(k) => k, // If value exists, assign it to 'value'
                        None => return Err(Error::ParsingError), // If value doesn't exist, return parsing error
                    };
                    query_params.insert(key.to_string(), value.to_string()); // Insert key-value pair into the HashMap
                }
            }
            None => (), // If no query parameters exist, do nothing
        };

        // Create and return the Request object
        Ok(Request {
            method: first_line_iter.next().unwrap().into(),
            uri: uri.to_string(),
            version: first_line_iter.next().unwrap().into(),
            headers: headers,
            query_params: query_params,
            reader: reader,
            path_params: HashMap::new(),
        })
    }
}

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
