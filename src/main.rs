use std::{net::TcpListener};

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    println!("{}",listener.local_addr().unwrap().port());
    let server = run(listener).unwrap();
    tokio::spawn(server).await?
}