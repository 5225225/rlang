use std::mem;

extern crate rlang;
use rlang::*;

fn main() {
    let code = vec![
        Instruction::LiteralUnsigned(3),
        Instruction::LiteralUnsigned(5),
        Instruction::AddUnsigned,
        Instruction::LiteralUnsigned(0),
        Instruction::Branch,
    ];

    let mut x = Process::new(code);

    println!("{:?}", x.run(16));

    println!("{:?}", x);

    println!("{}", mem::size_of::<Process>());
    println!("{}", mem::size_of::<Option<Object>>());


}
