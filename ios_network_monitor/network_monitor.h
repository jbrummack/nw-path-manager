#pragma once

#ifdef __cplusplus
extern "C" {
#endif
#include <stdint.h>


//Flags
#define FLAG_IPV4 (1u << 0)
#define FLAG_IPV6 (1u << 1)
#define FLAG_EXPENSIVE (1u << 2)
#define FLAG_CONSTRAINED (1u << 3)
#define FLAG_DNS (1u << 4)

//Types
typedef enum PathStatus {
    PATH_STATUS_SATISFIED,
    PATH_STATUS_UNSATISFIED,
    PATH_STATUS_SATISFIABLE,
    PATH_STATUS_INVALID
} PathStatus;
typedef struct NetworkStatus {
    PathStatus status;
    uint8_t flags;
} NetworkStatus;

//Exposed methods
typedef void (*network_callback_t)(NetworkStatus status);

void start_network_monitor(network_callback_t cb);
void stop_network_monitor(void);

//Testing
NetworkStatus status_satisfied_ipv4_ipv6(void);
NetworkStatus status_unsatisfied_none(void);
NetworkStatus status_satisfiable_expensive_dns(void);
NetworkStatus status_invalid_all_flags(void);
NetworkStatus make_status(PathStatus status, uint8_t flags);

#ifdef __cplusplus
}
#endif
