
#[get("/")]
pub fn get() -> &'static str {
    return "pong";
}
