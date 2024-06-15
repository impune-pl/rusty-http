use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::str::FromStr;
use crate::http_method::HttpMethod;
use crate::http_protocol::HttpProtocol;
use crate::http_request::HttpRequest;

pub(crate) fn parse_http_request(request_buffered_reader: BufReader<&mut TcpStream>) -> Result<HttpRequest, &str> {
    let request: Vec<String> = request_buffered_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("request: {}", request.join(", "));
    let start_line = match request.get(0) {
        None => return Err("No first line in request"),
        Some(line) => match parse_start_line(line.to_string()) {
            Ok(start_line) => start_line,
            Err(e) => return Err(e)
        }
    };
    let headers = parse_headers(request);
    let body = "".to_string();

    return Ok(HttpRequest {
        request_method: start_line.0,
        path: start_line.1,
        protocol: start_line.2,
        headers,
        body
    });
}

fn parse_headers(_lines: Vec<String>) -> HashMap<String,String> {
    return HashMap::new();
}

fn parse_start_line(line : String) -> Result<(HttpMethod, String, HttpProtocol), &'static str> {
    let mut words = line.split_ascii_whitespace();
    let method: HttpMethod = match words.next() {
        None => return Err("Request first line unreadable: method missing"),
        Some(word) => HttpMethod::from_str(word)?
    };
    let path: String = match words.next() {
        None => return Err("Request first line unreadable: path missing"),
        Some(word) => word.to_string()
    };
    let protocol = match words.next() {
        None => return Err("Request first line unreadable: protocol missing"),
        Some(word) => HttpProtocol::from_str(word)?
    };
    return Ok((
        method,
        path,
        protocol
    ))
}
