extern crate rlang;
use rlang::*;

#[test]
fn simple_unsigned_addition() {
    let mut x = Process::new(vec![
        Instruction::LiteralUnsigned(1),
        Instruction::LiteralUnsigned(2),
        Instruction::AddUnsigned,
    ]);

    let ret = x.run(64);

    assert_eq!(x.stack(), &vec![Object::Unsigned(3)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_signed_addition() {
    let mut x = Process::new(vec![
        Instruction::LiteralSigned(1),
        Instruction::LiteralSigned(2),
        Instruction::AddSigned,
    ]);

    let ret = x.run(64);

    assert_eq!(x.stack(), &vec![Object::Signed(3)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_unsigned_subtraction() {
    let mut x = Process::new(vec![
        Instruction::LiteralUnsigned(4),
        Instruction::LiteralUnsigned(2),
        Instruction::SubtractUnsigned,
    ]);

    let ret = x.run(64);

    assert_eq!(x.stack(), &vec![Object::Unsigned(2)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_signed_subtraction() {
    let mut x = Process::new(vec![
        Instruction::LiteralSigned(2),
        Instruction::LiteralSigned(4),
        Instruction::SubtractSigned,
    ]);

    let ret = x.run(64);

    assert_eq!(x.stack(), &vec![Object::Signed(-2)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_unsigned_multiplication() {
    let mut x = Process::new(vec![
        Instruction::LiteralUnsigned(4),
        Instruction::LiteralUnsigned(2),
        Instruction::MultiplyUnsigned,
    ]);

    let ret = x.run(64);

    assert_eq!(x.stack(), &vec![Object::Unsigned(8)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_signed_multiplication() {
    let mut x = Process::new(vec![
        Instruction::LiteralSigned(-2),
        Instruction::LiteralSigned(4),
        Instruction::MultiplySigned,
    ]);

    let ret = x.run(64);

    assert_eq!(x.stack(), &vec![Object::Signed(-8)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_unsigned_division() {
    let mut x = Process::new(vec![
        Instruction::LiteralUnsigned(10),
        Instruction::LiteralUnsigned(2),
        Instruction::DivideUnsigned,
    ]);

    let ret = x.run(64);

    assert_eq!(x.stack(), &vec![Object::Unsigned(5)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn simple_signed_division() {
    let mut x = Process::new(vec![
        Instruction::LiteralSigned(-10),
        Instruction::LiteralSigned(2),
        Instruction::DivideSigned,
    ]);

    let ret = x.run(64);

    assert_eq!(x.stack(), &vec![Object::Signed(-5)]);
    assert_eq!(ret, Err(HaltReason::OutOfBounds))
}

#[test]
fn stack_underflow() {
    let mut x = Process::new(vec![
        Instruction::AddUnsigned,
    ]);

    let reason = x.run(64);

    assert_eq!(reason, Err(HaltReason::StackUnderflow))
}

#[test]
fn type_error() {
    let mut x = Process::new(vec![
        Instruction::LiteralUnsigned(1),
        Instruction::LiteralSigned(1),
        Instruction::AddUnsigned
    ]);

    let reason = x.run(64);

    assert_eq!(reason, Err(HaltReason::TypeError))
}

#[test]
fn empty_scratch() {
    let reason = Process::new(vec![
        Instruction::PopSlot1,
    ]).run(64);

    assert_eq!(reason, Err(HaltReason::EmptyScratch))
}

#[test]
fn cycle_limit() {
    let reason = Process::new(vec![
        Instruction::LiteralUnsigned(0),
        Instruction::Branch,
    ]).run(64);

    assert_eq!(reason, Err(HaltReason::CycleLimit))
}
