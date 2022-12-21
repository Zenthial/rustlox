pub type Value = f64;

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
        return self.values.get(*index).unwrap_or(&-1.0);
    }
}
