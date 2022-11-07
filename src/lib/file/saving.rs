use std::{io::Error, path::Path};

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use crate::lib::def::display::Display; // for write_all() // for read_to_end()

pub async fn save_displays() -> Result<bool, Error> {
    let state = unsafe { crate::STATE.lock().await };

    let displays = state.displays.clone();

    let mut app_dir = crate::lib::file::statics::app_dir().unwrap();

    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir).unwrap();
    }

    app_dir.push(Path::new("displays.json"));

    let file = tokio::fs::File::create(app_dir).await;

    if let Ok(mut file) = file {
        let json_bytes = serde_json::to_string_pretty(&displays)
            .unwrap()
            .into_bytes();

        let result = file.write_all(&json_bytes).await;

        match result {
            Ok(_) => return Ok(true),
            Err(e) => return Err(e),
        }
    }

    Ok(false)
}

pub async fn load_displays() -> Result<(), Error> {
    let mut app_dir = crate::lib::file::statics::app_dir().unwrap();

    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir).unwrap();
    }

    app_dir.push(Path::new("displays.json"));

    let file = tokio::fs::File::open(app_dir).await;

    if let Ok(mut file) = file {
        let mut json_bytes = Vec::new();

        let result = file.read_to_end(&mut json_bytes).await;

        match result {
            Ok(_) => {
                let displays: Vec<Display> = serde_json::from_slice(&json_bytes).unwrap();

                let mut state = unsafe { crate::STATE.lock().await };

                state.displays = displays;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
