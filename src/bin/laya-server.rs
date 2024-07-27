#[path = "laya-server/telemetry.rs"]
mod telemetry;

fn main() {
    telemetry::install_telemetry().expect("failed to install telemetry");
    laya::start();
}