extern crate serde_json;
extern crate serde_derive;
use crate::server::structures;
use rocket::response::content;

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    security_pool: Option<u32>,
    role: Option<String>
}

#[post("/login", format="application/json", data="<login_data>")]
pub fn post(login_data: Option<rocket_contrib::json::Json<LoginData>>) -> Result<content::Json<String>, content::Json<String>> {
    if let Some(login_data) = login_data {
        let _role = match &login_data.role { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "role")}).to_string())) };
        let _username = match &login_data.username { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "username")}).to_string())) };
        let _password = match &login_data.password { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "password")}).to_string())) };
        let _email = match &login_data.email { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "email")}).to_string())) };
        let _security_pool = match login_data.security_pool { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "security_pool")}).to_string())) };

        let role: structures::roles::Roles = match _role.parse::<structures::roles::Roles>() {
            Ok(v) => v,
            Err(_e) => return Err(content::Json(json!({"success": false, "error": format!("role not found: {}", _role)}).to_string()))
        };
        let _user: structures::user::User = structures::user::User::new(_username.to_string(), _password.to_string(), _email.to_string(), _security_pool, role);
        Ok(content::Json(json!({"success": true, "message": format!("{}", _user)}).to_string()))
    }else {
        return Err(content::Json(json!({"success": false, "error": "missing json data"}).to_string()))
    }
    // notifications::send_test_message(&message.message).expect("Wow");
}
