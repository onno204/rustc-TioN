extern crate serde_json;
use crate::server::notifications::send_test_message;
use rocket::response::content;

#[post("/login")]
pub fn post() -> content::Json<String> {
    send_test_message();
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
