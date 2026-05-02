#import "network_monitor.h"

// Base constructor
inline NetworkStatus make_status(PathStatus status, uint8_t flags) {
    NetworkStatus s;
    s.status = status;
    s.flags = flags;
    return s;
}


inline NetworkStatus status_satisfied_ipv4_ipv6(void) {
    return make_status(PATH_STATUS_SATISFIED, FLAG_IPV4 | FLAG_IPV6);
}

inline NetworkStatus status_unsatisfied_none(void) {
    return make_status(PATH_STATUS_UNSATISFIED, 0);
}

inline NetworkStatus status_satisfiable_expensive_dns(void) {
    return make_status(PATH_STATUS_SATISFIABLE, FLAG_EXPENSIVE | FLAG_DNS);
}

inline NetworkStatus status_invalid_all_flags(void) {
    return make_status(
        PATH_STATUS_INVALID,
        FLAG_IPV4 | FLAG_IPV6 | FLAG_EXPENSIVE | FLAG_CONSTRAINED | FLAG_DNS
    );
}
