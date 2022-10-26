use lib::def::state::State;
use tokio::sync::Mutex;

use crate::lib::grpc::start_server;

pub mod lib;

pub static mut STATE: Option<Mutex<State>> = None;

#[tokio::main]
async fn main() {
    unsafe {
        STATE = Some(Mutex::new(State::new()));
    }

    start_server().await;
}
