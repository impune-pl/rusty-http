mod http_method;
mod http_status_code;
mod parse_request;
mod http_request;
mod http_protocol;

use std::io::{BufReader, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use crate::http_protocol::HttpProtocol;
use crate::http_request::HttpRequest;
use crate::parse_request::parse_http_request;

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening for connections on port 4221");
    for stream in listener.incoming() {
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
    }
}

fn read_http_request(_stream: &mut TcpStream) -> Result<HttpRequest, &str> {
    let reader = BufReader::new(_stream);
    return parse_http_request(reader);
}

fn generate_response(request: HttpRequest) -> String {
    match request.path.as_str() {
        "/" => return make_response_string("200 OK", &request.protocol, vec![], None),
        _ => return make_response_string("404 Not Found", &request.protocol, vec![], None)
    }
}

fn make_response_string(response_code: &str, protocol: &HttpProtocol, headers: Vec<&str>, body: Option<&str>) -> String {
    if body.is_some() {
        return format!("{} {}\r\n{}\r\n{}\r\n", protocol, response_code, headers.join("\r\n"), body.unwrap());
    }
    return format!("{} {}\r\n{}\r\n\r\n", protocol, response_code, headers.join("\r\n"));
}