use crate::core::kdl::config::Config;
use crate::core::kdl::host::Host;
use regex::Regex;
use std::fmt;
use std::fmt::Formatter;
use std::net::IpAddr;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Clone)]
pub struct HostEntry {
    pub ip: Option<IpAddr>,
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub comment: Option<String>,
}

impl HostEntry {
    pub fn empty() -> Self {
        Self {
            ip: None,
            name: None,
            aliases: None,
            comment: None,
        }
    }

    pub fn comment(comment: &str) -> Self {
        Self {
            ip: None,
            name: None,
            aliases: None,
            comment: Some(comment.to_string()),
        }
    }
}

impl PartialEq for HostEntry {
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip
            && self.name == other.name
            && self.aliases == other.aliases
            && self.comment == other.comment
    }
}

impl fmt::Display for HostEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.ip.is_none() && self.comment.is_some() {
            write!(f, "# {}", self.comment.as_ref().unwrap())
        } else if self.ip.is_some() {
            f.write_fmt(format_args!(
                "{}\t{}",
                self.ip.unwrap(),
                self.name.as_ref().unwrap()
            ))?;
            if self.aliases.is_some() {
                write!(
                    f,
                    "\t{}",
                    self.aliases.as_ref().unwrap_or(&vec![]).join(" ")
                )?;
            }
            if self.comment.is_some() {
                write!(f, "\t# {}", self.comment.as_ref().unwrap())?;
            }
            Ok(())
        } else {
            write!(f, "")
        }
    }
}

impl FromStr for HostEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let comment = Regex::new(r"^#\s*(?P<comment>.+)\s*$").unwrap();
        let entry =
            Regex::new(r"^(?P<ip>.+?)\s+(?P<name>.+?)(\s+(?P<aliases>[^#]+))?(#\s*(?P<c>.*))?$")
                .unwrap();
        if comment.is_match(s) {
            Ok(comment
                .captures(s)
                .map(|cap| {
                    HostEntry::comment(cap.name("comment").map(|t| t.as_str().trim()).unwrap_or(""))
                })
                .unwrap())
        } else if entry.is_match(s) {
            let caps = entry.captures(s).unwrap();
            let ip_str = caps.name("ip").map(|t| t.as_str()).unwrap();

            let ip: Option<IpAddr> = match ip_str.parse() {
                Ok(x) => Some(x),
                _ => None,
            };

            let name = caps.name("name").map(|t| String::from(t.as_str().trim()));
            let alias = caps
                .name("aliases")
                .map(|t| String::from(t.as_str().trim()));
            let alias_vec: Option<Vec<String>> = alias.map(|a| {
                a.split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>()
            });
            let comment = caps
                .name("comment")
                .map(|t| String::from(t.as_str().trim()));
            Ok(HostEntry {
                ip,
                name,
                aliases: alias_vec,
                comment,
            })
        } else {
            Ok(HostEntry::empty())
        }
    }
}

impl From<Config> for Vec<HostEntry> {
    fn from(config: Config) -> Self {
        if config.enabled_envs().is_empty() {
            return vec![];
        }
        let mut v = Vec::new();
        v.push(HostEntry::comment(super::host_file::SWH_CONTENT_START));
        for env in config.enabled_envs().iter() {
            for host in env.hosts.to_vec().iter() {
                let entry: HostEntry = host.to_owned().into();
                v.push(entry);
            }
        }
        v.push(HostEntry::comment(super::host_file::SWH_CONTENT_END));
        v
    }
}

impl From<Host> for HostEntry {
    fn from(host: Host) -> Self {
        Self {
            ip: Some(host.ip.parse::<IpAddr>().unwrap()),
            name: Some(host.name.clone()),
            aliases: Some(host.aliases()),
            comment: None,
        }
    }
}
