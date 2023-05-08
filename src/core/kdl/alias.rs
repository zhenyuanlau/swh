use std::fmt;
use std::fmt::Formatter;

#[derive(knuffel::Decode, Debug, Clone)]
pub struct Alias {
    #[knuffel(argument)]
    pub name: String,
}

const ALIAS_INDENT: &str = "\t\t";

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(format!("{}alias \"{}\"\n", ALIAS_INDENT, self.name).as_str())
    }
}
