pub mod ast;

use ast::*;
use std::{io::BufRead,io::BufReader, fs::File};

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

fn main() {
    let test = read_test();
    let ast: Algorithm = grammar::AlgorithmParser::new().parse(&test).unwrap();
    println!("{:?}", ast);
}

fn read_test() -> String {
    let f = File::open("src/test.adl").unwrap();
    let mut lines = BufReader::new(f).lines();
    let mut test = String::new();

    loop {
        if let Some(s) = lines.next() {
            test.push_str(&s.unwrap());
        } else {
            break;
        }
    }
    return test;
}
