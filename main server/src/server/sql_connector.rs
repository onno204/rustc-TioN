extern crate mysql;
use mysql as my;
use crate::server;

pub fn get_sql_pool() -> my::Pool {
    return my::Pool::new(
        format!("mysql://{user}:{password}@{url}:{port}/{database}",
            user = server::settings_private::SQL_USER,
            password = server::settings_private::SQL_PASSWORD,
            url = server::settings_private::SQL_URL,
            port = server::settings_private::SQL_PORT,
            database = server::settings_private::SQL_DATABASE
    )).unwrap();
}
