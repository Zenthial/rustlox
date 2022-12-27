use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjString {
    pub content: Box<String>,
}

impl ObjString {
    fn allocate(chars: Box<String>) -> Self {
        ObjString { content: chars }
    }
}

impl Display for ObjString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    String(ObjString),
}

impl ObjectType {
    fn print(&self) -> String {
        match self {
            Self::String(s) => s.content.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    Object(ObjectType),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(value) => write!(f, "{}", value),
            Value::Nil => write!(f, "Nil"),
            Value::Number(value) => write!(f, "{}", value),
            Value::Object(obj) => write!(f, "{}", obj.print()),
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

    pub fn from_string(a: String) -> Self {
        return Self::Object(ObjectType::String(ObjString {
            content: Box::new(a),
        }));
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
            _ => panic!("incorrect usage of as_bool"),
        }
    }

    pub fn as_object(&self) -> ObjectType {
        match self {
            Value::Object(o) => o.clone(),
            _ => panic!("Incorrect usage of as_object"),
        }
    }

    pub fn as_string(&self) -> ObjString {
        match self.as_object() {
            ObjectType::String(s) => s,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        if let Self::Object(s) = self {
            if let ObjectType::String(_) = s {
                return true;
            }
        }

        false
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

    pub fn is_falsey(&self) -> bool {
        return self.is_nil() || (self.is_bool() && !self.as_bool());
    }
}

pub fn print_value(val: &Value) {
    print!("{}", val);
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
        return self.values.get(*index).unwrap_or(&Value::Nil);
    }

    pub fn take(&mut self, index: &usize) -> Value {
        if self.values.get(*index).is_none() {
            return Value::from_number(-1.0);
        } else {
            return self.values.swap_remove(*index);
        }
    }
}
