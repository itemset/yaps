mod args;
mod scanner;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    let s = scanner::IpScanner::new(args.ip_address, args.threads);

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
