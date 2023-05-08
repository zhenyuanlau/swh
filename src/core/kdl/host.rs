use crate::core::kdl::alias::Alias;
use std::fmt;
use std::fmt::Formatter;

#[derive(knuffel::Decode, Debug, Clone)]
pub struct Host {
    #[knuffel(argument)]
    pub ip: String,
    #[knuffel(property)]
    pub name: String,
    #[knuffel(children(name = "alias"))]
    pub aliases: Vec<Alias>,
}

const HOST_INDENT: &str = "\t";

impl Host {
    pub fn aliases(&self) -> Vec<String> {
        self.aliases.iter().map(|a| a.name.to_owned()).collect()
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut content = String::new();
        content.push_str(
            format!(
                "{}host \"{}\" name=\"{}\" ",
                HOST_INDENT, self.ip, self.name
            )
            .as_str(),
        );
        if !self.aliases.is_empty() {
            content.push_str("{\n");
            for alias in self.aliases.iter() {
                content.push_str(alias.to_string().as_str());
            }
            content.push_str(format!("{}}}", HOST_INDENT).as_str());
        }
        content.push('\n');
        f.write_str(content.as_str())
    }
}

impl From<Host> for Vec<String> {
    fn from(value: Host) -> Self {
        if value.aliases.is_empty() {
            vec![value.ip, value.name]
        } else {
            let aliases: Vec<_> = value.aliases.into_iter().map(|a| a.name).collect();
            vec![value.ip, value.name, aliases.join(" ")]
        }
    }
}

impl From<Host> for (String, String, String) {
    fn from(value: Host) -> Self {
        if value.aliases.is_empty() {
            (value.ip, value.name, "".to_string())
        } else {
            let aliases: Vec<_> = value.aliases.into_iter().map(|a| a.name).collect();
            (value.ip, value.name, aliases.join(" "))
        }
    }
}
