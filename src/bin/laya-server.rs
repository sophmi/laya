use std::path::PathBuf;

#[path = "laya-server/telemetry.rs"]
mod telemetry;

fn main() {
    if let Err(e) = telemetry::install_telemetry() {
        eprintln!("Failed to install telemetry {:?}", e);
    }

    let mut path = PathBuf::new();
    path.push("./share");
    laya::start(path.into_boxed_path());
}
