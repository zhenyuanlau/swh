use crate::core::config_file::ConfigFile;
use crate::core::kdl::env::Env;
use crate::core::kdl::include::Include;
use crate::error::SwhError;
use log::info;
use miette::{IntoDiagnostic, Result};
use std::fmt;
use std::fmt::Formatter;

#[derive(knuffel::Decode, Debug, Clone)]
pub struct Config {
    #[knuffel(child, unwrap(argument))]
    version: Option<String>,
    #[knuffel(children(name = "env"))]
    envs: Vec<Env>,
    #[knuffel(children(name = "include"))]
    includes: Vec<Include>,
}

impl Config {
    pub fn enabled_envs(&self) -> Vec<Env> {
        self.envs.iter().cloned().filter(|e| e.enabled).collect()
    }

    pub fn list(&self) -> Vec<Vec<String>> {
        let mut env_vec = Vec::with_capacity(self.envs.len());
        for (idx, env) in self.envs.to_vec().iter().enumerate() {
            env_vec.push(vec![
                idx.to_string(),
                env.name.clone(),
                env.enabled.to_string(),
            ]);
        }
        env_vec
    }

    pub fn get_envs(&self, enabled: &Option<bool>) -> Vec<Vec<String>> {
        match enabled {
            Some(en) => {
                info!("{}", en);
                let target_envs: Vec<_> = self.envs.iter().filter(|e| e.enabled == *en).collect();
                let mut env_vec = Vec::with_capacity(target_envs.len());
                for (idx, env) in target_envs.iter().enumerate() {
                    env_vec.push(vec![
                        idx.to_string(),
                        env.name.clone(),
                        env.enabled.to_string(),
                    ]);
                }
                env_vec
            }
            None => {
                let mut env_vec = Vec::with_capacity(self.envs.len());
                for (idx, env) in self.envs.iter().enumerate() {
                    env_vec.push(vec![
                        idx.to_string(),
                        env.name.clone(),
                        env.enabled.to_string(),
                    ]);
                }
                env_vec
            }
        }
    }

    pub fn show(&self, name: Option<&String>) -> Vec<Vec<String>> {
        let f = |e: &Env| -> Vec<Vec<String>> {
            let mut hosts = Vec::with_capacity(e.hosts.len());
            for host in e.hosts.iter().cloned() {
                hosts.push(host.into())
            }
            hosts
        };

        let hosts = match name {
            Some(n) => self
                .envs
                .iter()
                .find(|e| e.name == *n)
                .map(|e| f(e))
                .unwrap(),
            None => self
                .envs
                .to_vec()
                .iter()
                .filter(|e| e.enabled)
                .flat_map(|e| f(e))
                .collect(),
        };
        hosts
    }

    pub fn get_env(&self, env: &str) -> Env {
        self.envs.iter().cloned().find(|e| e.name == *env).unwrap()
    }

    pub fn toggle(&mut self, env: &str) -> Result<()> {
        if let Some(mut target) = self.envs.iter_mut().find(|e| e.name == env) {
            target.enabled = !target.enabled;
            ConfigFile::write(self)?;
        } else {
            return Err(SwhError::EnvNotFound(env.to_string())).into_diagnostic();
        }
        Ok(())
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut content = String::new();
        if let Some(version) = self.version.clone() {
            content.push_str(format!("version \"{}\"\n", version).as_str());
        }
        for env in self.envs.iter() {
            content.push_str(env.to_string().as_str());
        }
        for include in self.includes.iter() {
            content.push_str(include.to_string().as_str());
        }
        f.write_str(content.as_str())
    }
}

impl From<Config> for Vec<(String, String, String)> {
    fn from(value: Config) -> Self {
        let mut vec = Vec::new();
        for env in value.envs.as_slice() {
            if !env.enabled {
                continue;
            }

            for host in env.hosts.iter().cloned() {
                let line: (String, String, String) = host.into();
                vec.push(line);
            }
        }
        vec
    }
}
