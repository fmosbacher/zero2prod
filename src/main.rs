use std::{error::Error, net::TcpListener};

use zero2prod::settings::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new();
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    zero2prod::startup::run(listener)?.await?;
    Ok(())
}
