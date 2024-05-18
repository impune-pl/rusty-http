use std::io::Write;
// Uncomment this block to pass the first stage
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    const RESPONSE: &[u8] = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let _ = _stream.write(RESPONSE);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
