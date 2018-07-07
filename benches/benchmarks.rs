#![feature(test)]

extern crate test;
extern crate rlang;

use test::{Bencher, black_box};
use std::mem;
use rlang::*;

#[bench]
fn spawn_process(b: &mut Bencher) {
    b.iter(|| {
        let instructions = vec![];
        let mut x = Process::new(&instructions);

        black_box(x);
    });
}

#[bench]
fn simple_addition(b: &mut Bencher) {
    b.iter(|| {
        let instructions = vec![
            Instruction::LiteralUnsigned(13),
            Instruction::LiteralUnsigned(37),
            Instruction::AddUnsigned,
        ];

        let mut x = Process::new(&instructions);

        let r = x.run(64);

        black_box(r);
        black_box(x);
    });
}

#[bench]
fn run_100k_cycles(b: &mut Bencher) {
    b.iter(|| {
        let instructions = vec![
            Instruction::LiteralUnsigned(0),
            Instruction::Branch,
        ];

        let mut x = Process::new(&instructions);

        let r = x.run(100_000);

        black_box(r);
        black_box(x);
    });
}

#[bench]
fn intrinsic(b: &mut Bencher) {
    fn nop(proc: &mut Process) {
        black_box(proc);
    }

    b.iter(|| {
        let instructions = vec![
            Instruction::LiteralUnsigned(0),
            Instruction::Intrinsic,
        ];

        let intrinsics: &[fn(&mut Process)] = &[
            nop
        ];

//        let mut x = Process::new_with_intrinsics(&instructions, &intrinsics);

//        let r = x.run(100_000);

//        black_box(r);
//        black_box(x);
    });
}
