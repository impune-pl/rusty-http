use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use itertools::Itertools;
use crate::http_method::HttpMethod;
use crate::http_protocol::HttpProtocol;

pub(crate) struct HttpRequest {
    pub(crate) request_method : HttpMethod,
    pub(crate) path: String,
    pub(crate) protocol: HttpProtocol,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: String
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\r\n{}\r\n\r\n{}\r\n", self.request_method, self.path, self.protocol, self.headers.iter().map(|(k,v)| format!("{} {}\r\n", k, v)).join(", "), self.body)
    }
}