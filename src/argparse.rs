use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    #[clap(short, long, default_value = "8080")]
    pub port: u16,

    #[clap(short, long = "bind-addr", value_names = &["ADDR"], default_value="127.0.0.1")]
    pub bind_address: String,

    #[clap(short, long = "root-dir", value_names = &["DIR"], default_value=".")]
    pub root_dir: PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
