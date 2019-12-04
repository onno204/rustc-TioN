extern crate mysql;
use crate::server;
use crate::server::structures;
use std::fmt;

#[derive(Debug)]
pub struct SecurityPool {
    pub id: u32,
    pub name: String,
    pub owner: u32
}

impl fmt::Display for SecurityPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {id}, name: {name}, owner: {owner}",
            id = self.id,
            name = self.name,
            owner = self.owner
        )
    }
}

impl SecurityPool {
    pub fn new(name: &String, owner: &u32) -> Result<SecurityPool, String> {
        match add_security_pool(name, owner) {
            Ok(v) => v,
            Err(_e) => return Err(format!("failed to add pool: {}", _e).to_string())
        };
        let pool: SecurityPool = match get_security_pool_by_name(name) {
            Ok(v) => v,
            Err(_e) => return Err(format!("failed find pool: {}", _e).to_string())
        };
        return Ok(pool)
    }
    pub fn get_security_pool_by_name(_pool_name: &String) -> Result<SecurityPool, String>{
        return get_security_pool_by_name(_pool_name)
    }
    pub fn get_security_pool_by_id(_pool_id: &u32) -> Result<SecurityPool, String>{
        return get_security_pool_by_id(_pool_id)
    }
}

fn get_security_pool_by_name(_pool_name: &String) -> Result<SecurityPool, String>{
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"SELECT
                                    id, name, owner FROM security_pool
                                    WHERE name = :name ",
                           params!{
                               "name" => _pool_name
                            }
    );
    let result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err("failed to fetch pool".to_string())
    };
    for _row in result {
        let row = match _row{
            Ok(v) => v,
            Err(_e) => panic!("[user_login] let row = match _row")
        };
        let (id, name, owner): (u32, String, u32) = mysql::from_row(row);
        if &name == _pool_name {
            let pool: SecurityPool = SecurityPool {
                id: id,
                name: name.to_string(),
                owner: owner
            };
            return Ok(pool)
        }
    }
    return Err("pool not found".to_string())
}
fn get_security_pool_by_id(_pool_id: &u32) -> Result<SecurityPool, String>{
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"SELECT
                                    id, name, owner FROM security_pool
                                    WHERE id = :id ",
                           params!{
                               "id" => _pool_id
                            }
    );
    let result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err("failed to fetch pool".to_string())
    };
    for _row in result {
        let row = match _row{
            Ok(v) => v,
            Err(_e) => panic!("[user_login] let row = match _row")
        };
        let (id, name, owner): (u32, String, u32) = mysql::from_row(row);
        if &id == _pool_id {
            let pool: SecurityPool = SecurityPool {
                id: id,
                name: name.to_string(),
                owner: owner
            };
            return Ok(pool)
        }
    }
    return Err("pool not found".to_string())
}

fn add_security_pool(name: &String, owner: &u32) -> Result<(), String> {
    match get_security_pool_by_name(&name){
        Ok(_v) => return Err("pool already exists".to_string()),
        Err(_e) => _e
    };
    let _user: structures::user::User = match structures::user::User::get_from_id(owner) {
        Ok(v) => v,
        Err(_e) => return Err("owner id not found".to_string())
    };
    let mut conn: mysql::PooledConn = server::sql_connector::SQL_POOL.get_conn().unwrap();
    let _result: Result<mysql::QueryResult<'_>, mysql::Error> = conn.prep_exec(r"INSERT INTO security_pool
                                   (name, owner) VALUES
                                   (:name, :owner)",
                           params!{
                               "name" => name,
                               "owner" => owner
                            }
    );
    let _result: mysql::QueryResult<'_> = match _result{
        Ok(v) => v,
        Err(_e) => return Err(format!("{}", _e))
    };
    if _result.affected_rows() < 1 {
        return Err(format!("unknown error: no pool created"))
    }
    return Ok(())
}
