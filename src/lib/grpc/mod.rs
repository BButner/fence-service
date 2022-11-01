use tonic::{transport::Server, Request, Response, Status};

use self::grpc_fence::{
    fence_manager_server::{FenceManager, FenceManagerServer},
    CursorLockResponse, DisplayList,
};

use super::hooks::windows::{set_displays, start_mouse_hook, stop_mouse_hook};

pub mod grpc_fence {
    tonic::include_proto!("fence");
}

#[derive(Default)]
pub struct Manager {}

#[tonic::async_trait]
impl FenceManager for Manager {
    async fn get_displays(&self, _request: Request<()>) -> Result<Response<DisplayList>, Status> {
        let state = unsafe { crate::STATE.lock().await };

        Ok(Response::new(DisplayList {
            displays: state
                .displays
                .clone()
                .into_iter()
                .map(|d| d.into())
                .collect(),
        }))
    }

    async fn set_displays(&self, request: Request<DisplayList>) -> Result<Response<()>, Status> {
        let mut state = unsafe { crate::STATE.lock().await };

        let displays: Vec<crate::lib::def::display::Display> = request
            .into_inner()
            .displays
            .into_iter()
            .map(|d| d.into())
            .collect();

        state.displays = displays.clone();

        set_displays(displays);

        Ok(Response::new(()))
    }

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

impl Into<grpc_fence::Display> for crate::lib::def::display::Display {
    fn into(self) -> grpc_fence::Display {
        grpc_fence::Display {
            name: self.name,
            width: self.width,
            height: self.height,
            top: self.top,
            left: self.left,
            selected: self.selected,
        }
    }
}

impl Into<crate::lib::def::display::Display> for grpc_fence::Display {
    fn into(self) -> crate::lib::def::display::Display {
        crate::lib::def::display::Display {
            name: self.name,
            width: self.width,
            height: self.height,
            top: self.top,
            left: self.left,
            selected: self.selected,
        }
    }
}

pub async fn start_server() {
    let addr = "0.0.0.0:50052".parse().unwrap();

    Server::builder()
        .add_service(FenceManagerServer::new(Manager::default()))
        .serve(addr)
        .await
        .unwrap();
}
