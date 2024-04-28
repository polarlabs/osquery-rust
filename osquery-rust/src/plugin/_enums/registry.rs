use strum_macros::{EnumString, VariantNames};

#[derive(EnumString, VariantNames, Debug, Eq, Hash, PartialEq)]
#[strum(serialize_all = "kebab_case")]
pub enum Registry {
    Config,
    Logger,
    Table,
}

impl ToString for Registry {
    fn to_string(&self) -> String {
        match self {
            Registry::Config => "config".to_string(),
            Registry::Logger => "logger".to_string(),
            Registry::Table => "table".to_string(),
        }
    }
}
