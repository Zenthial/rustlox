use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(value) => write!(f, "{}", value),
            Value::Nil => write!(f, "Nil"),
            Value::Number(value) => write!(f, "{}", value),
        }
    }
}

impl Value {
    pub fn from_bool(b: bool) -> Self {
        return Self::Bool(b);
    }

    pub fn from_number(n: f64) -> Self {
        return Self::Number(n);
    }

    pub fn from_nil() -> Self {
        return Self::Nil;
    }

    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => panic!("incorrect usage of as_number"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => panic!("incorrect usage of as_number"),
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }
}

pub fn print_value(val: &Value) {
    print!("{:?}", val);
}

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn init() -> Self {
        ValueArray { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn get(&self, index: &usize) -> &Value {
        return self.values.get(*index).unwrap_or(&Value::Number(-1.0));
    }
}
