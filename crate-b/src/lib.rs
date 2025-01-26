use tracing::{info, trace};

pub fn work() {
    info!("crate-b");
    trace!("crate-b");
}
