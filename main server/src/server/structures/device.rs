extern crate mysql;
use crate::server;
use crate::server::structures;
use std::fmt;

#[derive(Debug)]
pub struct Device {
    pub id: u32,
    pub security_pool: u32,
    pub devicename: String,
    pub type: String,
    pub token: String
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {id}, security_pool: {security_pool}, devicename: {devicename}, type: {type}, token: {token}",
            id = self.id,
            security_pool = self.security_pool,
            devicename = self.devicename,
            type = self.type,
            token = self.token
        )
    }
}

impl Device {
    pub fn new(_devicename: &String, _user: &structures::user::User) -> Result<Device, String> {
        match device_register(_devicename, _user) {
            Ok(v) => v,
            Err(_e) => return Err(format!("failed to register user: {}", _e).to_string())
        };
        let user: User = match user_login(username, password) {
            Ok(v) => v,
            Err(_e) => return Err(format!("failed to login user: {}", _e).to_string())
        };
        return Ok(user)
    }
    pub fn get_from_id(_id: &u32) -> Result<Device, String>{
        return get_device_by_id(_id)
    }
    pub fn get_from_devicename(_devicename: &String) -> Result<Device, String>{
        return get_device_by_devicename(_devicename)
    }
    pub fn get_from_email(_email: &String) -> Result<User, String>{
        return get_user_by_email(_email)
    }
    pub fn get_from_token(_token: &String) -> Result<Device, String>{
        return get_device_by_token(_token)
    }
}

fn get_user_by_id(_id: &u32) -> Result<User, String>{
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"SELECT
                                    id, security_pool, username, password, email, role, token FROM users
                                    WHERE id = :id ",
                           params!{
                               "id" => _id
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
        let (id, security_pool, username, password, email, _role, token): (u32, u32, String, String, String, String, String) = mysql::from_row(row);
        if &id == _id {
            let role: structures::roles::Roles = match _role.parse::<structures::roles::Roles>() {
                Ok(v) => v,
                Err(_e) => return Err("error while converting role".to_string())
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
        }
    }
    return Err("user not found".to_string())
}

fn get_user_by_username(_username: &String) -> Result<User, String>{
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"SELECT
                                    id, security_pool, username, password, email, role, token FROM users
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
        let (id, security_pool, username, password, email, _role, token): (u32, u32, String, String, String, String, String) = mysql::from_row(row);
        if username == _username.to_string() {
            let role: structures::roles::Roles = match _role.parse::<structures::roles::Roles>() {
                Ok(v) => v,
                Err(_e) => return Err("error while converting role".to_string())
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
        }
    }
    return Err("user not found".to_string())
}

fn get_user_by_email(_email: &String) -> Result<User, String>{
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"SELECT
                                    id, security_pool, username, password, email, role, token FROM users
                                    WHERE email = :email ",
                           params!{
                               "email" => _email
                            }
    );
    let result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err("failed to fetch user".to_string())
    };
    for _row in result {
        let row = match _row{
            Ok(v) => v,
            Err(_e) => panic!("let row = match _row")
        };
        let (id, security_pool, username, password, email, _role, token): (u32, u32, String, String, String, String, String) = mysql::from_row(row);
        if _email == &email {
            let role: structures::roles::Roles = match _role.parse::<structures::roles::Roles>() {
                Ok(v) => v,
                Err(_e) => return Err("error while converting role".to_string())
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
        }
    }
    return Err("user not found".to_string())
}

fn get_user_by_token(_token: &String) -> Result<User, String>{
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"SELECT
                                    id, security_pool, username, password, email, role, token FROM users
                                    WHERE token = :token ",
                           params!{
                               "token" => _token
                            }
    );
    let result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err("failed to fetch user".to_string())
    };
    for _row in result {
        let row = match _row{
            Ok(v) => v,
            Err(_e) => panic!("let row = match _row")
        };
        let (id, security_pool, username, password, email, _role, token): (u32, u32, String, String, String, String, String) = mysql::from_row(row);
        if _token == &token {
            let role: structures::roles::Roles = match _role.parse::<structures::roles::Roles>() {
                Ok(v) => v,
                Err(_e) => return Err("error while converting role".to_string())
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
        }
    }
    return Err("user not found".to_string())
}

fn user_login(_username: &String, _password: &String) -> Result<User, String> {
    if _password.len() <= 2 {
        return Err("Password to short".to_string())
    }
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

fn user_register(username: &String, _password: &String, email: &String, security_pool: &u32, role: &structures::roles::Roles) -> Result<(), String> {
    match get_user_by_username(&username){
        Ok(_v) => return Err("user already exists".to_string()),
        Err(_e) => _e
    };
    match get_user_by_email(&email){
        Ok(_v) => return Err("user-email already exists".to_string()),
        Err(_e) => _e
    };

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

// _user == user trying to register the device
fn device_register(_devicename: &String, _user: &structures::user::User) -> Result<(), String> {
    let security_pool: structures::security_pool::SecurityPool = match structures::security_pool::SecurityPool::get_security_pool_by_id(&_user.security_pool){
        Ok(_v) => _v,
        Err(_e) => return Err(format!("pool not found: {}", _e))
    };
    if security_pool.owner != _user.id {
        return Err("You are not the owner of this pool".to_string());
    }
    let devicename: String = format!("{:}_{:}", &security_pool.name, _devicename);
    let email: String = format!("{:}@{:}.device", _devicename, &security_pool.name);
    match get_user_by_username(&devicename){
        Ok(_v) => return Err("user already exists".to_string()),
        Err(_e) => _e
    };

    let password: String = "".to_string();
    let token: String = server::security::create_random_token();
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"INSERT INTO users
                                   (security_pool, username, password, email, role, token) VALUES
                                   (:security_pool, :username, :password, :email, :role, :token)",
                           params!{
                               "security_pool" => security_pool.id,
                               "username" => devicename,
                               "password" => password,
                               "email" => email,
                               "role" => structures::roles::Roles::Device.to_string(),
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
