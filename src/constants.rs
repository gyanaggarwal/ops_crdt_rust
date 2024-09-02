use dotenvy::dotenv;
use lazy_static::lazy_static;

pub mod env {
    pub const MAX_MSG_COUNT_VC_VAR: &str   = "MAX_MSG_COUNT_VC";
    pub const MAX_MSG_COUNT_CS_VAR: &str   = "MAX_MSG_COUNT_CS";
    pub const NODE_LIST_VAR: &str          = "NODE_LIST";
    pub const TEST_MSG_COUNT_VAR: &str     = "TEST_MSG_COUNT";
    pub const TEST_MSG_RANGE_PCT_VAR: &str = "TEST_MSG_RANGE_PCT";
    pub const TEST_MSG_RATE_PCT_VAR: &str  = "TEST_MSG_RATE_PCT";
    pub const TEST_SLEEP_TIME_MS_VAR: &str = "TEST_SLEEP_TIME_MS";
}

fn set_int_mode(param: &str) -> u64 {
    dotenv().ok();
    let value = std::env::var(param).unwrap_or("0".to_owned());
    parse_int(&value)
}

fn set_u16_mode(param: &str) -> u16 {
    set_int_mode(param) as u16
}

fn set_list_mode(param: &str) -> Vec<u16> {
    dotenv().ok();
    let value = std::env::var(param).unwrap_or("".to_owned());
    value.split(",").map(|x| parse_u16(x)).collect()
}

fn parse_int(s: &str) -> u64 {
    s.parse::<u64>().unwrap()
}

fn parse_u16(s: &str) -> u16 {
    parse_int(s) as u16
}

lazy_static! {
    pub static ref MAX_MSG_COUNT_VC: u16   = set_u16_mode(env::MAX_MSG_COUNT_VC_VAR);
    pub static ref MAX_MSG_COUNT_CS: u16   = set_u16_mode(env::MAX_MSG_COUNT_CS_VAR);
    pub static ref NODE_LIST: Vec<u16>     = set_list_mode(env::NODE_LIST_VAR);
    pub static ref TEST_MSG_COUNT: u16     = set_u16_mode(env::TEST_MSG_COUNT_VAR);
    pub static ref TEST_MSG_RANGE_PCT: u16 = set_u16_mode(env::TEST_MSG_RANGE_PCT_VAR);
    pub static ref TEST_MSG_RATE_PCT: u16  = set_u16_mode(env::TEST_MSG_RATE_PCT_VAR);
    pub static ref TEST_SLEEP_TIME_MS:u64  = set_int_mode(env::TEST_SLEEP_TIME_MS_VAR);
}