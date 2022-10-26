use std::sync::Arc;

use lib::def::state::State;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::lib::grpc::start_server;

pub mod lib;

pub static mut STATE: Lazy<Arc<Mutex<State>>> = Lazy::new(|| Arc::new(Mutex::new(State::new())));

#[tokio::main]
async fn main() {
    start_server().await;
}
