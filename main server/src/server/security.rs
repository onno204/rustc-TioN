extern crate rand;
extern crate bcrypt;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use bcrypt::{DEFAULT_COST, hash, verify};

pub fn create_random_token() -> String {
    return thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .collect();
}

pub fn password_hash(password: String) -> String{
    return hash(password, DEFAULT_COST).unwrap();
}
pub fn password_verify(password: String, hash: String) -> bool {
    return verify(password, &hash).unwrap();
}
