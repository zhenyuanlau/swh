use std::fmt;
use std::fmt::Formatter;

#[derive(knuffel::Decode, Debug, Clone)]
pub struct Include {
    #[knuffel(argument)]
    pub name: String,
}

impl fmt::Display for Include {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(format!("include \"{}\"\n", self.name).as_str())
    }
}
