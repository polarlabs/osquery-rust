pub(crate) mod column_def;
pub use column_def::ColumnDef;
pub use column_def::ColumnType;

pub(crate) mod query_constraint;
#[allow(unused_imports)]
pub use query_constraint::QueryConstraints;

use std::collections::BTreeMap;

use crate::_osquery as osquery;
use crate::_osquery::{ExtensionPluginRequest, ExtensionPluginResponse, ExtensionResponse, ExtensionStatus};
use crate::plugin::{OsqueryPlugin, Registry};

type TableFn = fn(ExtensionPluginRequest) -> osquery::ExtensionResponse;

#[derive(Clone, Debug)]
pub struct Table {
    name: String,
    columns: Vec<ColumnDef>,
    func: TableFn,
}

impl Table {
    pub fn new(name: &str, columns: Vec<ColumnDef>, func: TableFn) -> Self {
        // todo: error handling, not all names are allowed, e.g. when using - in name, we get:
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
