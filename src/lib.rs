#![feature(try_from)]
#![feature(try_trait)]
#![feature(never_type)]
#![feature(custom_attribute)]
#![no_std]

extern crate failure;
#[macro_use] extern crate failure_derive;

extern crate heapless;

use heapless::Vec;
use heapless::consts::*;

use core::convert::TryFrom;
use core::convert::TryInto;

#[derive(Fail, Debug)]
#[fail(display = "Type Mismatch")]
pub struct TypeMismatchError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Unsigned(u64),
    Signed(i64),
    Bool(bool),
}

impl TryFrom<Object> for u64 {
    type Error = TypeMismatchError;
    
    fn try_from(value: Object) -> Result<u64, TypeMismatchError> {
        match value {
            Object::Unsigned(x) => Ok(x),
            _ => Err(TypeMismatchError{}),
        }
    }
}

impl TryFrom<Object> for i64 {
    type Error = TypeMismatchError;
    
    fn try_from(value: Object) -> Result<i64, TypeMismatchError> {
        match value {
            Object::Signed(x) => Ok(x),
            _ => Err(TypeMismatchError{}),
        }
    }
}

impl TryFrom<Object> for bool {
    type Error = TypeMismatchError;
    
    fn try_from(value: Object) -> Result<bool, TypeMismatchError> {
        match value {
            Object::Bool(x) => Ok(x),
            _ => Err(TypeMismatchError{}),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    LiteralUnsigned(u64),
    LiteralSigned(i64),
    LiteralBool(bool),

    AddUnsigned,
    SubtractUnsigned,
    MultiplyUnsigned,
    DivideUnsigned,
    ModulusUnsigned,

    AddSigned,
    SubtractSigned,
    MultiplySigned,
    DivideSigned,
    ModulusSigned,

    BitAnd,
    BitOr,
    BitNot,
    BitXor,
    BitLShift,
    BitRShift,
    BitLRot,
    BitRRot,

    LogAnd,
    LogOr,
    LogNot,
    LogXor,

    EqUnsigned,
    NeqUnsigned,
    GtUnsigned,
    LtUnsigned,
    GtEqUnsigned,
    LtEqUnsigned,

    EqSigned,
    NeqSigned,
    GtSigned,
    LtSigned,
    GtEqSigned,
    LtEqSigned,

    Branch,
    BranchTrue,

    Call,
    Ret,

    PushSlot1,
    PushSlot2,
    PushSlot3,
    PushSlot4,

    PopSlot1,
    PopSlot2,
    PopSlot3,
    PopSlot4,

    Intrinsic,
}

struct StackUnderflow{}

enum PopFail {
    StackUnderflow,
    TypeError,
}

#[derive(Fail, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HaltReason {
    #[fail(display = "Cycle limit hit")]
    CycleLimit,
    #[fail(display = "Out of bounds IP")]
    OutOfBounds,
    #[fail(display = "Stack Underflow")]
    StackUnderflow,
    #[fail(display = "Stack Overflow")]
    StackOverflow,
    #[fail(display = "Read Uninitialised Scratch Register")]
    EmptyScratch,
    #[fail(display = "Type error")]
    TypeError,
    #[fail(display = "Invalid Intrinsic")]
    InvalidIntrinsic,
}

pub struct Process<'a> {
    ip: usize,
    stack: Vec<Object, U32>,
    callstack: Vec<usize, U32>,
    code: &'a [Instruction],
    scratch: [Option<Object>; 4],
    intrinsics: &'a [fn(&mut Process)]
}

impl From<StackUnderflow> for HaltReason {
    fn from(_x: StackUnderflow) -> HaltReason {
        HaltReason::StackUnderflow
    }
}

impl From<PopFail> for HaltReason {
    fn from(x: PopFail) -> HaltReason {
        match x {
            PopFail::StackUnderflow => HaltReason::StackUnderflow,
            PopFail::TypeError => HaltReason::TypeError,
        }
    }
}

impl<'a> Process<'a> {
    pub fn new(code: &'a [Instruction]) -> Process<'a> {
        Process {
            ip: 0,
            stack: Vec::new(),
            callstack: Vec::new(),
            code: code,
            intrinsics: &[],
            scratch: [None;4]
        }
    }

    pub fn new_with_intrinsics(code: &'a [Instruction], intrinsics: &'a [fn(&mut Process)]) -> Process<'a> {
        Process {
            ip: 0,
            stack: Vec::new(),
            callstack: Vec::new(),
            code: code,
            intrinsics: intrinsics,
            scratch: [None;4]
        }
    }

    pub fn run(&mut self, cycle_limit: u64) -> Result<!, HaltReason> {
        for _ in 0..cycle_limit {
            self.__run_once()?;
        }
        Err(HaltReason::CycleLimit)
    }

    pub fn stack(&'a self) -> &'a [Object] {
        &self.stack
    }

    fn pop(&mut self) -> Result<Object, StackUnderflow> {
        match self.stack.pop() {
            None => Err(StackUnderflow{}),
            Some(x) => Ok(x),
        }
    }

    pub fn pub_pop_as<T>(&mut self) -> Option<T>
        where T: core::convert::TryFrom<Object> {
        match self.stack.pop() {
            None => None,
            Some(x) => {
                let x: Result<T, _> = x.try_into();

                match x {
                    Ok(x) => Some(x),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn pub_push(&mut self, value: Object) -> bool {
        match self.stack.push(value) {
            Ok(()) => true,
            Err(val) => false
        }
    }

    fn pop_as<T>(&mut self) -> Result<T, PopFail>
        where T: core::convert::TryFrom<Object> {
        match self.stack.pop() {
            None => Err(PopFail::StackUnderflow),
            Some(x) => {
                let x: Result<T,_> = x.try_into();

                match x {
                    Ok(x) => Ok(x),
                    Err(_) => Err(PopFail::TypeError),
                }
            }
        }
    }

    fn pop2_as<T>(&mut self) -> Result<(T, T), PopFail>
        where T: core::convert::TryFrom<Object> {
        Ok((self.pop_as()?, self.pop_as()?))
    }

    #[allow(dead_code)]
    fn pop3_as<T>(&mut self) -> Result<(T, T, T), PopFail>
        where T: core::convert::TryFrom<Object> {
        Ok((self.pop_as()?, self.pop_as()?, self.pop_as()?))
    }

    #[inline(always)]
    fn __run_once(&mut self) -> Result<(), HaltReason> {
        use Instruction::*;


        let instruction = *match self.code.get(self.ip) {
            Some(x) => x,
            None => return Err(HaltReason::OutOfBounds),
        };

//        println!("{}: {:?}: {:?}: {:?}", self.ip, instruction, self.stack, self.scratch);

        match instruction {
            LiteralUnsigned(x) => match self.stack.push(Object::Unsigned(x)) {
                Ok(()) => {},
                Err(_) => return Err(HaltReason::StackOverflow),
            },
            LiteralSigned(x) => match self.stack.push(Object::Signed(x)) {
                Ok(()) => {},
                Err(_) => return Err(HaltReason::StackOverflow),
            },
            LiteralBool(x) => match self.stack.push(Object::Bool(x)) {
                Ok(()) => {},
                Err(_) => return Err(HaltReason::StackOverflow),
            },
            AddUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x + y)) {
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            },
            SubtractUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x - y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            },
            MultiplyUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x * y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            },
            DivideUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x / y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            },
            ModulusUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x % y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            },
            AddSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Signed(x + y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            },
            SubtractSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Signed(x - y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            },
            MultiplySigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Signed(x * y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            },
            DivideSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Signed(x / y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            },
            ModulusSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Signed(x % y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            },
            BitAnd => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x & y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            BitOr => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x | y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            BitXor => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x ^ y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            BitNot => {
                let x = self.pop_as::<u64>()?;
                match self.stack.push(Object::Unsigned(!x)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            BitLShift => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x << y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            BitRShift => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x >> y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            BitLRot => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x.rotate_left(y as u32))){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            BitRRot => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Unsigned(x.rotate_right(y as u32))){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            LogAnd => {
                let (y, x) = self.pop2_as::<bool>()?;
                match self.stack.push(Object::Bool(x & y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            LogOr => {
                let (y, x) = self.pop2_as::<bool>()?;
                match self.stack.push(Object::Bool(x | y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            LogNot => {
                let x = self.pop_as::<bool>()?;
                match self.stack.push(Object::Bool(!x)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            LogXor => {
                let (y, x) = self.pop2_as::<bool>()?;
                match self.stack.push(Object::Bool(x ^ y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            EqUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Bool(x == y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            NeqUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Bool(x != y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            GtUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Bool(x > y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            LtUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Bool(x < y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            GtEqUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Bool(x >= y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            LtEqUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                match self.stack.push(Object::Bool(x <= y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            EqSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Bool(x == y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            NeqSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Bool(x != y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            GtSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Bool(x > y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            LtSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Bool(x < y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            GtEqSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Bool(x >= y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            LtEqSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                match self.stack.push(Object::Bool(x <= y)){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            Branch => {
                let x = self.pop_as::<u64>()?;
                self.ip = x as usize;
                return Ok(());
            }
            BranchTrue => {
                let x = self.pop_as::<u64>()?;

                let y = self.pop_as::<bool>()?;
                if y {
                    self.ip = x as usize;
                    return Ok(());
                }
            }
            Call => {
                match self.callstack.push(self.ip){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
                let x = self.pop_as::<u64>()?;
                self.ip = x as usize;
                return Ok(());
            }
            Ret => {
                let ip = match self.callstack.pop() {
                    Some(ip) => ip,
                    None => return Err(HaltReason::StackUnderflow),
                };

                self.ip = ip;
                // Don't return here, we want to increment the instruction pointer.
            }
            PushSlot1 => {
                self.scratch[0] = Some(self.pop()?);
            }
            PushSlot2 => {
                self.scratch[1] = Some(self.pop()?);
            }
            PushSlot3 => {
                self.scratch[2] = Some(self.pop()?);
            }
            PushSlot4 => {
                self.scratch[3] = Some(self.pop()?);
            }
            PopSlot1 => {
                let obj = match self.scratch[0] {
                    Some(obj) => obj,
                    None => return Err(HaltReason::EmptyScratch)
                };
                
                match self.stack.push(obj){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            PopSlot2 => {
                let obj = match self.scratch[1] {
                    Some(obj) => obj,
                    None => return Err(HaltReason::EmptyScratch)
                };
                
                match self.stack.push(obj){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            PopSlot3 => {
                let obj = match self.scratch[2] {
                    Some(obj) => obj,
                    None => return Err(HaltReason::EmptyScratch)
                };
                
                match self.stack.push(obj){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            PopSlot4 => {
                let obj = match self.scratch[3] {
                    Some(obj) => obj,
                    None => return Err(HaltReason::EmptyScratch)
                };
                
                match self.stack.push(obj){
                    Ok(()) => {},
                    Err(_) => return Err(HaltReason::StackOverflow),
                };
            }
            Intrinsic => {
                let idx = self.pop_as::<u64>()?;
                let func = match self.intrinsics.get(idx as usize) {
                    Some(f) => f,
                    None => return Err(HaltReason::InvalidIntrinsic),
                };

                func(self);
            }
        }
        self.ip += 1;

        Ok(())
    }
}
