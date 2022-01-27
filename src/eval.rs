use crate::exp::Expression;
use crate::functions;
use crate::value::Value;
use std::collections::HashMap;

pub struct Evaluator<'a> {
    pub expr: Expression,
    pub context: &'a HashMap<String, Value>,
}

impl<'a> Evaluator<'a> {
    pub fn resolve(&self) -> Value {
        self._resolve(&self.expr)
    }

    fn _resolve(&self, e: &Expression) -> Value {
        match &e {
            Expression::Literal(ref v) => v.clone(),
            Expression::Operator { left, op, right } => {
                let lhs = self._resolve(&**left);
                match &**right {
                    None => lhs,
                    Some(r) => {
                        let rhs = self._resolve(r);
                        let op_str = op.as_ref().unwrap().as_str();
                        match op_str {
                            "+" => functions::add(&lhs, &rhs),
                            "-" => functions::subtract(&lhs, &rhs),
                            "*" => functions::multiply(&lhs, &rhs),
                            "/" => functions::divide(&lhs, &rhs),
                            ">" | ">=" | "<" | "<=" | "<>" | "=" | "==" => {
                                functions::compare(&lhs, &rhs, op_str)
                            }
                            // rcompare(left, right, ope.Op)
                            _ => functions::sum(&[lhs, rhs]),
                        }
                    }
                }
            }
            Expression::Function {
                function,
                arguments,
            } => {
                let mut args: Vec<Value> = vec![];
                for arg in arguments.iter() {
                    let v = self._resolve(arg);
                    if v.is_error() {
                        return v;
                    }
                    args.push(v)
                }

                let func = get_function(self.context, function);
                func(&args)
            }
        }
    }
}

fn get_function<'a>(
    context: &HashMap<String, Value>,
    name: &str,
) -> Box<dyn Fn(&'a [Value]) -> Value> {
    match name {
        "SUM" => Box::new(functions::sum),
        "TODAY" => Box::new(functions::today),
        "REF" => functions::get_ref_value(context.clone()),
        "AVERAGE" => Box::new(functions::avarage),
        _ => Box::new(functions::sum),
    }
}
