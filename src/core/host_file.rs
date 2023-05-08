use crate::core::host_entry::HostEntry;
use crate::core::kdl::config::Config;
use crate::util;
use miette::{miette, IntoDiagnostic, Result};
use std::fs::OpenOptions;
use std::io::Write;

pub const HOST_FILE_PATH: &str = "/etc/hosts";

#[derive(Debug)]
pub struct HostFile {
    pub entries: Option<Vec<HostEntry>>,
}

pub const SWH_CONTENT_START: &str = "SWH_CONTENT_START";
pub const SWH_CONTENT_END: &str = "SWH_CONTENT_END";

impl HostFile {
    pub fn load() -> Result<Self> {
        match util::read_lines(HOST_FILE_PATH) {
            Ok(lines) => {
                let entries: Option<Vec<HostEntry>> = lines
                    .map(
                        |line| match line.unwrap_or_else(|_e| "".to_string()).parse() {
                            Ok(host_entry) => Some(host_entry),
                            _ => None,
                        },
                    )
                    .collect();
                Ok(Self { entries })
            }
            Err(e) => Err(e).map_err(|e| miette!(e)),
        }
    }

    pub fn sync(&mut self, config: Config) -> Result<()> {
        let mut entries: Vec<HostEntry> = config.into();
        self.purge()?;
        self.merge(&mut entries)?;
        self.write()?;
        Ok(())
    }

    fn write(&self) -> Result<()> {
        let host_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(HOST_FILE_PATH);

        match host_file {
            Ok(mut file) => {
                for entry in self.entries.as_ref().unwrap() {
                    writeln!(file, "{}", entry).into_diagnostic()?;
                }
                Ok(())
            }
            Err(e) => Err(e).map_err(|e| miette!(e)),
        }
    }

    fn purge(&mut self) -> Result<()> {
        if let Some(entries) = self.entries.as_ref() {
            let start_opt = entries
                .iter()
                .position(|e| e.comment == Some(SWH_CONTENT_START.to_string()));
            let end_opt = entries
                .iter()
                .position(|e| e.comment == Some(SWH_CONTENT_END.to_string()));
            return match (start_opt, end_opt) {
                (Some(start), Some(end)) => {
                    let entry_slice = entries.as_slice();
                    let content = [&entry_slice[0..start], &entry_slice[end + 1..]].concat();
                    self.entries = Some(content);
                    Ok(())
                }
                _ => Ok(()),
            };
        }
        Ok(())
    }

    fn merge(&mut self, entries: &mut Vec<HostEntry>) -> Result<()> {
        let mut origin = self.entries.as_ref().unwrap().to_vec();
        origin.append(entries);
        self.entries = Some(origin);
        Ok(())
    }
}
