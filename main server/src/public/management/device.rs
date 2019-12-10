extern crate serde_json;
extern crate serde_derive;
use crate::server::structures;
use crate::server::authorization;
use rocket::response::content;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddDeviceData {
    devicename: Option<String>,
    devicetype: Option<String>,
    room: Option<String>
}

#[post("/add", format="application/json", data="<data>")]
pub fn add_device(data: Option<rocket_contrib::json::Json<AddDeviceData>>, _key: authorization::ApiKeyAdmin) -> Result<content::Json<String>, content::Json<String>> {
    if let Some(data) = data {
        let _devicename = match &data.devicename { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "devicename")}).to_string())) };
        let _devicetype = match &data.devicetype { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "devicetype")}).to_string())) };
        let _room = match &data.room { Some(x) => x, None => return Err(content::Json(json!({"success": false, "error": format!("missing argument: {}", "room")}).to_string())) };

        let user: structures::user::User = match structures::user::User::get_from_token(&_key.to_string()) {
            Ok(v) => v,
            Err(_e) => return Err(content::Json(json!({"success": false, "error": _e}).to_string()))
        };
        let device_type: structures::device_types::DeviceTypes = match _devicetype.parse::<structures::device_types::DeviceTypes>() {
            Ok(v) => v,
            Err(_e) => return Err(content::Json(json!({"success": false, "error": format!("device_type not found: {}", _devicetype)}).to_string()))
        };
        let device: structures::device::Device = match structures::device::Device::new(&_devicename, &device_type, &_room, &user) {
            Ok(v) => v,
            Err(_e) => return Err(content::Json(json!({"success": false, "error": _e}).to_string()))
        };

        return Ok(content::Json(json!({"success": true, "device": device.devicename, "token": device.token, "id": device.id, "room": device.room, "devicetype": device.device_type.to_string(), "security_pool": device.security_pool}).to_string()))
    }else {
        return Err(content::Json(json!({"success": false, "error": "missing json data"}).to_string()))
    }
}
