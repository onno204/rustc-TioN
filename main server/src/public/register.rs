extern crate serde_json;
extern crate serde_derive;
use crate::server::structures;
use crate::server::authorization;
use rocket::response::content;

#[derive(Serialize, Deserialize)]
pub struct RegisterData {
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    security_pool: Option<u32>,
    role: Option<String>
}

#[post("/register", format="application/json", data="<register_data>")]
pub fn post(register_data: Option<rocket_contrib::json::Json<RegisterData>>, _key: authorization::ApiKeyAdmin) -> Result<content::Json<String>, content::Json<String>> {
    if let Some(register_data) = register_data {
        let _role = match &register_data.role { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "role")}).to_string())) };
        let _username = match &register_data.username { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "username")}).to_string())) };
        let _password = match &register_data.password { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "password")}).to_string())) };
        let _email = match &register_data.email { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "email")}).to_string())) };
        let _security_pool = match register_data.security_pool { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "security_pool")}).to_string())) };

        let role: structures::roles::Roles = match _role.parse::<structures::roles::Roles>() {
            Ok(v) => v,
            Err(_e) => return Err(content::Json(json!({"success": false, "error": format!("role not found: {}", _role)}).to_string()))
        };

        let _user: structures::user::User = match structures::user::User::new(&_username, &_password, &_email, &_security_pool, &role) {
            Ok(v) => v,
            Err(_e) => return Err(content::Json(json!({"success": false, "error": _e}).to_string()))
        };
        Ok(content::Json(json!({"success": true, "token": _user.token}).to_string()))
    }else {
        return Err(content::Json(json!({"success": false, "error": "missing json data"}).to_string()))
    }
    // notifications::send_test_message(&message.message).expect("Wow");
}
