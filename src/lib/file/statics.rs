use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

use directories::ProjectDirs;

pub fn app_dir() -> Result<PathBuf, Error> {
    let dirs = ProjectDirs::from("com", "Fence", "Fence")
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not get app directory"))?;

    Ok(dirs.data_dir().to_path_buf())
}
