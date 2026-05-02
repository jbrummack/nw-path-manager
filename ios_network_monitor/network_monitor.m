#import "network_monitor.h"
#import <Network/Network.h>

static nw_path_monitor_t monitor;

NetworkStatus translate_nw_path_t_to_status(nw_path_t path) {
    //INIT
    nw_path_status_t status = nw_path_get_status(path);
    uint8_t flags = 0;
    enum PathStatus value = PATH_STATUS_INVALID;
    //EXTRACT
    switch (status) {
            case nw_path_status_satisfied: value = PATH_STATUS_SATISFIED; break;
            case nw_path_status_unsatisfied: value = PATH_STATUS_UNSATISFIED; break;
            case nw_path_status_satisfiable: value = PATH_STATUS_SATISFIABLE; break;
            case nw_path_status_invalid: value = PATH_STATUS_INVALID; break;
    }
    if (nw_path_has_ipv4(path)) {flags |= FLAG_IPV4;}
    if (nw_path_has_ipv6(path)) {flags |= FLAG_IPV6;}
    if (nw_path_is_expensive(path)) {flags |= FLAG_EXPENSIVE;}
    if (nw_path_is_constrained(path)) {flags |= FLAG_CONSTRAINED;}
    if (nw_path_has_dns(path)) {flags |= FLAG_DNS;}
    //RETURN
    struct NetworkStatus result = {value, flags};
    return result;
}
void start_network_monitor(network_callback_t cb) {
    monitor = nw_path_monitor_create();

    nw_path_monitor_set_update_handler(monitor, ^(nw_path_t path) {
        struct NetworkStatus value = translate_nw_path_t_to_status(path);
        cb(value);
    });

    dispatch_queue_t queue = dispatch_queue_create("net.monitor", DISPATCH_QUEUE_SERIAL);
    nw_path_monitor_set_queue(monitor, queue);
    nw_path_monitor_start(monitor);
}

void stop_network_monitor(void) {
    if (monitor) {
        nw_path_monitor_cancel(monitor);
        monitor = NULL;
    }
}
