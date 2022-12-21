use crate::{
    chunk::OpCode,
    values::{print_value, ValueArray},
};

// pub fn disassemble_chunk: &Chunk, name: &str) {
//     println!("== {} ==", name);
//     let boxed_= Box::new(chunk);
//     let mut offset = 0;
//     for instruction in &chunk.code {
//         offset = disassemble_instruction(instruction, offset);
//     }
// }

pub fn disassemble_instruction(
    lines: &Vec<i32>,
    constants: &ValueArray,
    instruction: &OpCode,
    offset: usize,
) -> usize {
    print!("{off:0>4} ", off = offset);

    if offset > 0 && lines.get(offset) == lines.get(offset - 1) {
        print!("   | ");
    } else {
        let line_option = lines.get(offset);
        if line_option.is_some() {
            let line = line_option.unwrap();
            print!("{off:>4} ", off = line);
        }
    }

    match instruction {
        OpCode::OpReturn => println!("OP_RETURN"),
        OpCode::OpNegate => println!("OP_NEGATE"),
        OpCode::OpAdd => println!("OP_ADD"),
        OpCode::OpSubtract => println!("OP_SUBTRACT"),
        OpCode::OpMultiply => println!("OP_MULTIPLY"),
        OpCode::OpDivide => println!("OP_DIVIDE"),
        OpCode::OpConstant(index) => {
            print!(
                "OP_CONSTANT {space:>16} {cnst} '",
                space = " ",
                cnst = index
            );
            print_value(constants.get(index));
            println!("'");
            // in the book this is + 2
            // this is because they add the instruction to the array, then add the index of where the constant is after
            // we wrap the index inside the constant, because rust has powerful enums
            // a refactor to consider in the future is to wrap the value of the constant in the enum, and remove the constant array entirely
            return offset + 1;
        }
    }
    return offset + 1;
}
