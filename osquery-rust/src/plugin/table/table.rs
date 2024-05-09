use crate::_osquery as osquery;
use crate::plugin::{ColumnDef, OsqueryPlugin, Registry};

use osquery::{ExtensionPluginRequest, ExtensionPluginResponse, ExtensionResponse, ExtensionStatus};
use std::collections::BTreeMap;
use std::str::FromStr;

type TableFn = fn(ExtensionPluginRequest) -> ExtensionResponse;

#[derive(Clone, Debug)]
pub struct Table {
    name: String,
    columns: Vec<ColumnDef>,
    func: TableFn,
}

impl Table {
    pub fn new(name: &str, columns: Vec<ColumnDef>, func: TableFn) -> Self {
        // todo: error handling, not all names are allowed, e.g. when using - in name, we get:
        //       could we provide a test facility to support the user with the extension implementation?
        //W0214 01:20:44.925724  9935 interface.cpp:143] Could not add extension foobar: SQLITE_ERROR
        // Status 1 registering extension foobar (0): Failed adding registry: SQLITE_ERROR
        //routes.insert("foobartable".to_string(), resp);
        Table {
            name: name.to_string(),
            columns,
            func,
        }
    }
}

///
/// Maximum length
///
struct TableName(String);

impl FromStr for TableName {
    type Err = ();

    // todo: fully implement this.
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        s.chars().all(char::is_alphanumeric);

        Ok(TableName(s.to_string()))
    }
}

impl OsqueryPlugin for Table {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn registry(&self) -> Registry {
        todo!()
    }

    fn routes(&self) -> osquery::ExtensionPluginResponse {
        let mut resp = ExtensionPluginResponse::new();

        for column in &self.columns {
            let mut r: BTreeMap<String, String> = BTreeMap::new();

            r.insert("id".to_string(), "column".to_string());
            r.insert("name".to_string(), column.name());
            r.insert("type".to_string(), column.t());
            r.insert("op".to_string(), "0".to_string());

            resp.push(r);
        }

        resp
    }

    fn ping(&self) -> ExtensionStatus {
        todo!()
    }

    fn call(&self, req: ExtensionPluginRequest) -> ExtensionResponse {
        (self.func)(req)
    }

    fn shutdown(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::plugin::table::table::TableName;

    #[test]
    fn check_table_names() {
        let table = TableName::from_str("Test");

        //assert!(TableName::from_str("Test").is_ok());
        //assert!(TableName::from_str("Test-Test").is_ok());
        assert!("test".chars().all(char::is_alphanumeric));
        //assert!("test-test".chars().all(char::is_alphanumeric));
        assert!("test_test".chars().all(|c| c.is_alphanumeric() || c == '_' ));
    }

}
