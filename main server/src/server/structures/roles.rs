use std::str::FromStr;
use std::fmt;
use std::mem::discriminant;

#[derive(Debug, Copy, Clone)]
pub enum Roles {
    Admin = 3,
    User = 2,
    Device = 1
}
impl Roles {
    pub fn is_role(user_role: &Roles, target_role: &Roles) -> bool{
        if discriminant(user_role) == discriminant(target_role){ return true }
        if discriminant(user_role) == discriminant(&Roles::Admin)
            && (discriminant(target_role) == discriminant(&Roles::Admin)
                || discriminant(target_role) == discriminant(&Roles::User)
                    || discriminant(target_role) == discriminant(&Roles::Device))
                { return true }

        if discriminant(user_role) == discriminant(&Roles::User)
            && (discriminant(target_role) == discriminant(&Roles::User)
                || discriminant(target_role) == discriminant(&Roles::Device))
                { return true }

        if discriminant(user_role) == discriminant(&Roles::Device)
            && discriminant(target_role) == discriminant(&Roles::Device)
                { return true }
        return false
    }
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
