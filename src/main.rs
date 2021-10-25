extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;

use parser::parse;

fn main() {
    let parsed = parse("1+(2+3)").unwrap_or_else(|e| panic!("{}", e));
    // .iter()
    // .fold(String::new(), |acc, arg| acc + &format!("{}", &arg));
}
