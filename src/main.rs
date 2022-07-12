pub mod lib;
use crate::lib::Dict;

// TODO: acrescentar documentação
fn main() {
    let dict = Dict::init();

    dict.export();
}
