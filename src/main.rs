pub mod lib;
use crate::lib::Dict;

// TODO: acrescentar documentação
fn main() {
    let dict = Dict::new();

    let unicos = dict
        .list
        .values()
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|x| x.tags.len() == 0)
        .collect::<Vec<_>>();

    println!("TAMANHO UNICO: {}", unicos.len());
    dict.export();
}
