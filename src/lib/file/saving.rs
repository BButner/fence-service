use std::{io::Error, path::Path};

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use crate::lib::def::config::Config;

pub async fn save_config(config: Config) -> Result<bool, Error> {
    let mut app_dir = crate::lib::file::statics::app_dir().unwrap();

    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir).unwrap();
    }

    app_dir.push(Path::new("config.json"));

    let file = tokio::fs::File::create(app_dir).await;

    if let Ok(mut file) = file {
        let json_bytes = serde_json::to_string_pretty(&config).unwrap().into_bytes();

        let result = file.write_all(&json_bytes).await;

        return match result {
            Ok(_) => Ok(true),
            Err(e) => {
                println!("Error saving config: {}", e);
                Err(e)
            }
        };
    }

    Ok(false)
}

pub async fn load_config() -> Result<(), Error> {
    let mut app_dir = crate::lib::file::statics::app_dir().unwrap();

    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir).unwrap();
    }

    app_dir.push(Path::new("config.json"));

    let file = tokio::fs::File::open(app_dir).await;

    if let Ok(mut file) = file {
        let mut json_bytes = Vec::new();

        let result = file.read_to_end(&mut json_bytes).await;

        match result {
            Ok(_) => {
                let config: Config = serde_json::from_slice(&json_bytes).unwrap();

                let mut state = unsafe { crate::STATE.lock().await };

                state.config = config;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
