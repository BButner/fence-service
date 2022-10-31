use tonic::{Request, Response, Status};

pub mod grpc_cursor {
    tonic::include_proto!("cursor");
}

use grpc_cursor::cursor_manager_server::{CursorManager, CursorManagerServer};

use crate::lib::hooks::windows::{start_mouse_hook, stop_mouse_hook};

use self::grpc_cursor::CursorLockResponse;

#[derive(Default)]
pub struct Manager {}

#[tonic::async_trait]
impl CursorManager for Manager {
    async fn activate_cursor_lock(
        &self,
        _request: Request<()>,
    ) -> Result<Response<CursorLockResponse>, Status> {
        start_mouse_hook();

        Ok(Response::new(CursorLockResponse {
            is_locked: true,
            error_message: "This is an example".to_string(),
        }))
    }

    async fn deactivate_cursor_lock(
        &self,
        _request: Request<()>,
    ) -> Result<Response<CursorLockResponse>, Status> {
        stop_mouse_hook().await;

        Ok(Response::new(CursorLockResponse {
            is_locked: false,
            error_message: "This is an example".to_string(),
        }))
    }
}

pub fn get_service() -> CursorManagerServer<Manager> {
    CursorManagerServer::new(Manager::default())
}
