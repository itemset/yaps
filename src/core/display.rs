use std::net::IpAddr;

pub enum DisplayModes {
    INFO,
    SUCCESS,
    ERROR,
    DEBUG,
}

pub fn log(message: &str, mode: DisplayModes) {
    let color = match mode {
        DisplayModes::INFO => ("INFO", "\x1b[35m"),
        DisplayModes::SUCCESS => ("SUCCESS", "\x1b[32m"),
        DisplayModes::ERROR => ("ERROR", "\x1b[31m"),
        DisplayModes::DEBUG => ("DEBUG", "\x1b[34m"),
    };

    println!("{}[{}]\x1b[0m {}", color.1, color.0, message);
}

pub fn pretty_print_ports(ip_address: IpAddr, ports: Vec<u16>) {
    log("Available ports:", DisplayModes::INFO);

    for port in ports {
        println!("\t({}) {}:{}", port, ip_address, port);
    }
}
