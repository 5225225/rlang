#![feature(test)]

extern crate test;
extern crate rlang;

use test::{Bencher, black_box};
use rlang::*;

#[bench]
fn spawn_process(b: &mut Bencher) {
    b.iter(|| {
        let mut x = Process::new(vec![]);

        black_box(x);
    });
}

#[bench]
fn simple_addition(b: &mut Bencher) {
    b.iter(|| {
        let mut x = Process::new(vec![
            Instruction::LiteralUnsigned(13),
            Instruction::LiteralUnsigned(37),
            Instruction::AddUnsigned,
        ]);

        let r = x.run(64);

        black_box(r);
        black_box(x);
    });
}
