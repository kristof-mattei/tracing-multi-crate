use tracing::{info, trace};

pub fn work() {
    info!("crate-a");
    trace!("crate-a");
}
