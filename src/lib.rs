pub use crate::network_status::NetworkStatus;
pub use flags::Flags;
pub use network_monitor::NetworkMonitor;
mod ffi;
mod flags;
mod network_monitor;
mod network_status;

pub trait NwCallback: Send + Sync {
    fn on_status(&self, cb: NetworkStatus);
}
