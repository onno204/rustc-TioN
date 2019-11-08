extern crate reqwest;
extern crate serde_json;
use std::collections::HashMap;
use reqwest::Error;
use serde_json::{Result, Value};


pub fn send_test_message() -> Result<(), Error> {
    let do_steps = || -> Result<(), Error> {
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
        let v: Value = serde_json::from_str(&_response.text()?.to_string());
        // println!("STATUS: {}", response["status"]);
        Ok(())
    };

    if let Err(_err) = do_steps() {
        println!("Failed to perform necessary steps");
    }
    Ok(())
}
