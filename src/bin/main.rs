extern crate rlang;
use rlang::*;

fn main() {
    use rlang::Instruction::*;
    let code = vec![
        
        Instruction::LiteralUnsigned(1071),
        Instruction::LiteralUnsigned(462),

        // A B GCD -> C
        PushSlot2,
        PushSlot1,

        
        PopSlot2,
        PushSlot3,

        PopSlot1,
        PopSlot2,
        ModulusUnsigned,
        PushSlot2,

        PopSlot3,
        PushSlot1,


        PopSlot2,
        LiteralUnsigned(0),
        NeqUnsigned,
        LiteralUnsigned(4),
        BranchTrue,

        PopSlot1

    ];

    let mut x = Process::new(&code);

    println!("{:?}", x.run(1024));

    println!("{:?}", x);
}
