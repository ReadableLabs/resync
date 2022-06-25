use std::{path::PathBuf, io::Result};
use crate::formatters::{get_formatter, Formatter};
use dirs;
use std::fs;
use pickledb::PickleDb;

pub struct Config {
}

impl Config {
    /// gets the path and creates if not exist
    pub fn get_and_create(&self) -> Result<PathBuf> {
        let dir = dirs::config_dir().expect("Failed to get config dir").join("resync/");
        if !dir.exists() {
            fs::create_dir(&dir).expect("Failed to create config directory");
        }

        Ok(dir)
    }

    pub fn get_formatter(&self, porcelain: &bool) -> Box<dyn Formatter> {
        get_formatter(porcelain)
    }

    pub fn get_db_path(&self) -> PathBuf {
        self.get_and_create().unwrap().join("file_info.db")
    }

    pub fn open_db(&self, debug: bool) -> PickleDb {
        let file = self.get_db_path();
        if debug == true {
            return PickleDb::new(
                &file,
                pickledb::PickleDbDumpPolicy::AutoDump,
                pickledb::SerializationMethod::Json
            );
        }
        let db = match PickleDb::load(
            &file,
            pickledb::PickleDbDumpPolicy::AutoDump,
            pickledb::SerializationMethod::Json
        ) {
            Ok(db) => db,
            Err(e) => {
                PickleDb::new(
                    &file,
                    pickledb::PickleDbDumpPolicy::AutoDump,
                    pickledb::SerializationMethod::Json
                )
            }
        };

        return db;
    }
}
