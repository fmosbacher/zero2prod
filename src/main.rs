use std::{error::Error, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    zero2prod::run(listener)?.await?;
    Ok(())
}
