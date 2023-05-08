use crate::core::kdl::host::Host;
use std::fmt;
use std::fmt::Formatter;

#[derive(knuffel::Decode, Debug, Clone)]
pub struct Env {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(property)]
    pub enabled: bool,
    #[knuffel(children(name = "host"))]
    pub hosts: Vec<Host>,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut content = String::new();
        content.push_str(format!("env \"{}\" enabled={} {{\n", self.name, self.enabled).as_str());
        for host in self.hosts.iter() {
            content.push_str(host.to_string().as_str());
        }
        content.push_str("}\n");
        f.write_str(content.as_str())
    }
}

impl From<Env> for Vec<String> {
    fn from(value: Env) -> Self {
        vec![value.name, value.enabled.to_string()]
    }
}
