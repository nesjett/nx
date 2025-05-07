use tracing::{error, info};
use crate::native::logger::enable_logger;

#[napi]
pub fn info(message: String) {
    enable_logger();
    info!(message);
}

#[napi]
pub fn error(message: String) {
    enable_logger();
    error!(message);
}
