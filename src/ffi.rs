use crate::network_status::NetworkStatus;

unsafe extern "C" {
    ///Starts a static network monitor singleton
    pub(crate) fn start_network_monitor(cb: unsafe extern "C" fn(NetworkStatus));
}

#[cfg(test)]
mod tests {
    use crate::{flags::Flags, network_status::PathStatus};

    use super::*;

    unsafe extern "C" {
        fn status_satisfied_ipv4_ipv6() -> NetworkStatus;
        fn status_unsatisfied_none() -> NetworkStatus;
        fn status_satisfiable_expensive_dns() -> NetworkStatus;
        fn status_invalid_all_flags() -> NetworkStatus;
    }

    #[test]
    fn test_satisfied_ipv4_ipv6() {
        let s = unsafe { status_satisfied_ipv4_ipv6() };

        assert_eq!(s.path_status, PathStatus::Satisfied);
        assert!(s.get_flag::<{ Flags::IPV4 }>());
        assert!(s.get_flag::<{ Flags::IPV6 }>());
        assert!(!s.get_flag::<{ Flags::DNS }>());
    }

    #[test]
    fn test_unsatisfied_none() {
        let s = unsafe { status_unsatisfied_none() };

        assert_eq!(s.path_status, PathStatus::Unsatisfied);
        assert_eq!(s.flags, 0);
    }

    #[test]
    fn test_satisfiable_expensive_dns() {
        let s = unsafe { status_satisfiable_expensive_dns() };

        assert_eq!(s.path_status, PathStatus::Satisfiable);
        assert!(s.get_flag::<{ Flags::EXPENSIVE }>());
        assert!(s.get_flag::<{ Flags::DNS }>());
    }

    #[test]
    fn test_invalid_all_flags() {
        let s = unsafe { status_invalid_all_flags() };

        assert_eq!(s.path_status, PathStatus::Invalid);
        assert!(s.get_flag::<{ Flags::IPV4 }>());
        assert!(s.get_flag::<{ Flags::CONSTRAINED }>());
    }
}
