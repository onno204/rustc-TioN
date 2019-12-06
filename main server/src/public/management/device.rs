extern crate serde_json;
extern crate serde_derive;
use crate::server::structures;
use crate::server::authorization;
use rocket::response::content;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddDeviceData {
    devicename: Option<String>,
    devicetype: Option<String>
}

#[post("/add", format="application/json", data="<data>")]
pub fn add_device(data: Option<rocket_contrib::json::Json<AddDeviceData>>, _key: authorization::ApiKeyAdmin) -> Result<content::Json<String>, content::Json<String>> {
    if let Some(data) = data {
        let _devicename = match &data.devicename { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "devicename")}).to_string())) };
        let _devicetype = match &data.devicetype { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "devicetype")}).to_string())) };

        let user: structures::device::Device = match structures::device::Device::get_from_username(&_username) {
            Ok(v) => v,
            Err(_e) => return Err(content::Json(json!({"success": false, "error": _e}).to_string()))
        };

        return Ok(content::Json(json!({"success": true, "username": user.username, "email": user.email, "id": user.id, "role": user.role.to_string(), "security_pool": user.security_pool}).to_string()))
    }else {
        return Err(content::Json(json!({"success": false, "error": "missing json data"}).to_string()))
    }
    // notifications::send_test_message(&message.message).expect("Wow");
}
