use log::{info, warn, debug, error, Level, log_enabled, trace};
use std::env;

fn main() {
    // Log levels are controlled on a per-module basis,
    // and by default all logging is disabled except for the error level.
    // error, warn, info, debug, trace
    // RUST_LOG=trace ./log_demo

    env_logger::init();
    let rust_log_level = env::var("RUST_LOG");
    println!("log_level = {:?}", rust_log_level);

    error!("this is printed by default");
    warn!("this is a warn {}", "message");
    debug!("this is a debug {}", "message");
    trace!("this is a trace {}", "message");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}

