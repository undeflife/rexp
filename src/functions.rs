use crate::oadate;
use crate::value::Value;

use std::collections::HashMap;

const GUID_LENGTH: usize = 11;

/* pub enum CriterionType {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
} */

pub fn add(left: &Value, right: &Value) -> Value {
    Value::Number(left.to_number() + right.to_number())
}

pub fn sum(args: &[Value]) -> Value {
    let mut result = 0f64;
    for v in args {
        result += v.to_number();
    }
    Value::Number(result)
}

pub fn count(args: &[Value]) -> Value {
    let mut cnt = 0.0;
    for v in args {
        if v.is_number() {
            cnt += 1.0;
        }
    }
    Value::Number(cnt)
}

pub fn avarage(args: &[Value]) -> Value {
    let cnt = count(args).to_number();
    if cnt == 0.0 {
        return Value::Empty;
    }
    Value::Number(sum(args).to_number() / cnt)
}

pub fn subtract(left: &Value, right: &Value) -> Value {
    Value::Number(left.to_number() - right.to_number())
}

pub fn multiply(left: &Value, right: &Value) -> Value {
    Value::Number(left.to_number() * right.to_number())
}
pub fn divide(left: &Value, right: &Value) -> Value {
    if right.to_number() == 0f64 {
        return Value::Error("#DIV/0".into());
    }
    Value::Number(left.to_number() / right.to_number())
}

pub fn today(_args: &[Value]) -> Value {
    Value::Number(oadate::today())
}

pub fn compare(left: &Value, right: &Value, op: &str) -> Value {
    match op {
        "=" | "==" => Value::Boolean(left.to_number() - right.to_number() == 0.0),
        ">" => Value::Boolean(left.to_number() - right.to_number() > 0.0),
        ">=" => Value::Boolean(left.to_number() - right.to_number() >= 0.0),
        "<" => Value::Boolean(left.to_number() - right.to_number() < 0.0),
        "<=" => Value::Boolean(left.to_number() - right.to_number() <= 0.0),
        "<>" => Value::Boolean(left.to_number() - right.to_number() != 0.0),
        _ => Value::Boolean(false),
    }
}

// return values in eval context  key:value pairs, key should be like "11-byte-table-guid.11-byte-column-guid"
pub fn get_ref_value(context: HashMap<String, Value>) -> Box<dyn Fn(&[Value]) -> Value> {
    Box::new(move |val| {
        if val.is_empty() {
            return Value::Error("#REF!".to_string());
        }
        let ref_string = val.get(0).unwrap().as_string();
        let (_, column_id) = ref_string.split_at(GUID_LENGTH + 1);
        return match context.get(column_id) {
            Some(arg) => arg.clone(),
            _ => Value::Error("#REF!".to_string()),
        };
    })
}
