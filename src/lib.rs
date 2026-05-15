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

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;
    struct CallbackDummy(mpsc::Sender<NetworkStatus>);
    impl NwCallback for CallbackDummy {
        fn on_status(&self, cb: NetworkStatus) {
            let _ = self.0.send(cb);
        }
    }

    #[test]
    fn test_operation() {
        let (sender, receiver) = mpsc::channel();
        let callback = CallbackDummy(sender);
        NetworkMonitor::start(Box::new(callback));
        while let Ok(msg) = receiver.recv() {
            println!("{msg:?}");
            break;
        }
    }
}
