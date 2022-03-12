use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::crate_name;

use osquery_rust::prelude::*;
use osquery_rust::plugin::{ColumnDef, ColumnType, Plugin, Table};

use regex::Regex;

#[osquery_rust::args]
fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // todo: handle non existing socket gracefully
    if !args.standalone {
        let mut manager = Server::new(Some(crate_name!()), args.socket().unwrap().as_str()).unwrap();

        manager.register_plugin(Plugin::Table(Table::new("proc_meminfo", columns(), generate)));

        manager.run();
    }

    Ok(())
}

fn columns() -> Vec<ColumnDef> {
    let mut columns: Vec<ColumnDef> = Vec::new();
    let regex = Regex::new(r"(?P<label>\S+):").unwrap();

    let f = File::open("/proc/meminfo").unwrap();
    let reader = BufReader::new(f);

    for (_index, line) in reader.lines().enumerate() {
        let s: String = line.unwrap();

        let cap = regex.captures(s.as_str()).unwrap();
        let s = cap[1].replace('(', "_").replace(')', "");
        columns.push(ColumnDef::new(s.to_lowercase().as_str(), ColumnType::BigInt));
    }

    columns
}

fn generate(_req: ExtensionPluginRequest) -> ExtensionResponse {
    let mut resp = ExtensionPluginResponse::new();

    resp.push(proc_meminfo());

    ExtensionResponse::new(ExtensionStatus::default(), resp)
}

fn proc_meminfo() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let regex = Regex::new(r"(?P<label>\S+):\s+(?P<number>\d+)").unwrap();

    let f = File::open("/proc/meminfo").unwrap();
    let reader = BufReader::new(f);

    for (_index, line) in reader.lines().enumerate() {
        let s: String = line.unwrap();
        let cap = regex.captures(s.as_str()).unwrap();
        let s = cap[1].replace('(', "_").replace(')', "");
        map.insert(s.to_lowercase(), cap[2].to_string());
    }

    map
}
