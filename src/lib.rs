pub mod eval;
pub mod exp;
pub mod functions;
pub mod oadate;
pub mod parser;
pub mod token;
pub mod value;

use eval::Evaluator;
use parser::Parser;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub extern "C" fn resolve(formula: &str) -> String {
    let mut p = Parser::new(formula);
    p.tokenize();
    let e = p.parse();
    // TODO: context should be initialized from input
    let context: HashMap<String, value::Value> = HashMap::default();
    let evaluator = Evaluator {
        expr: e,
        context: &context,
    };
    evaluator.resolve().as_string()
}
