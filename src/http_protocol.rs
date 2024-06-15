use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum HttpProtocol{
    HTTP1_1
}

impl Display for HttpProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { HttpProtocol::HTTP1_1 => write!(f, "HTTP/1.1") }
    }
}

impl FromStr for HttpProtocol {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(HttpProtocol::HTTP1_1),
            _ => Err("dupa")
        }
    }
}