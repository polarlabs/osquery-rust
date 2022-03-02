use strum_macros::Display;

// ColumnDef defines a column used in a table plugin.
// Prefer using the helper functions to create a ColumnDef.
#[derive(Clone, Debug)]
pub struct ColumnDef {
    name: String,
    t: ColumnType,
}

#[derive(Clone, Display, Debug)]
#[strum(serialize_all = "UPPERCASE")]
pub enum ColumnType {
    // TEXT: containing strings
    Text,
    // INTEGER: containing integers
    Integer,
    // BIGINT: containing large integers
    BigInt,
    // DOUBLE: containing floating point values
    Double,
}

impl ColumnDef {
    pub fn new(name: &str, t: ColumnType) -> Self {
        ColumnDef {
            name: name.to_owned(),
            t,
        }
    }

    pub(crate) fn name(&self) -> String {
        self.name.to_string()
    }

    pub(crate) fn t(&self) -> String {
        self.t.to_string()
    }
}
