use crate::server::structures;

pub fn has_token_permissions(token: &String, permission: &String) -> bool{
    let user: structures::user::User = match structures::user::User::get_from_token(&token) {
        Ok(v) => v,
        Err(_e) => return false
    };
    if permission == "21341234123" && structures::roles::Roles::is_role(&user.role, &structures::roles::Roles::Admin) { return true }
    return false
}
pub fn has_token_no_permissions(token: &String, permission: &String) -> bool{
    return !has_token_permissions(token, permission)
}

pub fn has_token_role(token: &String, permission: structures::roles::Roles) -> bool{
    let user: structures::user::User = match structures::user::User::get_from_token(&token) {
        Ok(v) => v,
        Err(_e) => return false
    };
    if structures::roles::Roles::is_role(&user.role, &permission) { return true }
    return false
}
pub fn has_token_no_role(token: &String, permission: structures::roles::Roles) -> bool{
    return !has_token_role(token, permission)
}
