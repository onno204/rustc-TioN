use std::fmt;

pub enum Errors {
    FileNotFound,
    InternalServerError
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Errors::FileNotFound => write!(f, "error_file_not_found"),
           Errors::InternalServerError => write!(f, "error_internal_server_error"),
       }
    }
}
