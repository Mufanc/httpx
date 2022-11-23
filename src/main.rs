use std::env;
use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;

mod argparse;

#[tokio::main]
async fn main() {
    let args = argparse::parse();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    let address = SocketAddr::V4(
        SocketAddrV4::from_str(&format!("{}:{}", args.bind_address, args.port))
            .expect("failed to parse address"),
    );

    warp::serve(warp::fs::dir(args.root_dir)).run(address).await;
}
