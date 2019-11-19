use std::str::FromStr;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Roles {
    Admin,
    User,
    Device
}

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Roles::Admin => write!(f, "Admin"),
           Roles::User => write!(f, "User"),
           Roles::Device => write!(f, "Device"),
       }
    }
}
impl FromStr for Roles {
    type Err = ();

    fn from_str(s: &str) -> Result<Roles, ()> {
        match s {
            "Admin" => Ok(Roles::Admin),
            "User" => Ok(Roles::User),
            "Device" => Ok(Roles::Device),
            _ => Err(()),
        }
    }
}
