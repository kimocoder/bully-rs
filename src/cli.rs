use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Bully-RS: WPS PIN Brute-force tool")]
pub struct Args {
    /// Wireless interface
    #[arg(index = 1)]
    pub interface: String,

    /// Target BSSID
    #[arg(short = 'b', long)]
    pub bssid: String,

    /// Target channel
    #[arg(short = 'c', long)]
    pub channel: Option<u8>,

    /// Use Pixie-Dust attack
    #[arg(short = 'd', long = "pixiewps")]
    pub pixie: bool,

    /// Specific PIN to test
    #[arg(short = 'p', long)]
    pub pin: Option<String>,

    /// Verbosity level
    #[arg(short = 'v', default_value = "3")]
    pub verbosity: u8,

    /// Sequential attack
    #[arg(short = 'S', long)]
    pub sequential: bool,

    /// Bruteforce attack
    #[arg(short = 'B', long)]
    pub bruteforce: bool,

    /// Force continue prior sessions
    #[arg(short = 'F', long)]
    pub force: bool,

    /// 5GHz support
    #[arg(short = '5', long)]
    pub five_ghz: bool,

    /// Ignore lock
    #[arg(short = 'L', long)]
    pub lock_ignore: bool,

    /// Detect lock
    #[arg(short = 'D', long)]
    pub detect_lock: bool,

    /// Lock wait time (seconds)
    #[arg(short = 'l', long, default_value = "43")]
    pub lock_wait: u32,

    /// Windows 7 mode
    #[arg(short = 'W', long)]
    pub windows_7: bool,

    /// Probe mode
    #[arg(short = 'P', long)]
    pub probe: bool,

    /// No ACKs
    #[arg(short = 'A', long)]
    pub no_acks: bool,

    /// No FCS
    #[arg(short = 'N', long)]
    pub no_fcs: bool,

    /// No check
    #[arg(short = 'C', long)]
    pub no_check: bool,

    /// EAP fail
    #[arg(short = 'E', long)]
    pub eap_fail: bool,

    /// Suppress output
    #[arg(short = 'Z', long)]
    pub suppress: bool,
}

impl Args {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }
}
