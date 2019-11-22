extern crate serde_json;
extern crate serde_derive;
use crate::server::structures;
use rocket::response::content;

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    username: Option<String>,
    password: Option<String>
}

#[post("/login", format="application/json", data="<login_data>")]
pub fn post(login_data: Option<rocket_contrib::json::Json<LoginData>>) -> Result<content::Json<String>, content::Json<String>> {
    if let Some(login_data) = login_data {
        let _username = match &login_data.username { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "username")}).to_string())) };
        let _password = match &login_data.password { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "password")}).to_string())) };

        let user: structures::user::User = match structures::user::User::login(&_username, &_password) {
            Ok(v) => v,
            Err(_e) => return Err(content::Json(json!({"success": false, "error": _e}).to_string()))
        };

        return Ok(content::Json(json!({"success": true, "token": user.token}).to_string()))
    }else {
        return Err(content::Json(json!({"success": false, "error": "missing json data"}).to_string()))
    }
    // notifications::send_test_message(&message.message).expect("Wow");
}
