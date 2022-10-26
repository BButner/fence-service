use tonic::{Request, Response, Status};

pub mod grpc_display {
    tonic::include_proto!("displays");
}

use grpc_display::display_manager_server::{DisplayManager, DisplayManagerServer};
use grpc_display::Display;

#[derive(Default)]
pub struct Manager {}

#[tonic::async_trait]
impl DisplayManager for Manager {
    async fn say_display(&self, request: Request<Display>) -> Result<Response<Display>, Status> {
        println!("Got a request: {:?}", request);

        Ok(Response::new(request.into_inner()))
    }
}

pub fn get_service() -> DisplayManagerServer<Manager> {
    DisplayManagerServer::new(Manager::default())
}
