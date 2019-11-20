extern crate mysql;
extern crate lazy_static;
use mysql as my;
use crate::server;
use std::sync::Mutex;
use std::vec::Vec;

lazy_static! {
    #[derive(Debug, Clone, )]
    pub static ref SQL_POOL: Mutex<Vec<my::Pool>> = Mutex::new(Vec::new());

}
impl SQL_POOL {
    pub fn get_conn(&self) -> Result<my::PooledConn, String> {
        let _lock: Vec<my::Pool> = match SQL_POOL.lock() {
            Ok(v) => v.to_vec(),
            Err(_e) => return Err("[sql.get_conn] locked".to_string())
        };
        if _lock.len() < 1{
            return Err("[sql.get_conn] no pools available".to_string())
        }
        let conn: my::PooledConn = match _lock[_lock.len()-1].get_conn() {
            Ok(v) => v,
            Err(_e) => return Err("[sql.get_conn] failed to get conn".to_string())
        };
        Ok(conn)
    }
}

pub fn check_sql_pool() -> Result<(), String> {
    // Check if a mysqlpool (which manages connections) exists or not
    if SQL_POOL.lock().unwrap().len() < 1 {
        let _pool = my::Pool::new(
            format!("mysql://{user}:{password}@{url}:{port}/{database}",
                user = server::settings_private::SQL_USER,
                password = server::settings_private::SQL_PASSWORD,
                url = server::settings_private::SQL_URL,
                port = server::settings_private::SQL_PORT,
                database = server::settings_private::SQL_DATABASE
        ));
        let pool: my::Pool = match _pool {
            Ok(v) => v,
            Err(_e) => return Err("Mysql could not connect".to_string())
        };
        SQL_POOL.lock().unwrap().push(pool);
    }
    return Ok(())
}
