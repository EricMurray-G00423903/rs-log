use std::error::Error;
use tracing_subscriber::FmtSubscriber;
use tracing::{info, subscriber:: set_global_default, Level};

pub fn init_logger(env: &str) -> Result<(), Box<dyn Error>> {

    let log_level = match env {
        "prod" => Level::INFO,
        "dev" => Level::DEBUG,
        _ => return Err("Invalid Environment".into()),
    };
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();

    set_global_default(subscriber)?;

    info!("Logger initialised at {log_level:?} level");

    Ok(())

}