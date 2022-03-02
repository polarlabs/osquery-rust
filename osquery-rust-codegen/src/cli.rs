#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None, global_setting = clap::AppSettings::DeriveDisplayOrder)]
#[clap(arg_required_else_help = true)]
#[clap(global_setting(clap::AppSettings::DeriveDisplayOrder))]
#[clap(group(
  clap::ArgGroup::new("mode")
    .required(true)
    .multiple(false)
    .args(&["standalone", "socket"]),
))]
#[clap(group(
  clap::ArgGroup::new("mode::socket")
    .required(false)
    .multiple(true)
    .conflicts_with("standalone")
    .args(&["interval", "timeout"]),
))]
pub struct Args {
    // Operating in standalone mode
    #[clap(long)]
    standalone: bool,

    // Operating in socket mode
    #[clap(long, value_name = "PATH_TO_SOCKET")]
    socket: Option<String>,

    /// Delay in seconds between connectivity checks.
    #[clap(long, default_value_t = 30)]
    interval: u32,

    /// Time in seconds to wait for autoloaded extensions until connection times out.
    #[clap(long, default_value_t = 30)]
    timeout: u32,

    /// Enable verbose informational messages.
    #[clap(long)]
    verbose: bool,
}

impl Args {
    pub fn standalone(&self) -> bool {
        self.standalone
    }

    pub fn socket(&self) -> Option<String> {
        self.socket.clone()
    }
}
