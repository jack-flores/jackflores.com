use backend::startup::run;

use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = format!("127.0.0.1:8000");
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
