use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref RIOT_BASE: String = {
        env::var("RIOT_URL_BASE")
            .expect("the RIOT_URL_BASE environment variable should not be empty")
    };
    pub static ref RIOT_API_KEY: String =
        env::var("RIOT_API").expect("the RIOT_API environment variable should not be empty");
    pub static ref SOCKET: String =
        env::var("SOCKET").expect("the SOCKET environment variable should not be empty");
}
