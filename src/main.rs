mod chunk;
mod debug;
mod values;
mod vm;

use chunk::Chunk;
use chunk::OpCode;
use vm::VM;

fn main() {
    let vm = VM::init();
    // let mut vm = VM::init();
    // vm.set_debug();

    let mut chunk = Chunk::init();
    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant(constant), 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(OpCode::OpConstant(constant), 123);

    chunk.write(OpCode::OpAdd, 123);

    let add_const = chunk.add_constant(5.6);
    chunk.write(OpCode::OpConstant(add_const), 123);

    chunk.write(OpCode::OpDivide, 123);
    chunk.write(OpCode::OpNegate, 123);

    chunk.write(OpCode::OpReturn, 123);

    println!("{:?}", vm.interpret(chunk));
}
