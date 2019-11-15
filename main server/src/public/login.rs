extern crate serde_json;
extern crate serde_derive;
use crate::server::notifications;
use rocket::response::content;

#[derive(Serialize, Deserialize)]
pub struct Message {
    id: i32,
    message: String
}

#[post("/login", format = "application/json", data="<message>")]
pub fn post(message: rocket_contrib::json::Json<Message>) -> content::Json<String> {
    notifications::send_test_message(&message.message).expect("Wow");
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    return content::Json(john.to_string());
}
