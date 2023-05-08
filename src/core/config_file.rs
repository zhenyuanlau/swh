use crate::core::kdl::config::Config;
use directories_next::BaseDirs;
use log::error;
use miette::{IntoDiagnostic, Report, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

pub struct ConfigFile;

pub const SWH_CFG_DIR: &str = ".config/swh";
pub const SWH_CFG_FILE_PATH: &str = ".config/swh/config.kdl";

const EXAMPLE_CONFIG: &str = r#"
version "1.0"
// env "local" enabled=true {
//   host "127.0.0.1" name="localhost" {
//     alias "localhost.domain"
//   }
//   host "255.255.255.255" name="broadcast.host"
// }
// include "dev"
"#;

impl ConfigFile {
    pub fn create_if_not_exists() -> Result<()> {
        let swh_config_dir = Self::swh_config_dir().unwrap();
        let swh_config_file = Self::swh_config_file_path().unwrap();

        if !swh_config_dir.exists() {
            fs::create_dir(swh_config_dir).into_diagnostic()?;
            if let Ok(mut file) = fs::File::create(swh_config_file) {
                file.write(EXAMPLE_CONFIG.as_bytes()).into_diagnostic()?;
            }
        }
        Ok(())
    }

    pub fn write(config: &Config) -> Result<()> {
        let swh_config_path = Self::swh_config_file_path().unwrap();
        let mut swh_config = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(swh_config_path)
            .into_diagnostic()?;
        swh_config
            .write(config.to_string().as_bytes())
            .into_diagnostic()?;
        Ok(())
    }

    pub fn load() -> Result<Config> {
        let swh_config_path = Rc::new(Self::swh_config_file_path().unwrap());
        let text = fs::read_to_string(swh_config_path.as_ref()).into_diagnostic()?;
        match knuffel::parse::<Config>(swh_config_path.to_str().unwrap(), &text) {
            Ok(config) => Ok(config),
            Err(e) => {
                error!("{:?}", Report::new(e));
                std::process::exit(1);
            }
        }
    }

    pub fn swh_config_dir() -> Option<PathBuf> {
        if let Some(user_dirs) = BaseDirs::new() {
            let swh_cfg_dir = user_dirs.home_dir().join(SWH_CFG_DIR);
            Some(swh_cfg_dir)
        } else {
            None
        }
    }

    pub fn swh_config_file_path() -> Option<PathBuf> {
        if let Some(user_dirs) = BaseDirs::new() {
            let swh_hosts_file_path = user_dirs.home_dir().join(SWH_CFG_FILE_PATH);
            Some(swh_hosts_file_path)
        } else {
            None
        }
    }
}
