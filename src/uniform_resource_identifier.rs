use std::fmt::{Display, Formatter};
use std::borrow::Borrow;
use std::str::FromStr;

use nom::{FindSubstring, Slice};

const SCHEME_SEPARATOR: &str = "://";
const PORT_SEPARATOR: &str = ":";
const PARAMETERS_SEPARATOR: &str = "?";
const ANCHOR_SEPARATOR: &str = "#";
pub(crate) struct URL {
    pub(crate) scheme: Option<Scheme>,
    pub(crate) domain: Option<String>,
    pub(crate) port: Option<u16>,
    pub(crate) path: String,
    pub(crate) parameters: Option<String>,
    pub(crate) anchor: Option<String>,
}

impl FromStr for URL {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scheme_separator_position = s.find_substring(SCHEME_SEPARATOR);
        let port_separator_position = s.find_substring(PORT_SEPARATOR);
        // The only one that is 100% there
        let path_start_position = s.find_substring("/").ok_or("Missing path")?;
        let parameters_separator_position = s.find_substring(PARAMETERS_SEPARATOR);
        let anchor_separator_position = s.find_substring(ANCHOR_SEPARATOR);

        let scheme = match scheme_separator_position {
            None => None,
            Some(pos) => Scheme::from_str(s.slice(0..pos)).ok()
        };

        let domain = match scheme_separator_position {
            None => match s.slice(0..1) {
                s if s.starts_with("/") => None,
                _ => Some(String::from(s.slice(0..path_start_position)))
            },
            Some(scheme_separator_position) => Some(String::from(s.slice(SCHEME_SEPARATOR.len() + scheme_separator_position..port_separator_position.or(Some(path_start_position)).ok_or("Error impossible by all sane logic but required by rust")?)))
        };

        let default_port = match scheme.borrow() {
            None => None,
            Some(scheme) => Some(scheme.get_port())
        };

        Ok(URL {
            scheme,
            domain,
            port: match port_separator_position {
                None => default_port,
                Some(pos) => Some(str::parse::<u16>(s.slice(pos..path_start_position)).or(Err("Error impossible by all sane logic but required by rust"))?)
            },
            path: String::from(s.slice(path_start_position..parameters_separator_position.or(anchor_separator_position).or(Some(s.len())).ok_or("Error impossible by all sane logic but required by rust")?)),
            parameters: match parameters_separator_position {
                None => None,
                Some(pos) => Some(String::from(s.slice(pos..anchor_separator_position.or(Some(s.len())).ok_or("Error impossible by all sane logic but required by rust")?)))
            },
            anchor: match anchor_separator_position {
                None => None,
                Some(pos) => Some(String::from(s.slice(pos..s.len())))
            },
        })
    }
}

// impl Display for URL {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             x if x.parameters.is_some() => write!(f, "{}{SCHEME_SEPARATOR}{}{PORT_SEPARATOR}{}{PARAMETERS_SEPARATOR}{}", self.scheme, self.domain, self.path, x.parameters.clone().unwrap()),
//             x if x.anchor.is_some() => write!(f, "{}{SCHEME_SEPARATOR}{}{PORT_SEPARATOR}{}{ANCHOR_SEPARATOR}{}", self.scheme, self.domain, self.path, x.anchor.clone().unwrap()),
//             x if x.parameters.is_some() && x.anchor.is_some() => write!(f, "{}{SCHEME_SEPARATOR}{}{PORT_SEPARATOR}{}{PARAMETERS_SEPARATOR}{}{ANCHOR_SEPARATOR}{}", x.scheme, x.domain, x.path, x.parameters.clone().unwrap(), x.anchor.clone().unwrap()),
//             _ => write!(f, "{}{SCHEME_SEPARATOR}{}{PORT_SEPARATOR}{}", self.scheme, self.domain, self.path)
//         }
//     }
// }

pub enum Scheme {
    HTTP,
    HTTPS,
}

impl Scheme {
    fn get_port(&self) -> u16 {
        match self {
            Scheme::HTTP => 80,
            Scheme::HTTPS => 443
        }
    }
}

impl FromStr for Scheme {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http" => Ok(Scheme::HTTP),
            "https" => Ok(Scheme::HTTPS),
            _ => Err("Unknown scheme, this server only handles http.")
        }
    }
}

impl Display for Scheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Scheme::HTTP => write!(f, "http"),
            Scheme::HTTPS => write!(f, "https")
        }
    }
}
