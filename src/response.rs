use std::fmt::{Display, Formatter};

use crate::error::AssuanError;

#[derive(Debug, PartialEq)]
pub enum Response {
    Ok(Option<String>),
    Error(AssuanError),
    Data(String),
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let string = match self {
            Response::Ok(data) => match data {
                Some(data) => format!("OK {}", data),
                None => "OK".to_string(),
            },
            Response::Error(error) => error.to_string(),
            Response::Data(data) => format!("D {}", data),
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_response_converts_to_string() {
        assert_eq!(Response::Ok(None).to_string(), "OK");
        assert_eq!(Response::Ok(Some("foo".to_string())).to_string(), "OK foo");
    }

    #[test]
    fn error_response_converts_to_string() {
        assert_eq!(
            Response::Error(AssuanError::UnknownIPCCommand).to_string(),
            "ERR 536871187 Unknown IPC command <User defined source 1>"
        );
    }

    #[test]
    fn data_response_converts_to_string() {
        assert_eq!(Response::Data("foo".to_string()).to_string(), "D foo");
    }
}
