use std::sync::OnceLock;

use crate::{NwCallback, network_status::NetworkStatus};

static CALLBACK: OnceLock<Box<dyn NwCallback>> = OnceLock::new();
pub struct NetworkMonitor;

impl NetworkMonitor {
    pub fn start(handler: Box<dyn NwCallback>) {
        if let Ok(()) = CALLBACK.set(handler) {
            unsafe { crate::ffi::start_network_monitor(callback_shim) };
        }
    }
}
unsafe extern "C" fn callback_shim(status: NetworkStatus) {
    if let Some(cb) = CALLBACK.get() {
        cb.on_status(status);
    }
}
