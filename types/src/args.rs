use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
    /// Port to listen on for P2P communication
    #[clap(short, long)]
    pub port: Option<u16>,

    /// Initial peer to connect to
    #[clap(short, long)]
    pub peer: Option<String>,

    /// Database file name
    #[clap(short, long)]
    pub database: Option<String>,

    /// Custom name for HTTP access logs
    #[clap(short, long)]
    pub name: Option<String>,

    /// Hostname to use for the server (defaults to external IP)
    #[clap(short = 'H', long)]
    pub hostname: Option<String>,

    /// Path to config file
    #[clap(short, long)]
    pub config: Option<String>,

    /// No-IP hostname (e.g., example.ddns.net)
    #[clap(long)]
    pub noip_hostname: Option<String>,

    /// No-IP username
    #[clap(long)]
    pub noip_username: Option<String>,

    /// No-IP password
    #[clap(long)]
    pub noip_password: Option<String>,

    /// OpenDNS hostname
    #[clap(long)]
    pub opendns_hostname: Option<String>,

    /// OpenDNS username
    #[clap(long)]
    pub opendns_username: Option<String>,

    /// OpenDNS password
    #[clap(long)]
    pub opendns_password: Option<String>,

    /// OpenDNS network label
    #[clap(long)]
    pub opendns_network: Option<String>,

    /// Enable post-apocalyptic survival mode
    #[clap(long)]
    pub survival_mode: Option<bool>,

    /// Log level (trace, debug, info, warn, error)
    #[clap(short = 'L', long, default_value = "info")]
    pub log_level: Option<String>,
}
