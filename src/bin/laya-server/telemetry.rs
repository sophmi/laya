use std::error::Error;

pub fn install_telemetry() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        // enable everything
        .with_max_level(tracing::Level::INFO)
        // sets this to be the default, global collector for this application.
        .init();

    Ok(())
}
