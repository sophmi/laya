#[path = "laya-server/telemetry.rs"]
mod telemetry;

fn main() {
    if let Err(e) = telemetry::install_telemetry() {
        eprintln!("Failed to install telemetry {:?}", e);        
    }
    laya::start();
}