extern crate reqwest;
extern crate serde_json;
use std::collections::HashMap;

pub fn send_test_message() -> Result<(), reqwest::Error> {
    let request_url = format!("https://api.pushover.net/1/messages.json");
    let mut params = HashMap::new();
    params.insert("token", "afip8fu96zoeaikx9oc4obzi6hzb5e");
    params.insert("user", "u7bmikzkj575u6symxpxjhoueij162");
    params.insert("message", "YEET");
    params.insert("priority", "1");
    let client = reqwest::Client::new();
    let mut _response = client.post(&request_url)
        .form(&params)
        .send()?;
    let output: serde_json::Value = get_string_from_json(&_response.text()?).unwrap();
    println!("STATUS: {}", output["status"]);
    Ok(())

}

fn get_string_from_json(input_string: &str) -> Result<serde_json::Value, ()> {
    let response = serde_json::from_str(&input_string.to_string()).unwrap();
    Ok(response)
}
