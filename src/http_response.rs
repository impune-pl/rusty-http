use std::{env, fs};
use std::str::FromStr;
use crate::uniform_resource_identifier::URL;
use crate::http_protocol::HttpProtocol;
use crate::http_request::HttpRequest;
use crate::http_status_code::HttpResponseCode;

pub fn generate_response(mut request: HttpRequest) -> String {
    match request.url.as_str() {
        "/" => return make_response_string(HttpResponseCode::Ok200.to_string().as_str(), &request.protocol, vec![], None),
        x if x.starts_with("/echo/") => return make_response_string(HttpResponseCode::Ok200.to_string().as_str(), &request.protocol, vec![], Some(x.strip_prefix("/echo/").unwrap().to_string())),
        x if x.starts_with("/user-agent") => return make_response_string(HttpResponseCode::Ok200.to_string().as_str(), &request.protocol, vec![], request.headers.remove("User-Agent")),
        x if x.starts_with("/files") => return make_response_from_file(&request.protocol, vec![], request.url ),
        _ => return make_response_string("404 Not Found", &request.protocol, vec![], None)
    }
}

fn make_response_from_file(http_protocol: &HttpProtocol, headers: Vec<String>, url: String) -> String {
    let url =  url.strip_prefix("/files").or(Some("/index.html")).map(String::from).unwrap();
    match get_file_contents(url) {
        Ok(content) => make_response_string(HttpResponseCode::Ok200.to_string().as_str(), http_protocol, headers, Some(content)),
        Err(_) => make_response_string("404 Not Found", http_protocol, vec![], None)
    }
}

fn get_file_contents(url: String) -> Result<String, &'static str> {
    let filesystem_path:String = get_filesystem_path(url)?;
    return fs::read_to_string(filesystem_path).or_else(|e| {
        println!("{}", e.to_string());
        Err("idk what is going on with filesystem")
    });
}

fn get_filesystem_path(url: String) -> Result<String, &'static str> {
    let base_path: String = env::args()
        .skip_while(|arg| !arg.starts_with("--directory"))
        .skip(1)
        .next().or(Some(String::from("."))).unwrap();
    let parsed_url = URL::from_str(url.as_str())?;
    return Ok(base_path + parsed_url.path.clone().as_str().strip_prefix("/").unwrap());
}

fn make_response_string(response_code: &str, protocol: &HttpProtocol, mut headers: Vec<String>, body: Option<String>) -> String {
    match body {
        Some(body) => {
            headers.push(format!("Content-Length: {}", body.len()));
            headers.push("Content-Type: application/octet-stream".to_string());
            let response = format!("{} {}\r\n{}\r\n\r\n{}", protocol, response_code, headers.join("\r\n"), body);
            println!("Responding with:\r\n{}", response);
            return response;
        }
        None => return format!("{} {}\r\n{}\r\n", protocol, response_code, headers.join("\r\n"))
    }
}