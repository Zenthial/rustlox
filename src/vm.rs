use std::ops::Deref;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::compile,
    debug::disassemble_instruction,
    values::{print_value, ObjectType, Value},
};

#[derive(Debug)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub enum Operation {
    Greater,
    Less,
    Plus,
    Minus,
    Star,
    Div,
}

// one key thing to note here is that the books implementation uses an ip pointer
// we don't do this, we just iterate through the code vector
// pointer fuckery isn't that useful in rust, nor is it suggested due to the memory model

// we don't keep an IP because the chunk is just the start
pub struct VM {
    pub chunk: Box<Chunk>,
    debug: bool,

    stack: Vec<Value>,
}

impl VM {
    pub fn init() -> Self {
        VM {
            chunk: Box::new(Chunk::init()),
            debug: false,
            stack: Vec::new(),
        }
    }

    pub fn set_debug(&mut self) {
        self.debug = true
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let mut chunk = Chunk::init();
        if !compile(source, &mut chunk) {
            return InterpretResult::InterpretCompileError;
        }

        self.chunk = Box::new(chunk);
        self.run()
    }

    fn peak(&self, distance: usize) -> Option<&Value> {
        return self.stack.get(self.stack.len() - 1 - distance);
    }

    fn runtime_error(&mut self, format: &str) {
        eprint!("{format} ");

        let index = self.chunk.code.len() - 1;
        let line = self.chunk.lines[index];
        eprintln!("[line {}] in script", line);
        // reset stack
        self.stack.clear();
    }

    fn binary_op(&mut self, operation: Operation) -> InterpretResult {
        if let Some(peak_value) = self.peak(0) {
            if !peak_value.is_number() {
                self.runtime_error("Operands must be numbers.");
                return InterpretResult::InterpretRuntimeError;
            }
        } else {
            return InterpretResult::InterpretRuntimeError;
        }

        let b = match self.stack.pop() {
            Some(val) => val,
            None => return InterpretResult::InterpretCompileError,
        }
        .as_number();

        let a = match self.stack.pop() {
            Some(val) => val,
            None => return InterpretResult::InterpretCompileError,
        }
        .as_number();

        match operation {
            Operation::Plus => self.stack.push(Value::from_number(a + b)),
            Operation::Minus => self.stack.push(Value::from_number(a - b)),
            Operation::Star => self.stack.push(Value::from_number(a * b)),
            Operation::Div => self.stack.push(Value::from_number(a / b)),
            Operation::Greater => self.stack.push(Value::from_bool(a > b)),
            Operation::Less => self.stack.push(Value::from_bool(a < b)),
        }

        InterpretResult::InterpretOk
    }

    fn allocate_object(&mut self, object_type: ObjectType) {
        // i don't believe i need to handle garbage collection on our object types yet, as they are automatically cleaned up due to rust's memory management
        self.stack.push(Value::Object(object_type));
    }

    fn concatenate(&mut self) -> InterpretResult {
        let b = match self.stack.pop() {
            Some(val) => val,
            None => return InterpretResult::InterpretCompileError,
        }
        .as_string();

        let a = match self.stack.pop() {
            Some(val) => val,
            None => return InterpretResult::InterpretCompileError,
        }
        .as_string();

        self.stack.push(Value::from_string(a.content + &b.content));
        InterpretResult::InterpretOk
    }

    fn run(&mut self) -> InterpretResult {
        for instruction in &self.chunk.code {
            if self.debug {
                for element in &self.stack {
                    print!("[{element}]");
                }
                println!("");

                disassemble_instruction(&self.chunk.lines, &self.chunk.constants, &instruction, 0);
            }

            match instruction {
                OpCode::OpReturn => {
                    let pop_val = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    print_value(&pop_val);
                    println!("");
                    return InterpretResult::InterpretOk;
                }
                OpCode::OpNegate => {
                    if let Some(peak_value) = self.peak(0) {
                        if !peak_value.is_number() {
                            self.runtime_error("Operands must be numbers.");
                            return InterpretResult::InterpretRuntimeError;
                        }
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }

                    let pop_val = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    // as_number can panic if we do not have the above check
                    self.stack.push(Value::from_number(-pop_val.as_number()));
                }
                OpCode::OpConstant(index) => {
                    let constant = self.chunk.constants.get(&index);
                    // push value
                    self.stack.push(constant.deref().clone());
                }
                // definitely some way to not have all this repeated code, but we're prototyping
                OpCode::OpGreater => return self.binary_op(Operation::Greater),
                OpCode::OpLess => return self.binary_op(Operation::Less),
                OpCode::OpDivide => return self.binary_op(Operation::Div),
                OpCode::OpMultiply => return self.binary_op(Operation::Star),
                OpCode::OpAdd => {
                    let peak_0 = self.peak(0);
                    let peak_1 = self.peak(1);
                    if peak_0.is_some() && peak_1.is_some() {
                        let value_0 = peak_0.unwrap();
                        let value_1 = peak_1.unwrap();

                        if value_0.is_string() && value_1.is_string() {
                            return self.concatenate();
                        } else if value_0.is_number() && value_1.is_number() {
                            return self.binary_op(Operation::Plus);
                        }
                    }

                    self.runtime_error("Operands must be two numbers or two strings.");
                    return InterpretResult::InterpretRuntimeError;
                }
                OpCode::OpSubtract => return self.binary_op(Operation::Minus),
                OpCode::OpNil => self.stack.push(Value::from_nil()),
                OpCode::OpTrue => self.stack.push(Value::from_bool(true)),
                OpCode::OpFalse => self.stack.push(Value::from_bool(false)),
                OpCode::OpNot => {
                    let pop_val = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    self.stack.push(Value::from_bool(pop_val.is_falsey()));
                }
                OpCode::OpEqual => {
                    let b = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    let a = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    self.stack.push(Value::from_bool(a == b));
                }
            }
        }

        InterpretResult::InterpretOk
    }
}
