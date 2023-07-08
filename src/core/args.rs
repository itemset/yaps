use clap::Parser;

/// Yet another port sniffing tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// IP address to scan open ports on
    #[arg(short, long)]
    pub ip_address: String,

    /// Number of threads to scan a given IP address on
    #[arg(short, long, default_value_t = 1)]
    pub threads: u16,

    /// Verbose logging
    #[arg(long, short, action)]
    pub verbose: bool,
}
