extern crate rlang;
use rlang::*;

#[test]
fn simple_unsigned_addition() {
    let ins = vec![
        Instruction::LiteralUnsigned(1),
        Instruction::LiteralUnsigned(2),
        Instruction::AddUnsigned,
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Unsigned(3)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds));
}

#[test]
fn simple_signed_addition() {
    let ins = vec![
        Instruction::LiteralSigned(1),
        Instruction::LiteralSigned(2),
        Instruction::AddSigned,
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Signed(3)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_unsigned_subtraction() {
    let ins = vec![
        Instruction::LiteralUnsigned(4),
        Instruction::LiteralUnsigned(2),
        Instruction::SubtractUnsigned,
    ];
    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Unsigned(2)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_signed_subtraction() {
    let ins = vec![
        Instruction::LiteralSigned(2),
        Instruction::LiteralSigned(4),
        Instruction::SubtractSigned,
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Signed(-2)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_unsigned_multiplication() {
    let ins = vec![
        Instruction::LiteralUnsigned(4),
        Instruction::LiteralUnsigned(2),
        Instruction::MultiplyUnsigned,
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Unsigned(8)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_signed_multiplication() {
    let ins = vec![
        Instruction::LiteralSigned(-2),
        Instruction::LiteralSigned(4),
        Instruction::MultiplySigned,
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Signed(-8)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_unsigned_division() {
    let ins = vec![
        Instruction::LiteralUnsigned(10),
        Instruction::LiteralUnsigned(2),
        Instruction::DivideUnsigned,
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Unsigned(5)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_signed_division() {
    let ins = vec![
        Instruction::LiteralSigned(-10),
        Instruction::LiteralSigned(2),
        Instruction::DivideSigned,
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Signed(-5)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_unsigned_modulus() {
    let ins = vec![
        Instruction::LiteralUnsigned(101),
        Instruction::LiteralUnsigned(7),
        Instruction::ModulusUnsigned,
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Unsigned(3)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_signed_modulus() {
    let ins = vec![
        Instruction::LiteralSigned(-101),
        Instruction::LiteralSigned(7),
        Instruction::ModulusSigned,
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Signed(-3)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn branch() {
    let ins = vec![
        Instruction::LiteralUnsigned(3),
        Instruction::Branch,
        Instruction::LiteralUnsigned(100),
        Instruction::LiteralUnsigned(50),
        Instruction::LiteralUnsigned(25),
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Unsigned(50), Object::Unsigned(25)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn conditional_branch_true() {
    let ins = vec![
        Instruction::LiteralBool(true),
        Instruction::LiteralUnsigned(4),
        Instruction::BranchTrue,
        Instruction::LiteralUnsigned(100),
        Instruction::LiteralUnsigned(50),
        Instruction::LiteralUnsigned(25),
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Unsigned(50), Object::Unsigned(25)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn conditional_branch_false() {
    let ins = vec![
        Instruction::LiteralBool(false),
        Instruction::LiteralUnsigned(4),
        Instruction::BranchTrue,
        Instruction::LiteralUnsigned(100),
        Instruction::LiteralUnsigned(50),
        Instruction::LiteralUnsigned(25),
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Unsigned(100), Object::Unsigned(50), Object::Unsigned(25)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn calling_incrementer() {
    let ins = vec![
        Instruction::LiteralUnsigned(5),
        Instruction::Branch,
        Instruction::LiteralUnsigned(1),
        Instruction::AddUnsigned,
        Instruction::Ret,
        Instruction::LiteralUnsigned(10),
        Instruction::LiteralUnsigned(2),
        Instruction::Call,
        Instruction::LiteralUnsigned(2),
        Instruction::Call,
    ];

    let mut x = Process::new(&ins);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Unsigned(12)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
#[should_panic(expected = "Called expected_panic from intrinsic")]
fn simple_intrinsic() {

    fn expected_panic(proc: &mut rlang::Process) {
        panic!("Called expected_panic from intrinsic")
    }

    let ins = vec![
        Instruction::LiteralUnsigned(0),
        Instruction::Intrinsic,
    ];

    let intrinsics: &[fn(&mut Process)] = &[
        expected_panic
    ];

    let mut x = Process::new_with_intrinsics(&ins, &intrinsics[..]);

    let ret = x.run(64);

    assert!(false);
}

#[test]
fn intrinsic_mutation() {
    fn triple_top(proc: &mut rlang::Process) {
        let top = proc.pub_pop_as::<u64>().unwrap();
        let new_top = top * 3;
        assert!(proc.pub_push(Object::Unsigned(new_top)));
    }

    let ins = vec![
        Instruction::LiteralUnsigned(13),
        Instruction::LiteralUnsigned(0),
        Instruction::Intrinsic,
    ];

    let intrinsics: &[fn(&mut Process)] = &[
        triple_top
    ];

    let mut x = Process::new_with_intrinsics(&ins, &intrinsics[..]);

    let ret = x.run(64);

    assert_eq!(x.stack(), &*vec![Object::Unsigned(39)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn stack_underflow() {
    let ins = vec![
        Instruction::AddUnsigned,
    ];

    let mut x = Process::new(&ins);

    let reason = x.run(64);

    assert_eq!(reason, Err(HaltReason::StackUnderflow))
}

#[test]
fn type_error() {
    let ins = vec![
        Instruction::LiteralUnsigned(1),
        Instruction::LiteralSigned(1),
        Instruction::AddUnsigned
    ];

    let mut x = Process::new(&ins);

    let reason = x.run(64);

    assert_eq!(reason, Err(HaltReason::TypeError))
}

#[test]
fn empty_scratch() {
    let reason = Process::new(&vec![
        Instruction::PopSlot1,
    ]).run(64);

    assert_eq!(reason, Err(HaltReason::EmptyScratch))
}

#[test]
fn cycle_limit() {
    let reason = Process::new(&vec![
        Instruction::LiteralUnsigned(0),
        Instruction::Branch,
    ]).run(64);

    assert_eq!(reason, Err(HaltReason::CycleLimit))
}

#[test]
fn invalid_intrinsic() {
    let reason = Process::new(&vec![
        Instruction::LiteralUnsigned(0),
        Instruction::Intrinsic,
    ]).run(64);

    assert_eq!(reason, Err(HaltReason::InvalidIntrinsic))
}
