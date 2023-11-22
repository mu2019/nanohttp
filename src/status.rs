#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    SwitchingProtocols,
    Ok,
    SeeOther,
    NotFound,
    InternalServerError,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotAllowed,
}

impl Status {
    /// Get the numeric representation of the status code.
    fn code(&self) -> u16 {
        match self {
            Status::SwitchingProtocols => 101,
            Status::Ok => 200,
            Status::SeeOther => 303,
            Status::BadRequest => 400,
            Status::Unauthorized => 401,
            Status::Forbidden => 403,
            Status::NotFound => 404,
            Status::NotAllowed => 405,
            Status::InternalServerError => 500,
        }
    }

    fn message(&self) -> &str {
        // Get the status message.
        match self {
            Status::SwitchingProtocols => "SWITCHING PROTOCOLS",
            Status::Ok => "OK",
            Status::SeeOther => "SEE OTHER",
            Status::BadRequest => "BAD REQUEST",
            Status::Unauthorized => "UNAUTHORIZED",
            Status::Forbidden => "FORBIDDEN",
            Status::NotFound => "NOT FOUND",
            Status::NotAllowed => "NOT ALLOWED",
            Status::InternalServerError => "INTERNAL SERVER ERROR",
        }
    }
}

impl ToString for Status {
    /// Convert the `Status` to a valid http plaintext representation.
    fn to_string(&self) -> String {
        format!("{} {}", self.code(), self.message())
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let parser_err = Error {
            err_type: ErrorType::ParserError,
            msg: "Invalid status format".to_string(),
        };
        match code {
            "101" => Ok(Self::SwitchingProtocols),
            "200" => Ok(Self::Ok),
            "303" => Ok(Self::SeeOther),
            "400" => Ok(Self::BadRequest),
            "401" => Ok(Self::Unauthorized),
            "403" => Ok(Self::Forbidden),
            "404" => Ok(Self::NotFound),
            "405" => Ok(Self::NotAllowed),
            "500" => Ok(Self::InternalServerError),
            _ => Err(parser_err)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Status;

    #[test]
    fn status_code() {
        let result = Status::Ok.code();
        let expected = 200;

        assert_eq!(result, expected);
    }

    #[test]
    fn status_message() {
        let result = Status::InternalServerError.message();
        let expected = "INTERNAL SERVER ERROR";

        assert_eq!(result, expected);
    }

    #[test]
    fn string_representation() {
        let result = Status::NotFound.to_string();
        let expected = "404 NOT FOUND".to_string();

        assert_eq!(result, expected);
    }
}
