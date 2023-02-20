use std::{error::Error, net::TcpListener};

use axum::{
    routing::{get, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;

pub fn run(
    listener: TcpListener,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, Box<dyn Error>> {
    let app = Router::new().route("/health-check", get(health_check));
    let server = Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}

async fn health_check() {}
