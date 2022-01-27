mod eval;
mod exp;
mod functions;
mod oadate;
mod parser;
mod token;
mod value;

use std::collections::HashMap;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage example: rexp -(1+(2+3)*4-5)");
        return;
    }
    let mut p = parser::Parser::new(&args[1]);
    p.tokenize();
    let e = p.parse();
    let context: HashMap<String, value::Value> = HashMap::default();
    let evaluator = eval::Evaluator {
        expr: e,
        context: &context,
    };
    let result = evaluator.resolve();
    println!("Input Formula:\n {} \n", p.render());
    println!("Parse Result: \n {} \n", p.pretty());
    println!("Eval Result:\n {:?}\n", result);
}
