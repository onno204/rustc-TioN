extern crate serde_json;
extern crate serde_derive;
use crate::server::structures;
use crate::server::authorization;
use rocket::response::content;

#[derive(Serialize, Deserialize)]
pub struct CreatePoolData {
    token: Option<String>,
    name: Option<String>,
    owner: Option<u32>
}

#[post("/create", format="application/json", data="<data>")]
pub fn create_post(data: Option<rocket_contrib::json::Json<CreatePoolData>>) -> Result<content::Json<String>, content::Json<String>> {
    if let Some(data) = data {
        let _token = match &data.token { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "token")}).to_string())) };
        let _name = match &data.name { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "name")}).to_string())) };
        let _owner = match &data.owner { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "owner")}).to_string())) };
        if authorization::has_token_no_role(_token, structures::roles::Roles::Admin){ return Err(content::Json(json!({"success": false, "error": "no_permissions"}).to_string())) }

        let _pool: structures::security_pool::SecurityPool = match structures::security_pool::SecurityPool::new(_name, _owner) {
            Ok(v) => v,
            Err(_e) => return Err(content::Json(json!({"success": false, "error": _e}).to_string()))
        };
        Ok(content::Json(json!({"success": true, "pool_id": _pool.id}).to_string()))
    }else {
        return Err(content::Json(json!({"success": false, "error": "missing json data"}).to_string()))
    }
    // notifications::send_test_message(&message.message).expect("Wow");
}
