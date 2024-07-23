use std::fmt::{Display, Formatter};

pub enum HttpResponseCode {
    Ok200,
    NotFound404,
    Error400,
    Created201,
}

impl Display for HttpResponseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpResponseCode::Ok200 => write!(f, "200 OK"),
            HttpResponseCode::Created201 => write!(f, "201 Created"),
            HttpResponseCode::NotFound404 => write!(f, "404 NOT FOUND"),
            HttpResponseCode::Error400 => write!(f, "400 ERROR")
        }
    }
}