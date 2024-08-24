use dotenvy::dotenv;
use lazy_static::lazy_static;

pub mod env {
    pub const MAX_MSG_COUNT_VC_VAR: &str = "MAX_MSG_COUNT_VC";
    pub const MAX_MSG_COUNT_CS_VAR: &str = "MAX_MSG_COUNT_CS";
    pub const NODE_LIST_VAR: &str        = "NODE_LIST";
}

fn set_int_mode(param: &str) -> u16 {
    dotenv().ok();
    let value = std::env::var(param).unwrap_or("0".to_owned());
    parse_int(&value)
}

fn set_list_mode(param: &str) -> Vec<u16> {
    dotenv().ok();
    let value = std::env::var(param).unwrap_or("".to_owned());
    value.split(",").map(|x| parse_int(x)).collect()
}

fn parse_int(s: &str) -> u16 {
    s.parse::<u16>().unwrap()
}

lazy_static! {
    pub static ref MAX_MSG_COUNT_VC: u16 = set_int_mode(env::MAX_MSG_COUNT_VC_VAR);
    pub static ref MAX_MSG_COUNT_CS: u16 = set_int_mode(env::MAX_MSG_COUNT_CS_VAR);
    pub static ref NODE_LIST: Vec<u16>   = set_list_mode(env::NODE_LIST_VAR);
}