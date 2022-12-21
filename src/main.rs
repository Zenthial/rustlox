mod chunk;
mod debug;
mod values;
mod vm;

use chunk::Chunk;
use chunk::OpCode;
use vm::VM;

fn main() {
    let vm = VM::init();

    let mut chunk = Chunk::init();

    let one_constant = chunk.add_constant(1.);
    chunk.write(OpCode::OpConstant(one_constant), 1);

    let two_constant = chunk.add_constant(2.);
    chunk.write(OpCode::OpConstant(two_constant), 1);

    chunk.write(OpCode::OpAdd, 1);

    let three_constant = chunk.add_constant(3.);
    chunk.write(OpCode::OpConstant(three_constant), 1);

    chunk.write(OpCode::OpMultiply, 1);

    let four_constant = chunk.add_constant(4.);
    chunk.write(OpCode::OpConstant(four_constant), 1);

    chunk.write(OpCode::OpSubtract, 1);

    let five_constant = chunk.add_constant(5.);
    chunk.write(OpCode::OpConstant(five_constant), 1);

    chunk.write(OpCode::OpNegate, 1);
    chunk.write(OpCode::OpDivide, 1);

    chunk.write(OpCode::OpReturn, 1);

    vm.interpret(chunk);
}
