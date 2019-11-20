extern crate mysql;
use crate::server;
use crate::server::structures;
use std::fmt;

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub security_pool: u32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: structures::roles::Roles,
    pub token: String
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {id}, security_pool: {security_pool}, username: {username}, password: {password}, email: {email}, role: {role}",
            id = self.id,
            security_pool = self.security_pool,
            username = self.username,
            password = self.password,
            email = self.email,
            role = self.role
        )
    }
}

impl User {
    pub fn new(username: &String, password: &String, email: &String, security_pool: &u32, role: &structures::roles::Roles) -> Result<User, String> {
        match user_register(username, password, email, security_pool, role) {
            Ok(v) => v,
            Err(_e) => return Err(format!("failed to register user: {}", _e).to_string())
        };
        let user: User = match user_login(username, password) {
            Ok(v) => v,
            Err(_e) => return Err(format!("failed to login user: {}", _e).to_string())
        };
        return Ok(user)
    }
}

pub fn user_login(_username: &String, _password: &String) -> Result<User, String> {
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"SELECT
                                    id, security_pool, username, password, email, role FROM users
                                    WHERE username = :username ",
                           params!{
                               "username" => _username
                            }
    );
    let result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err("failed to fetch user".to_string())
    };
    for _row in result {
        let row = match _row{
            Ok(v) => v,
            Err(_e) => panic!("[user_login] let row = match _row")
        };
        let (id, security_pool, username, password, email, _role): (u32, u32, String, String, String, String) = mysql::from_row(row);
        if username == _username.to_string() {
            let password_ok = server::security::password_verify(_password.to_string(), password.to_string());
            if password_ok{
                let role: structures::roles::Roles = match _role.parse::<structures::roles::Roles>() {
                    Ok(v) => v,
                    Err(_e) => return Err("error while converting role".to_string())
                };
                let _token: Result<String, String> = update_token(&username);
                let token: String = match _token{
                    Ok(v) => v,
                    Err(_e) => return Err("failed to update token".to_string())
                };

                let user: User = User {
                    id: id,
                    security_pool: security_pool,
                    username: username.to_string(),
                    password: password.to_string(),
                    email: email.to_string(),
                    role: role,
                    token: token.to_string()
                };
                return Ok(user)
            }else {
                return Err("login failed".to_string())
            }
        }
    }
    return Err("login failed".to_string())
}
fn update_token(username: &String) -> Result<String, String> {
    let token: String = server::security::create_random_token();
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"UPDATE users SET
                                    token = :token
                                    WHERE username = :username ",
                           params!{
                               "username" => username,
                               "token" => token.to_string()
                            }
    );
    let _result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err("failed to update token".to_string())
    };
    return Ok(token)
}

pub fn user_register(username: &String, _password: &String, email: &String, security_pool: &u32, role: &structures::roles::Roles) -> Result<(), String> {
    let token: String = server::security::create_random_token();
    let password: String = server::security::password_hash(_password.to_string());
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"INSERT INTO users
                                   (security_pool, username, password, email, role, token) VALUES
                                   (:security_pool, :username, :password, :email, :role, :token)",
                           params!{
                               "security_pool" => security_pool,
                               "username" => username,
                               "password" => password,
                               "email" => email,
                               "role" => role.to_string(),
                               "token" => token
                            }
    );
    let _result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err(format!("{}", _e))
    };
    if _result.affected_rows() < 1 {
        return Err(format!("unknown error: no user created"))
    }
    return Ok(())
}
