use clap::Parser;

/// Command line arguments for the P2P network application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Port to listen on for P2P communication (defaults to random port between 10000-65535)
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Initial peer to connect to
    #[arg(short('e'), long)]
    pub peer: Option<String>,

    /// Database file name (defaults to p2p_network.db)
    #[arg(short, long)]
    pub database: Option<String>,

    /// Custom name for HTTP access logs
    #[arg(short, long)]
    pub name: Option<String>,

    /// Hostname to use for the server (defaults to external IP)
    #[arg(short('H'), long)]
    pub hostname: Option<String>,

    /// Path to YAML configuration file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Enable post-apocalyptic survival mode
    #[arg(long)]
    pub survival_mode: Option<bool>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short('l'), long, default_value = "info")]
    pub log_level: Option<String>,

    /// No-IP hostname (e.g., example.ddns.net)
    #[arg(long)]
    pub noip_hostname: Option<String>,

    /// No-IP username
    #[arg(long)]
    pub noip_username: Option<String>,

    /// No-IP password
    #[arg(long)]
    pub noip_password: Option<String>,

    /// OpenDNS hostname
    #[arg(long)]
    pub opendns_hostname: Option<String>,

    /// OpenDNS username
    #[arg(long)]
    pub opendns_username: Option<String>,

    /// OpenDNS password
    #[arg(long)]
    pub opendns_password: Option<String>,

    /// OpenDNS network label
    #[arg(long)]
    pub opendns_network: Option<String>,
}
