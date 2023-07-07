use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

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
        println!("Scanning {} on {} threads", self.ip_address, self.threads);
        let port_ranges = self.generate_port_ranges();

        // logic
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

    fn scan_range(&self, tx: Sender<u16>, range: (u16, u16)) {
        let (start_port, end_port) = range;
        for port in start_port..=end_port {
            if self.scan_port(port) {
                println!("Port {} is open", port);

                continue;
            }

            println!("Port {} is not open", port);
        }
    }

    fn scan_port(&self, port: u16) -> bool {
        match TcpStream::connect_timeout(
            &SocketAddr::new(self.ip_address, port),
            std::time::Duration::from_secs(1),
        ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
