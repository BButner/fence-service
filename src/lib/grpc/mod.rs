use tonic::{transport::Server, Request, Response, Status};

use self::grpc_fence::{
    fence_manager_server::{FenceManager, FenceManagerServer},
    ConfigResponse, CursorLockResponse, DisplayList, DisplayToggleRequest, DisplayToggleResponse,
    SaveConfigResponse,
};

use super::{
    def::config::Config,
    file::saving::save_config,
    hooks::windows::{set_displays, start_mouse_hook, stop_mouse_hook},
};

pub mod grpc_fence {
    tonic::include_proto!("fence");
}

#[derive(Default)]
pub struct Manager {}

#[tonic::async_trait]
impl FenceManager for Manager {
    async fn get_config(&self, _request: Request<()>) -> Result<Response<ConfigResponse>, Status> {
        let state = unsafe { crate::STATE.lock().await };

        Ok(Response::new(ConfigResponse {
            displays: state
                .config
                .displays
                .clone()
                .into_iter()
                .map(|d| d.into())
                .collect(),
            ui_display_factor: state.config.ui_display_factor,
            active_by_default: state.config.active_by_default,
        }))
    }

    async fn save_config(
        &self,
        request: Request<ConfigResponse>,
    ) -> Result<Response<ConfigResponse>, Status> {
        let mut state = unsafe { crate::STATE.lock().await };

        let config: Config = Config {
            displays: request
                .into_inner()
                .displays
                .into_iter()
                .map(|d| d.into())
                .collect(),
            ui_display_factor: state.config.ui_display_factor,
            active_by_default: state.config.active_by_default,
        };

        state.config = config.clone();
        let result = save_config(config).await;

        match result {
            Ok(_) => Ok(Response::new(ConfigResponse {
                displays: state
                    .config
                    .displays
                    .clone()
                    .into_iter()
                    .map(|d| d.into())
                    .collect(),
                ui_display_factor: state.config.ui_display_factor,
                active_by_default: state.config.active_by_default,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
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

    async fn toggle_display_selected(
        &self,
        request: Request<grpc_fence::DisplayToggleRequest>,
    ) -> Result<Response<DisplayToggleResponse>, Status> {
        let mut state = unsafe { crate::STATE.lock().await };
        let request_display = request.into_inner();

        let mut display = state
            .config
            .displays
            .iter_mut()
            .find(|d| d.name == request_display.name.clone())
            .unwrap();

        display.selected = !display.selected;

        Ok(Response::new(DisplayToggleResponse {
            selected: display.selected,
        }))
    }

    async fn set_displays(&self, request: Request<DisplayList>) -> Result<Response<()>, Status> {
        let mut state = unsafe { crate::STATE.lock().await };
        let request_displays = request.into_inner();

        state.config.displays = request_displays
            .displays
            .into_iter()
            .map(|d| d.into())
            .collect();

        set_displays(state.config.displays.clone());

        Ok(Response::new(()))
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
