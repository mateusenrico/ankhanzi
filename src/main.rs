use std::{fs::File, path::Path};

pub mod lib;
use crate::lib::Dict;

static READ_PATH: &'static str = "./static/cedict_ts.txt";
static WRITE_NAME: &'static str = "DICT";

fn read_file(path: &str) -> Vec<cedict::DictEntry> {
    let file = File::open(Path::new(path)).unwrap();
    cedict::parse_reader(file).collect::<Vec<cedict::DictEntry>>()
}

fn main() {
    let dict = Dict::new(read_file(READ_PATH), WRITE_NAME, "map", "@|@");
    dict.export();
}
