use std::net::IpAddr;
use tracing::{info, warn, error, debug, trace};
use uuid::Uuid;


pub fn account_created(user_id: Uuid, ip: IpAddr) {
    info!(%user_id, %ip, "New Account Created");
}

pub fn fail_account_creation(ip: IpAddr) {
    info!(%ip, "Account Creation Failed");
}

pub fn login_attempted(user_id: Uuid, login_attempt: u16, ip: IpAddr) {
    match login_attempt {
        1 => info!(%user_id, %ip, "First Login Attempt"),
        2 => info!(%user_id, %ip, "Second Login Attempt"),
        3 => info!(%user_id, %ip, "Final Login Attempt"),
        _ => warn!(%user_id, %ip, "Potential Brute Force Detected")
    };
}

pub fn login_successful(user_id: Uuid, ip: IpAddr) {
    info!(%user_id, %ip, "Log In Successful");
}

pub fn unhashed_password(ip: IpAddr) {
    warn!(%ip, "Unhashed Password Received!");
}

pub fn fatal_error(ip: IpAddr) {
    error!(%ip, "Fatal error occured!");
}

pub fn call_debug() {
    debug!("Debug Called");
}

pub fn call_trace() {
    trace!("Trace Called");
}