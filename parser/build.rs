extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .always_use_colors()
        .process().unwrap();
}