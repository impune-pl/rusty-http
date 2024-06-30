use std::io::{BufReader, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

use crate::http_request::HttpRequest;
use crate::http_response::generate_response;
use crate::parse_request::parse_http_request;

mod http_method;
mod http_status_code;
mod parse_request;
mod http_request;
mod http_protocol;
mod http_response;
mod uniform_resource_identifier;

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening for connections on port 4221");
    for stream in listener.incoming() {
        thread::spawn(||
            match stream {
                Ok(mut _stream) => {
                    println!("Accepted new connection");
                    println!("Reading request");
                    match read_http_request(&mut _stream) {
                        Ok(request) => {
                            println!("Request read: {}", request);
                            let response: String = generate_response(request);
                            println!("Responding with {}", response);
                            let _ = _stream.write(response.as_bytes());
                            println!("Response sent");
                        }
                        Err(e) => println!("{}", e.to_string())
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        );
    }
}

fn read_http_request(_stream: &mut TcpStream) -> Result<HttpRequest, &str> {
    let reader = BufReader::new(_stream);
    return parse_http_request(reader);
}
