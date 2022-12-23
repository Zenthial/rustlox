use crate::{
    chunk::{Chunk, OpCode},
    compiler::compile,
    debug::disassemble_instruction,
    values::{print_value, Value},
};

#[derive(Debug)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

// one key thing to note here is that the books implementation uses an ip pointer
// we dont do this, we just iterate through the code vector
// pointer fuckery isnt that useful in rust, nor is it suggested due to the memory model
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
                    let pop_val = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    self.stack.push(-pop_val);
                }
                OpCode::OpConstant(index) => {
                    let constant = self.chunk.constants.get(&index);
                    // push value
                    self.stack.push(*constant);
                }
                // definitely some way to not have all this repeated code, but we're prototyping
                OpCode::OpDivide => {
                    let b = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    let a = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    self.stack.push(a / b);
                }
                OpCode::OpMultiply => {
                    let b = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    let a = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    self.stack.push(a * b);
                }
                OpCode::OpAdd => {
                    let b = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    let a = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    self.stack.push(a + b);
                }
                OpCode::OpSubtract => {
                    let b = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    let a = match self.stack.pop() {
                        Some(val) => val,
                        None => return InterpretResult::InterpretCompileError,
                    };

                    self.stack.push(a - b);
                }
            }
        }

        InterpretResult::InterpretOk
    }
}
