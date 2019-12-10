extern crate mysql;
use crate::server;
use crate::server::structures;
use std::fmt;

#[derive(Debug)]
pub struct Device {
    pub id: u32,
    pub security_pool: u32,
    pub devicename: String,
    pub device_type: structures::device_types::DeviceTypes,
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
    pub fn new(_devicename: &String, _device_type: &structures::device_types::DeviceTypes, _room: &String, _user: &structures::user::User) -> Result<Device, String> {
        match device_register(_devicename, _device_type, _room, _user) {
            Ok(v) => v,
            Err(_e) => return Err(format!("failed to register Device: {}", _e).to_string())
        };
        let device: Device = match get_device_by_devicename(_devicename, &_user.security_pool) {
            Ok(v) => v,
            Err(_e) => return Err(format!("failed to login Device: {}", _e).to_string())
        };
        return Ok(device)
    }
    pub fn get_from_id(_id: &u32) -> Result<Device, String>{
        return get_device_by_id(_id)
    }
    pub fn get_from_devicename(_devicename: &String, _security_pool: &u32) -> Result<Device, String>{
        return get_device_by_devicename(_devicename, _security_pool)
    }
    pub fn get_from_token(_token: &String) -> Result<Device, String>{
        return get_device_by_token(_token)
    }
}

fn get_device_by_devicename(_devicename: &String, _security_pool: &u32) -> Result<Device, String>{
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"SELECT
                                    id, security_pool, devicename, device_type, room, token FROM devices
                                    WHERE devicename = :devicename AND security_pool = :security_pool  ",
                           params!{
                               "devicename" => _devicename,
                               "security_pool" => _security_pool
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
        if &devicename == _devicename && &security_pool == _security_pool {
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
fn get_device_by_id(_id: &u32) -> Result<Device, String>{
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
        let (id, security_pool, devicename, _device_type, room, token): (u32, u32, String, String, String, String) = mysql::from_row(row);
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
fn get_device_by_token(_token: &String) -> Result<Device, String>{
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

// _user == user trying to register the device
fn device_register(_devicename: &String, _device_type: &structures::device_types::DeviceTypes, _room: &String, _user: &structures::user::User) -> Result<(), String> {
    let security_pool: structures::security_pool::SecurityPool = match structures::security_pool::SecurityPool::get_security_pool_by_id(&_user.security_pool){
        Ok(_v) => _v,
        Err(_e) => return Err(format!("pool not found: {}", _e))
    };
    if security_pool.owner != _user.id {
        return Err("You are not the owner of this pool".to_string());
    }
    let devicename: String = format!("{:}_{:}", &security_pool.name, _devicename);
    match get_device_by_devicename(&devicename, &security_pool.id){
        Ok(_v) => return Err("device already exists".to_string()),
        Err(_e) => _e
    };

    let token: String = server::security::create_random_token();
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"INSERT INTO devices
                                   (security_pool, devicename, device_type, room, token) VALUES
                                   (:security_pool, :devicename, :device_type, :email, :role, :token)",
                           params!{
                               "security_pool" => security_pool.id,
                               "devicename" => devicename,
                               "device_type" => _device_type.to_string(),
                               "room" => _room,
                               "token" => token
                            }
    );
    let _result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err(format!("{}", _e))
    };
    if _result.affected_rows() < 1 {
        return Err(format!("unknown error: no device created"))
    }
    return Ok(())
}
