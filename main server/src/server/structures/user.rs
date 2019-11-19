extern crate mysql;
use crate::server;
use crate::server::structures;
use std::fmt;

#[derive(Debug)]
pub struct User {
    id: u32,
    security_pool: u32,
    username: String,
    password: String,
    email: String,
    role: structures::roles::Roles,
    token: String
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
    pub fn new(username: String, password: String, email: String, security_pool: u32, role: structures::roles::Roles) -> User {
        let user: User = User {
            id: 0,
            security_pool: security_pool,
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            role: role,
            token: "".to_string()
        };
        user_register(user.username.to_string(), user.password.to_string(), user.email.to_string(), user.security_pool, user.role);
        return user;
    }

}

pub fn user_register(username: String, mut password: String, email: String, security_pool: u32, role: structures::roles::Roles ) {
    let token: String = server::security::create_random_token();
    password = server::security::password_hash(password);
    let pool: mysql::Pool = server::sql_connector::get_sql_pool();
    let mut _result: Vec<User> = pool.prep_exec(r"INSERT INTO users
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
    ).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, security_pool, username, password, email, role, token): (u32, u32, String, String, String, String, String) = mysql::from_row(row);
            User {
                id: id,
                security_pool: security_pool,
                username: username,
                password: password,
                email: email,
                role: role.parse::<structures::roles::Roles>().unwrap(),
                token: token
            }
        }).collect()
    }).unwrap();
    // println!("{:?}", _result[0]);
}
