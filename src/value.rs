use crate::oadate;

pub trait Values: Sized {
    fn to_value(&self) -> Value;
}

type Array = Vec<Value>;
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    String(String),
    Date(f64),
    Empty,
    Unknown,
    Error(String),
    Array(Array),
}

impl Value {
    pub fn to_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Boolean(b) => (*b as i8) as f64,
            _ => 0f64,
        }
    }
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }
    pub fn is_error(&self) -> bool {
        matches!(self, Value::Error(_))
    }
    pub fn as_string(&self) -> String {
        match self {
            Value::Error(e) => e.clone(),
            Value::String(t) => t.clone(),
            Value::Number(f) => format!("{}", f),
            Value::Date(f) => oadate::from_oadate(*f)
                .format(&oadate::default_format())
                .unwrap(),
            _ => String::from(""),
        }
    }
}

impl Values for &str {
    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

impl Values for String {
    fn to_value(&self) -> Value {
        Value::String(self.clone())
    }
}

impl<V: Into<Value>> From<Vec<V>> for Value {
    fn from(val: Vec<V>) -> Value {
        Value::Array(val.into_iter().map(|v| v.into()).collect())
    }
}

impl<T: Values> From<T> for Value {
    fn from(val: T) -> Value {
        val.to_value()
    }
}

macro_rules! impl_values_trait {
    ($variant:ident :$T:ty,$A:ty) => {
        impl Values for $T {
            fn to_value(&self) -> Value {
                Value::$variant(*self as $A)
            }
        }
    };
}
impl_values_trait!(Number: i8, f64);
impl_values_trait!(Number: u8, f64);
impl_values_trait!(Number: i32, f64);
impl_values_trait!(Number: u32, f64);
// converts from i64 to f64 can't losslessly
impl_values_trait!(Number: i64, f64);
impl_values_trait!(Boolean: bool, bool);

#[cfg(test)]
mod tests {
    use crate::value::Value;
    #[test]
    fn test_get_ref() {
        let v: Value = Value::from(1);
        assert_eq!(v, Value::Number(1.0));
        let v: Value = Value::from(true);
        assert_eq!(v, Value::Boolean(true));
        let v: Value = Value::from("true");
        assert_eq!(v, Value::String(String::from("true")));
    }
}
