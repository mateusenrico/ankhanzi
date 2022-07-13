pub mod lib;
use crate::lib::Dict;

fn main() {
    let dict = Dict::init();

    println!("{}", dict.count_discovered());
}
