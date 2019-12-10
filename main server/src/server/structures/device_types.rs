use std::str::FromStr;
use std::fmt;

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum DeviceTypes {
    Camera = 4,
    Windowsensor = 3,
    Doorsensor = 2,
    Motionsensor = 1
}

impl fmt::Display for DeviceTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           DeviceTypes::Camera => write!(f, "Camera"),
           DeviceTypes::Windowsensor => write!(f, "Windowsensor"),
           DeviceTypes::Doorsensor => write!(f, "Doorsensor"),
           DeviceTypes::Motionsensor => write!(f, "Motionsensor"),
       }
    }
}
impl FromStr for DeviceTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<DeviceTypes, ()> {
        match s {
            "Camera" => Ok(DeviceTypes::Camera),
            "Windowsensor" => Ok(DeviceTypes::Windowsensor),
            "Doorsensor" => Ok(DeviceTypes::Doorsensor),
            "Motionsensor" => Ok(DeviceTypes::Motionsensor),
            _ => Err(()),
        }
    }
}
