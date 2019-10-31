
#[get("/test")]
pub fn get() -> &'static str {
    return "test!";
}
