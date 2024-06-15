use std::fmt::{Display, Formatter};

pub enum HttpResponseCode {
    Ok200,
    NotFound404,
    Error400
}

impl Display for HttpResponseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpResponseCode::Ok200 => write!(f, "200 OK"),
            HttpResponseCode::NotFound404 => write!(f, "404 NOT FOUND"),
            HttpResponseCode::Error400 => write!(f, "400 ERROR")
        }
    }
}