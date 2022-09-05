use std::{path::PathBuf, io::Result, env};
use crate::formatters::{get_formatter, Formatter};
use dirs;
use std::fs;
use pickledb::PickleDb;

pub struct Config {
    pub porcelain: bool,
}

impl Config {
    pub fn new(porcelain: bool) -> Self {
        return Config {
            porcelain: porcelain
        };
    }

    /// gets the path and creates if not exist
    pub fn get_and_create(&self) -> PathBuf {
        match dirs::config_dir() {
            Some(dir) => {
                if dir.exists() {
                    return dir.join("resync/");
                }

                if fs::create_dir(&dir).is_err() && self.porcelain == false {
                    println!("Failed creating resync config dir. Using temp dir");
                    return env::temp_dir();
                }

                dir.join("resync/")
            },

            _ => {
                if self.porcelain == false {
                    println!("User config dir not found, using temp dir");
                }
                env::temp_dir()
            }
        }
    }

    pub fn get_formatter(&self, porcelain: &bool) -> Box<dyn Formatter> {
        get_formatter(porcelain)
    }

    pub fn get_db_path(&self) -> PathBuf {
        self.get_and_create().join("file_info.db")
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
            Err(_) => {
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
