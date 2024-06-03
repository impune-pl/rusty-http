use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                println!("reading request:");
                let request_parsed: Result<HttpRequest, Err> = read_http_request(*_stream);
                let response: String = match request_parsed {
                    Ok(request) => {
                       match request.path {
                           "/" => make_response_string(&HttpResponseCode::OK200, request.protocol, vec![], None),
                           _ => make_response_string(&HttpResponseCode::ERR404, request.protocol, vec![], None)
                       }
                    }
                    Err(_) => {
                        make_response_string(&HttpResponseCode::ERR404, "HTTP/1.1", vec![], None)
                    }
                };

                let _ = _stream.write(response.as_bytes());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

struct HttpRequest<'a> {
    request_method : HttpRequestMethod,
    path : &'a str,
    protocol: &'a str,
    headers: Vec<&'a str>,
    body: &'a str
}

enum HttpRequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

fn read_http_request(_stream : &mut TcpStream) -> Result<HttpRequest, Err> {
    let request : &mut String = String::new().as_mut_string();
    let _ = _stream.read_to_string(request);
    return parse_http_request(request);
}

fn parse_http_request(request_string: &str) -> Result<HttpRequest, Err> {
    let mut request_lines = request_string.lines();

    // method, path, protocol
    let maybe_first_line = request_lines.next();
    let first_line = match maybe_first_line {
        Some(mut first_line) => {
            let portions = first_line.split_whitespace();
            (
                parse_http_request_method(portions[0]),
                portions[1],
                portions[2]
            )
        }
        None => {
            return Err("No first line in request")
        }
    };

    // headers
    let mut headers : Vec<&str> = !vec![];
    while let Some(line) = request_lines.next() {
        if line.is_empty() { break };
        headers.push(line);
    }
    // body
    let mut body : &str = request_lines.collect();

    return Ok(HttpRequest {
        request_method: first_line[0],
        path: first_line[1],
        protocol: first_line[2],
        headers,
        body
    })
}

fn parse_http_request_method(method_specifier: &str)  -> Result<HttpRequestMethod, Err> {
    match method_specifier {
        "GET"=> Ok(HttpRequestMethod::GET),
        "POST"=> Ok(HttpRequestMethod::POST),
        "PUT"=> Ok(HttpRequestMethod::PUT),
        "DELETE"=> Ok(HttpRequestMethod::DELETE),
        "HEAD"=> Ok(HttpRequestMethod::HEAD),
        "CONNECT"=> Ok(HttpRequestMethod::CONNECT),
        "OPTIONS"=> Ok(HttpRequestMethod::OPTIONS),
        "TRACE"=> Ok(HttpRequestMethod::TRACE),
        "PATCH"=> Ok(HttpRequestMethod::PATCH),
        _ => Err("Unknown http method: ".to_owned() + method_specifier)
    }
}

#[non_exhaustive]
struct HttpResponseCode;

impl HttpResponseCode {
    pub const OK200: str = *"200 OK";
    pub const ERR404: str = *"404 Not Found";
    pub const ERR400: str = *"400 Bad Request";
}

fn make_response_string(response_code : &str, protocol: &str, headers: Vec<&str>, body: Option<&str>) -> String {
    if body.is_some() {
        return format!("{} {}\r\n{}\r\n{}\r\n", protocol, response_code, headers.join("\r\n"), body);
    }
    return format!("{} {}\r\n{}\r\n\r\n", protocol, response_code, headers.join("\r\n"));
}