use crate::core::display;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};

pub struct IpScanner {
    ip_address: IpAddr,
    threads: u16,
}

impl IpScanner {
    const MAX_PORT: u16 = u16::MAX; // 65,535

    pub fn new(address: String, threads: u16) -> Option<IpScanner> {
        if let Ok(ip_address) = IpAddr::from_str(&address) {
            return Some(IpScanner {
                ip_address,
                threads,
            });
        }

        None
    }

    pub fn scan(&self) {
        display::log(
            &format!("Scanning {} on {} threads", self.ip_address, self.threads),
            display::DisplayModes::INFO,
        );

        let port_ranges = self.generate_port_ranges();
        let (tx, rx) = channel::<u16>();
        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for range in port_ranges {
            let tx = tx.clone();
            let ip_address = self.ip_address.clone();

            let handle = thread::spawn(move || IpScanner::scan_range(ip_address, range, tx));
            handles.push(handle);
        }

        drop(tx);
        let _ = handles.into_iter().map(|handle| handle.join().unwrap());
        let ports: Vec<u16> = rx.into_iter().collect();

        display::pretty_print_ports(self.ip_address, ports);
    }

    fn generate_port_ranges(&self) -> Vec<(u16, u16)> {
        let ports_per_thread = IpScanner::MAX_PORT / self.threads;
        let mut port_ranges = Vec::with_capacity(self.threads as usize);

        for i in 0..self.threads {
            let start_port = i * ports_per_thread + 1;
            let end_port = if i == self.threads - 1 {
                IpScanner::MAX_PORT
            } else {
                (i + 1) * ports_per_thread
            };

            port_ranges.push((start_port, end_port));
        }

        port_ranges
    }

    fn scan_range(ip_address: IpAddr, range: (u16, u16), transmitter: Sender<u16>) {
        let (start_port, end_port) = range;
        for port in start_port..=end_port {
            let message = if IpScanner::scan_port(ip_address, port) {
                transmitter.send(port).unwrap();
                let message = format!("Port {} is open", port);
                message
            } else {
                let message = format!("Port {} is not open", port);
                message
            };

            display::log(message.as_str(), display::DisplayModes::DEBUG);
        }
    }

    fn scan_port(ip_address: IpAddr, port: u16) -> bool {
        match TcpStream::connect_timeout(
            &SocketAddr::new(ip_address, port),
            std::time::Duration::from_secs(1),
        ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
