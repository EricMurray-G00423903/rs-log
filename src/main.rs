use logger::init_logger;
use logger::log_events;
use logger::log_events::*;
fn main() {
    init_logger("dev").unwrap();

    let ip: std::net::IpAddr = "192.168.0.1".parse().unwrap();
    let user_id = uuid::Uuid::max();

    log_events::account_created(user_id, ip);
    log_events::fail_account_creation(ip);
    log_events::login_attempted(user_id, 4, ip);
    log_events::login_successful(user_id, ip);
    log_events::unhashed_password(ip);
    log_events::fatal_error(ip);
    log_events::call_debug();
    log_events::call_trace();
}
