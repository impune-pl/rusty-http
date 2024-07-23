use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use std::str::FromStr;

use crate::http_method::HttpMethod;
use crate::http_protocol::HttpProtocol;
use crate::http_request::HttpRequest;

pub(crate) fn parse_http_request(mut request_buffered_reader: BufReader<&mut TcpStream>) -> Result<HttpRequest, &str> {
    let mut fline: Vec<u8> = vec![];
    let skipfline = request_buffered_reader.read_until(0xA, &mut fline).expect("Dupa");
    let start_line = match String::from_utf8(fline) {
        Err(_) => return Err("No first line in request"),
        Ok(line) => match parse_start_line(line.to_string()) {
            Ok(start_line) => start_line,
            Err(e) => return Err(e)
        }
    };
    println!("sline: {} {} {}", start_line.0, start_line.1, start_line.2);
    let mut h_line: Vec<Vec<u8>> = vec![];
    let mut lh_line: Vec<u8>;
    loop {
        lh_line = vec![];
        request_buffered_reader.read_until(0xA, &mut lh_line).expect("Dupa");
        h_line.push(lh_line);
        if h_line.last().unwrap().len() == 2 {
            break;
        }
    }
    let header_lines: Vec<String> = h_line.into_iter()
                                          .map(|x| String::from_utf8(x).unwrap().trim().to_string())
                                          .filter(|x| !x.is_empty())
                                          .collect();
    println!("headers: {}", header_lines.join("-"));
    let headers = parse_headers(&header_lines);
    let content_bytes: u64 = headers.get("Content-Length")
                                    .or(Some(&String::from("0")))
                                    .map(|x| x.parse().unwrap())
                                    .unwrap();
    let mut bd: Vec<u8> = Vec::new();
    bd.resize(content_bytes.try_into().unwrap(), 0);
    request_buffered_reader.read_exact(&mut bd).expect("Dupa");
    let body = String::from_utf8(bd).unwrap();
    println!("body: {}", body);
    return Ok(HttpRequest {
        request_method: start_line.0,
        url: start_line.1,
        protocol: start_line.2,
        headers,
        body,
    });
}

fn parse_headers(lines: &Vec<String>) -> HashMap<String, String> {
    return lines.iter().clone().skip(1).take_while(|s| !s.starts_with("\r\n"))
                .map(|s| s.split_once(':').unwrap())
                .map(|(k, v)| (k.trim(), v.trim()))
                .fold(HashMap::new(), |mut map, (k, v)| {
                    map.insert(k.to_string(), v.to_string()).and_then(|ov| map.insert(k.to_string(), ov + "," + v));
                    map
                });
}

fn parse_start_line(line: String) -> Result<(HttpMethod, String, HttpProtocol), &'static str> {
    let mut words = line.split_ascii_whitespace();
    let method: HttpMethod = match words.next() {
        None => return Err("Request first line unreadable: method missing"),
        Some(word) => HttpMethod::from_str(word)?
    };
    let url: String = match words.next() {
        None => return Err("Request first line unreadable: path missing"),
        Some(word) => word.to_string()
    };
    let protocol = match words.next() {
        None => return Err("Request first line unreadable: protocol missing"),
        Some(word) => HttpProtocol::from_str(word)?
    };
    return Ok((
        method,
        url,
        protocol
    ));
}
