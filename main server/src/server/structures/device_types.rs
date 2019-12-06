use std::str::FromStr;
use std::fmt;

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum DeviceTypes {
    Camera = 4,
    Windowsensor = 3,
    Doorsensor = 2,
    Motionsensor = 1
}

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Roles::Camera => write!(f, "Camera"),
           Roles::Windowsensor => write!(f, "Windowsensor"),
           Roles::Doorsensor => write!(f, "Doorsensor"),
           Roles::Motionsensor => write!(f, "Motionsensor"),
       }
    }
}
impl FromStr for Roles {
    type Err = ();
    fn from_str(s: &str) -> Result<Roles, ()> {
        match s {
            "Camera" => Ok(Roles::Camera),
            "Windowsensor" => Ok(Roles::Windowsensor),
            "Doorsensor" => Ok(Roles::Doorsensor),
            "Motionsensor" => Ok(Roles::Motionsensor),
            _ => Err(()),
        }
    }
}
