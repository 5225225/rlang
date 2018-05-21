#![feature(try_from)]
#![feature(try_trait)]
#![feature(never_type)]

#[macro_use] extern crate failure;

use std::convert::TryFrom;
use std::convert::TryInto;

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

    PushSlot1,
    PushSlot2,
    PushSlot3,
    PushSlot4,

    PopSlot1,
    PopSlot2,
    PopSlot3,
    PopSlot4,
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
    #[fail(display = "Read Uninitialised Scratch Register")]
    EmptyScratch,
    #[fail(display = "Type error")]
    TypeError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Process {
    ip: usize,
    stack: Vec<Object>,
    code: Vec<Instruction>,
    scratch: [Option<Object>; 4]
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

impl Process {
    pub fn new(code: Vec<Instruction>) -> Process {
        Process {
            ip: 0,
            stack: Vec::new(),
            code: code,
            scratch: [None;4]
        }
    }

    pub fn run(&mut self, cycle_limit: u64) -> Result<!, HaltReason> {
        for _ in 0..cycle_limit {
            self.__run_once()?;
        }
        Err(HaltReason::CycleLimit)
    }

    pub fn stack(&self) -> &Vec<Object> {
        &self.stack
    }

    fn pop(&mut self) -> Result<Object, StackUnderflow> {
        match self.stack.pop() {
            None => Err(StackUnderflow{}),
            Some(x) => Ok(x),
        }
    }

    fn pop_as<T>(&mut self) -> Result<T, PopFail>
        where T: std::convert::TryFrom<Object> {
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
        where T: std::convert::TryFrom<Object> {
        Ok((self.pop_as()?, self.pop_as()?))
    }

    #[allow(dead_code)]
    fn pop3_as<T>(&mut self) -> Result<(T, T, T), PopFail>
        where T: std::convert::TryFrom<Object> {
        Ok((self.pop_as()?, self.pop_as()?, self.pop_as()?))
    }

    #[inline(always)]
    fn __run_once(&mut self) -> Result<(), HaltReason> {
        use Instruction::*;

        let instruction = *match self.code.get(self.ip) {
            Some(x) => x,
            None => return Err(HaltReason::OutOfBounds),
        };

        match instruction {
            LiteralUnsigned(x) => self.stack.push(Object::Unsigned(x)),
            LiteralSigned(x) => self.stack.push(Object::Signed(x)),
            LiteralBool(x) => self.stack.push(Object::Bool(x)),
            AddUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x + y));
            },
            SubtractUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x - y));
            },
            MultiplyUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x * y));
            },
            DivideUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x / y));
            },
            ModulusUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x % y));
            },
            AddSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Signed(x + y));
            },
            SubtractSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Signed(x - y));
            },
            MultiplySigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Signed(x * y));
            },
            DivideSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Signed(x / y));
            },
            ModulusSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Signed(x % y));
            },
            BitAnd => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x & y));
            }
            BitOr => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x | y));
            }
            BitXor => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x ^ y));
            }
            BitNot => {
                let x = self.pop_as::<u64>()?;
                self.stack.push(Object::Unsigned(!x));
            }
            BitLShift => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x << y));
            }
            BitRShift => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x >> y));
            }
            BitLRot => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x.rotate_left(y as u32)));
            }
            BitRRot => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Unsigned(x.rotate_right(y as u32)));
            }
            LogAnd => {
                let (y, x) = self.pop2_as::<bool>()?;
                self.stack.push(Object::Bool(x & y));
            }
            LogOr => {
                let (y, x) = self.pop2_as::<bool>()?;
                self.stack.push(Object::Bool(x | y));
            }
            LogNot => {
                let x = self.pop_as::<bool>()?;
                self.stack.push(Object::Bool(!x));
            }
            LogXor => {
                let (y, x) = self.pop2_as::<bool>()?;
                self.stack.push(Object::Bool(x ^ y));
            }
            EqUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Bool(x == y));
            }
            NeqUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Bool(x != y));
            }
            GtUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Bool(x > y));
            }
            LtUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Bool(x < y));
            }
            GtEqUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Bool(x >= y));
            }
            LtEqUnsigned => {
                let (y, x) = self.pop2_as::<u64>()?;
                self.stack.push(Object::Bool(x <= y));
            }
            EqSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Bool(x == y));
            }
            NeqSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Bool(x != y));
            }
            GtSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Bool(x > y));
            }
            LtSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Bool(x < y));
            }
            GtEqSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Bool(x >= y));
            }
            LtEqSigned => {
                let (y, x) = self.pop2_as::<i64>()?;
                self.stack.push(Object::Bool(x <= y));
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
                
                self.stack.push(obj);
            }
            PopSlot2 => {
                let obj = match self.scratch[1] {
                    Some(obj) => obj,
                    None => return Err(HaltReason::EmptyScratch)
                };
                
                self.stack.push(obj);
            }
            PopSlot3 => {
                let obj = match self.scratch[2] {
                    Some(obj) => obj,
                    None => return Err(HaltReason::EmptyScratch)
                };
                
                self.stack.push(obj);
            }
            PopSlot4 => {
                let obj = match self.scratch[3] {
                    Some(obj) => obj,
                    None => return Err(HaltReason::EmptyScratch)
                };
                
                self.stack.push(obj);
            }
        }
        self.ip += 1;

        Ok(())
    }
}
