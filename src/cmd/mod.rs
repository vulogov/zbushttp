extern crate log;

use crate::stdlib::hostname;

use clap::{Args,Parser, Subcommand};
use std::env;
use std::fmt::Debug;
use crate::stdlib;

pub mod setloglevel;
pub mod zabbix_lib;
pub mod zbus_version;

pub fn init() {
    log::debug!("Parsing CLI parameters");
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    stdlib::initlib(&cli);

    match &cli.command {
        Commands::Version(_) => {
            log::debug!("Get the tool version");
            zbus_version::run(&cli);
        }
    }
}

#[derive(Parser, Clone)]
#[clap(name = "zbushttp")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "ZBUS federated observability HTTP interface", long_about = None)]
pub struct Cli {
    #[clap(short, long, action = clap::ArgAction::Count, help="Increase verbosity")]
    pub debug: u8,

    #[clap(help="ZBUS HTTP gateway location", long, default_value_t = String::from(hostname::get_hostname()))]
    pub location: String,

    #[clap(help="Zabbix API endpoint", long, default_value_t = String::from("http://127.0.0.1:8080"))]
    pub zabbix_api: String,

    #[clap(long, default_value_t = 16, help="Number of threads in ThreadManager")]
    pub threads: u16,

    #[clap(long, default_value_t = 3600, help="Timeout for Zabbix ITEMS cache")]
    pub item_cache_timeout: u16,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Get the version of the tool")]
pub struct Version {
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Version(Version),
}
