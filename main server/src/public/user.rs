extern crate serde_json;
extern crate serde_derive;
use crate::server::structures;
use crate::server::authorization;
use rocket::response::content;

#[derive(Serialize, Deserialize)]
pub struct InfoData {
    token: Option<String>,
    username: Option<String>
}

#[post("/info", format="application/json", data="<data>")]
pub fn info_post(data: Option<rocket_contrib::json::Json<InfoData>>) -> Result<content::Json<String>, content::Json<String>> {
    if let Some(data) = data {
        let _token = match &data.token { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "token")}).to_string())) };
        let _username = match &data.username { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "username")}).to_string())) };
        if authorization::has_token_no_role(_token, structures::roles::Roles::Admin){ return Err(content::Json(json!({"success": false, "error": "no_permissions"}).to_string())) }

        let user: structures::user::User = match structures::user::User::get_from_username(&_username) {
            Ok(v) => v,
            Err(_e) => return Err(content::Json(json!({"success": false, "error": _e}).to_string()))
        };

        return Ok(content::Json(json!({"success": true, "username": user.username, "email": user.email, "id": user.id, "role": user.role.to_string(), "security_pool": user.security_pool}).to_string()))
    }else {
        return Err(content::Json(json!({"success": false, "error": "missing json data"}).to_string()))
    }
    // notifications::send_test_message(&message.message).expect("Wow");
}
