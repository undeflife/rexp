use crate::value::Value;

#[derive(Debug)]
pub enum Expression {
    Literal(Value),
    Operator {
        op: Option<String>,
        left: Box<Expression>,
        right: Box<Option<Expression>>,
    },
    Function {
        function: String,
        arguments: Vec<Expression>,
    },
}

impl Expression {
    /*  pub fn valid(&self) -> bool {
        return match self {
            Expression::Literal(ref v) => match v {
                Value::Error(_) => true,
                _ => false,
            },
            Expression::Operator { .. } => true,
            Expression::Function { .. } => true,
        };
    } */
}

pub fn get_precedence(op: &str) -> i8 {
    match op {
        "/" => 40,
        "*" => 40,
        "+" => 30,
        "-" => 30,
        "&" => 20,
        "<" => 10,
        ">" => 10,
        "=" => 10,
        "==" => 10,
        "<=" => 10,
        ">=" => 10,
        "<>" => 10,
        _ => -1,
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Expression::Literal(ref v) => write!(f, "{:?}", v.as_string()),
            Expression::Operator { left, op, right } => write!(f, "{}{:?}{:?}", left, op, right),
            Expression::Function {
                function,
                arguments,
            } => write!(f, "{}{:?}", function, arguments),
        };
    }
}
