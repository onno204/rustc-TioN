use crate::server::structures;
use std::fmt;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

pub fn has_token_permissions(token: &String, permission: &String) -> bool{
    let user: structures::user::User = match structures::user::User::get_from_token(&token) {
        Ok(v) => v,
        Err(_e) => return false
    };
    if permission == "21341234123" && structures::roles::Roles::is_role(&user.role, &structures::roles::Roles::Admin) { return true }
    return false
}
pub fn has_token_no_permissions(token: &String, permission: &String) -> bool{
    return !has_token_permissions(token, permission)
}

pub fn has_token_role(token: &String, permission: structures::roles::Roles) -> bool{
    let user: structures::user::User = match structures::user::User::get_from_token(&token) {
        Ok(v) => v,
        Err(_e) => return false
    };
    if structures::roles::Roles::is_role(&user.role, &permission) { return true }
    return false
}
pub fn has_token_no_role(token: &String, permission: structures::roles::Roles) -> bool{
    return !has_token_role(token, permission)
}

// Create API verifier
#[derive(Serialize, Debug)]
pub struct ApiKeyAdmin(String);
pub struct ApiKeyUser(String);
pub struct ApiKeyDevice(String);
pub struct ApiKey(String);

fn check_apikey_role(request: &Request, role: structures::roles::Roles) -> Result<String, ()>{
    let keys: Vec<_> = request.headers().get("Authorization").collect();
    if keys.len() != 1 {
        return Err(())
    }

    let _key: &str = keys[0];
    let key: String = str::replace(_key, "Bearer ", "");
    // Fail if key is to short(To prevent empty keys)
    if key.len() <= 10{
        return Err(());
    }
    if !has_token_role(&key.to_string(), role) {
        return Err(());
    }

    return Ok(key.to_string());
}

impl fmt::Display for ApiKeyAdmin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", str::replace(&self.0[..], "Bearer ", ""))
    }
}
impl fmt::Display for ApiKeyUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", str::replace(&self.0[..], "Bearer ", ""))
    }
}
impl fmt::Display for ApiKeyDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", str::replace(&self.0[..], "Bearer ", ""))
    }
}
impl<'a, 'r> FromRequest<'a, 'r> for ApiKeyAdmin {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKeyAdmin, ()> {
        match check_apikey_role(request, structures::roles::Roles::Admin) {
            Ok(v) => return Outcome::Success(ApiKeyAdmin(v)),
            Err(_e) => return Outcome::Failure((Status::new(401, "Unauthorized"), ()))
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKeyUser {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKeyUser, ()> {
        match check_apikey_role(request, structures::roles::Roles::User) {
            Ok(v) => return Outcome::Success(ApiKeyUser(v)),
            Err(_e) => return Outcome::Failure((Status::new(401, "Unauthorized"), ()))
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKeyDevice {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKeyDevice, ()> {
        match check_apikey_role(request, structures::roles::Roles::Device) {
            Ok(v) => return Outcome::Success(ApiKeyDevice(v)),
            Err(_e) => return Outcome::Failure((Status::new(401, "Unauthorized"), ()))
        }
    }
}
