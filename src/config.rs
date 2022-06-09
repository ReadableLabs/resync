use std::path::PathBuf;
use pickledb::PickleDb;
use dirs;

pub struct Config {
}

impl Config {
    /// gets the path and creates if not exist
    pub fn path(&self) -> PathBuf {
        dirs::config_dir().expect("Failed to get config dir").join("resync/")
    }

    pub fn get_db_path(&self) -> PathBuf {
        self.path().join("file_info.db")
    }
}
