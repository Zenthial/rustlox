use crate::values::{Value, ValueArray};

pub enum OpCode {
    OpReturn,
    OpSubtract,
    OpAdd,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpConstant(usize),
}

pub struct Chunk {
    pub lines: Vec<i32>,
    pub code: Vec<OpCode>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn init() -> Self {
        Chunk {
            lines: Vec::new(),
            code: Vec::new(),
            constants: ValueArray::init(),
        }
    }

    pub fn write(&mut self, byte: OpCode, line_num: i32) {
        self.code.push(byte);
        self.lines.push(line_num);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        return self.constants.values.len() - 1;
    }
}
