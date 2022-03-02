use strum_macros::{EnumString, EnumVariantNames};

#[derive(EnumString, EnumVariantNames, Eq, PartialEq, Hash, Debug)]
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
