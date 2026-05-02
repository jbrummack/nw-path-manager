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
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
