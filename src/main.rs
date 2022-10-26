use std::sync::Mutex;

use lib::def::state::State;

use crate::lib::grpc::start_server;

pub mod lib;

pub static mut state: Option<Mutex<State>> = None;

#[tokio::main]
async fn main() {
    unsafe {
        state = Some(Mutex::new(State::new()));
    }

    start_server().await;
}
