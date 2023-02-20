use std::net::TcpListener;

pub type BaseUrl = String;

pub fn spawn_app() -> BaseUrl {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("listener binded to random available port");
    let port = listener
        .local_addr()
        .expect("local socket address data")
        .port();
    let server = zero2prod::startup::run(listener).expect("server running");
    tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
