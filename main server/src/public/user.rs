extern crate serde_json;
extern crate serde_derive;
use crate::server::structures;
use crate::server::authorization;
use rocket::response::content;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

#[derive(Serialize, Debug)]
pub struct ApiKey(String);

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey<role: structures::roles::Roles> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        println!("{:?}", keys);
        if keys.len() != 1 {
            return Outcome::Failure((Status::new(401, "Unauthorized"), ()));
        }

        let key = keys[0];
        if !authorization::has_token_no_role(&key.to_string(), role) {
            return Outcome::Forward(());
        }

        return Outcome::Success(ApiKey(key.to_string()));
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct InfoData {
    username: Option<String>
}

#[post("/info", format="application/json", data="<data>")]
pub fn info_post(data: Option<rocket_contrib::json::Json<InfoData>>, _key: ApiKey<structures::roles::Roles>) -> Result<content::Json<String>, content::Json<String>> {
    //println!("{:?}", data);
    if let Some(data) = data {
        // let _token = match &data.token { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "token")}).to_string())) };
        let _username = match &data.username { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "username")}).to_string())) };
        //

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
