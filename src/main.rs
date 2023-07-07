mod core;

use clap::Parser;

fn main() {
    let args = core::args::Args::parse();

    let s = core::scanner::IpScanner::new(args.ip_address, args.threads);

    match s {
        Some(scan) => {
            println!(r#"
             _ _   ___   ___   ___ 
            | | |_| .'|_| . |_|_ -|
            |_  |_|__,|_|  _|_|___|
            |___|       |_|        
                            yet another port sniffer
            "#);
            scan.scan();
        }
        None => println!("failed to initialize...")
    }
}
