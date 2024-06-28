#[derive(Debug, PartialEq)]
pub enum Response {
    Ok(Option<String>),
    Error(i32, Option<String>),
    //Info(String, String),
    //Comment(String),
    Data(String),
    //Inquire(String),
}

impl ToString for Response {
    fn to_string(&self) -> String {
        match self {
            Response::Ok(data) => match data {
                Some(data) => format!("OK {}", data),
                None => "OK".to_string(),
            },
            Response::Error(code, data) => match data {
                Some(data) => format!("ERR {} {}", code, data),
                None => format!("ERR {}", code),
            },
            Response::Data(data) => format!("D {}", data),
        }
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
        assert_eq!(Response::Error(1, None).to_string(), "ERR 1");
        assert_eq!(
            Response::Error(1, Some("foo".to_string())).to_string(),
            "ERR 1 foo"
        );
    }

    #[test]
    fn data_response_converts_to_string() {
        assert_eq!(Response::Data("foo".to_string()).to_string(), "D foo");
    }
}
