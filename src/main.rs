use std::{fs::File, path::Path};

pub mod lib;
use crate::lib::Dict;

static READ_PATH: &'static str = "./static/cedict.txt";
static WRITE_NAME: &'static str = "DICT";

fn read_file(path: &str) -> Vec<cedict::DictEntry> {
    let file = File::open(Path::new(path)).unwrap();
    cedict::parse_reader(file).collect::<Vec<cedict::DictEntry>>()
}

// TODO: acrescentar documentação
fn main() {
    let style = std::env::args().nth(1).expect("Missing style"); // Values: map or list

    let dict = Dict::new(read_file(READ_PATH), WRITE_NAME, &style);

    // TODO: Adicionar funcionalidade de acrescentar o que já tem no deck do Anki
    dict.export();
}
