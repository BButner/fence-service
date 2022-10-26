use tonic::{Request, Response, Status};

pub mod grpc_display {
    tonic::include_proto!("displays");
}

use grpc_display::display_manager_server::{DisplayManager, DisplayManagerServer};

use self::grpc_display::DisplayList;

#[derive(Default)]
pub struct Manager {}

#[tonic::async_trait]
impl DisplayManager for Manager {
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

        state.displays = request
            .into_inner()
            .displays
            .into_iter()
            .map(|d| d.into())
            .collect();

        Ok(Response::new(()))
    }
}

pub fn get_service() -> DisplayManagerServer<Manager> {
    DisplayManagerServer::new(Manager::default())
}

impl Into<grpc_display::Display> for crate::lib::def::display::Display {
    fn into(self) -> grpc_display::Display {
        grpc_display::Display {
            name: self.name,
            width: self.width,
            height: self.height,
            top: self.top,
            left: self.left,
            selected: self.selected,
        }
    }
}

impl Into<crate::lib::def::display::Display> for grpc_display::Display {
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
