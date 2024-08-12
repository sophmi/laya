use std::path::Path;

use clap::Parser;

#[path = "laya-server/telemetry.rs"]
mod telemetry;

fn main() {
    let args = Args::parse();
    if !args.path.exists() {
        panic!("Could not resolve image directory '{}'.", args.path.to_string_lossy())
    }

    if let Err(e) = telemetry::install_telemetry() {
        eprintln!("Failed to install telemetry {:?}", e);
    }

    laya::start(args.path);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_missing_value = "./share/")]
    path: Box<Path>,
}
