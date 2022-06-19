use std::path::PathBuf;
use crate::formatters::{get_formatter, Formatter};
use dirs;
use pickledb::PickleDb;

pub struct Config {
}

impl Config {
    /// gets the path and creates if not exist
    pub fn get_and_create(&self) -> PathBuf {
        dirs::config_dir().expect("Failed to get config dir").join("resync/")
    }

    pub fn get_formatter(&self, porcelain: &bool) -> Box<dyn Formatter> {
        get_formatter(porcelain)
    }

    pub fn get_db_path(&self) -> PathBuf {
        self.get_and_create().join("file_info.db")
    }

    pub fn open_db(&self) -> PickleDb {
        PickleDb::new(
            self.get_and_create().join("file_info.db"),
            pickledb::PickleDbDumpPolicy::AutoDump,
            pickledb::SerializationMethod::Json
        )
    }
}
