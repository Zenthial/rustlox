#![allow(dead_code)]

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod values;
mod vm;

use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::io::Result;
use std::process::exit;

use vm::VM;

const DEBUG_PRINT: bool = true;

fn repl(mut vm: VM) -> Result<()> {
    print!("> ");
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        vm.interpret(line? + "\n");
        print!("> ");
    }

    Ok(())
}

fn run_file(mut vm: VM, file_name: &str) -> Result<()> {
    let mut file = File::open(file_name)?;
    println!("{:?} {}", file, file_name);
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let interpret_result = vm.interpret(contents);
    match interpret_result {
        vm::InterpretResult::InterpretOk => Ok(()),
        vm::InterpretResult::InterpretCompileError => {
            eprintln!("{}", "Compile Time Error");
            exit(65);
        }
        vm::InterpretResult::InterpretRuntimeError => {
            eprintln!("{}", "Runtime Time Error");
            exit(70);
        }
    }
}

fn main() -> Result<()> {
    let vm = VM::init();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return repl(vm);
    } else if args.len() == 2 {
        return run_file(vm, args.get(1).unwrap());
    } else {
        eprintln!("Usage: rustlox [path]");
        exit(64);
    }
}

// let mut chunk = Chunk::init();

// let one_constant = chunk.add_constant(1.);
// chunk.write(OpCode::OpConstant(one_constant), 1);

// let two_constant = chunk.add_constant(2.);
// chunk.write(OpCode::OpConstant(two_constant), 1);

// chunk.write(OpCode::OpAdd, 1);

// let three_constant = chunk.add_constant(3.);
// chunk.write(OpCode::OpConstant(three_constant), 1);

// chunk.write(OpCode::OpMultiply, 1);

// let four_constant = chunk.add_constant(4.);
// chunk.write(OpCode::OpConstant(four_constant), 1);

// chunk.write(OpCode::OpSubtract, 1);

// let five_constant = chunk.add_constant(5.);
// chunk.write(OpCode::OpConstant(five_constant), 1);

// chunk.write(OpCode::OpNegate, 1);
// chunk.write(OpCode::OpDivide, 1);

// chunk.write(OpCode::OpReturn, 1);
