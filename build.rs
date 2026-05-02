fn main() {
    build_ios();
}

fn build_ios() {
    println!("cargo:rerun-if-changed=ios_network_monitor/ffi_test.c");
    println!("cargo:rerun-if-changed=ios_network_monitor/network_monitor.m");
    println!("cargo:rerun-if-changed=ios_network_monitor/network_monitor.h");

    let target = std::env::var("TARGET").unwrap();

    if !target.contains("apple") {
        return;
    }

    let mut build = cc::Build::new();

    build
        .file("ios_network_monitor/network_monitor.m")
        .file("ios_network_monitor/ffi_test.c")
        .include("native")
        .flag("-fobjc-arc")
        .compile("network_monitor");

    println!("cargo:rustc-link-lib=framework=Network");
    println!("cargo:rustc-link-lib=framework=Foundation");

    println!("cargo:rustc-link-lib=objc");
}
