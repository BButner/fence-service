use crate::lib::grpc::start_server;

pub mod lib;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    start_server().await;
}
