extern crate mysql;
use crate::server;
use crate::server::structures;
use std::fmt;

#[derive(Debug)]
pub struct Device {
    pub id: u32,
    pub security_pool: u32,
    pub devicename: String,
    pub device_type: DeviceTypes,
    pub room: String,
    pub token: String
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {id}, security_pool: {security_pool}, devicename: {devicename}, device_type: {device_type}, room: {room}, token: {token}",
            id = self.id,
            security_pool = self.security_pool,
            devicename = self.devicename,
            device_type = self.device_type.to_string(),
            room = self.room,
            token = self.token
        )
    }
}

impl Device {
    pub fn new(_devicename: &String, _user: &structures::user::User) -> Result<Device, String> {
        match device_register(_devicename, _user) {
            Ok(v) => v,
            Err(_e) => return Err(format!("failed to register Device: {}", _e).to_string())
        };
        let device: Device = match user_login(username, password) {
            Ok(v) => v,
            Err(_e) => return Err(format!("failed to login Device: {}", _e).to_string())
        };
        return Ok(device)
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

fn get_device_by_id(_id: &u32) -> Result<User, String>{
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"SELECT
                                    id, security_pool, devicename, device_type, room, token FROM devices
                                    WHERE id = :id ",
                           params!{
                               "id" => _id
                            }
    );
    let result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err("failed to fetch device".to_string())
    };
    for _row in result {
        let row = match _row{
            Ok(v) => v,
            Err(_e) => panic!("[get_device_by_id] let row = match _row")
        };
        let (id, security_pool, devicename, _device_type, room, token): (u32, u32, String, String, String, String, String) = mysql::from_row(row);
        if &id == _id {
            let device_type: structures::device_types::DeviceTypes = match _device_type.parse::<structures::device_types::DeviceTypes>() {
                Ok(v) => v,
                Err(_e) => return Err("error while converting device_type".to_string())
            };
            let device: Device = Device {
                id: id,
                security_pool: security_pool,
                devicename: devicename.to_string(),
                device_type: device_type,
                room: room.to_string(),
                token: token.to_string()
            };
            return Ok(device)
        }
    }
    return Err("device not found".to_string())
}
fn get_device_by_token(_token: &String) -> Result<User, String>{
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"SELECT
                                    id, security_pool, devicename, device_type, room, token FROM devices
                                    WHERE token = :token ",
                           params!{
                               "token" => _token
                            }
    );
    let result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err("failed to fetch device".to_string())
    };
    for _row in result {
        let row = match _row{
            Ok(v) => v,
            Err(_e) => panic!("[get_device_by_id] let row = match _row")
        };
        let (id, security_pool, devicename, _device_type, room, token): (u32, u32, String, String, String, String) = mysql::from_row(row);
        if &token == _token {
            let device_type: structures::device_types::DeviceTypes = match _device_type.parse::<structures::device_types::DeviceTypes>() {
                Ok(v) => v,
                Err(_e) => return Err("error while converting device_type".to_string())
            };
            let device: Device = Device {
                id: id,
                security_pool: security_pool,
                devicename: devicename.to_string(),
                device_type: device_type,
                room: room.to_string(),
                token: token.to_string()
            };
            return Ok(device)
        }
    }
    return Err("device not found".to_string())
}


fn update_token(id: &u32) -> Result<String, String> {
    let token: String = server::security::create_random_token();
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"UPDATE devices SET
                                    token = :token
                                    WHERE id = :id ",
                           params!{
                               "id" => id,
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
