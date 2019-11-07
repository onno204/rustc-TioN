use rocket::http::Cookies;

#[get("/login")]
pub fn get(cookies: Cookies) -> String {
    return cookies.get("user_id").map(|cookie| format!("User ID: {}", cookie.value()));
}
