use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Write};
use std::{fmt, mem};

use crate::bfasm::binterp::{BFError, BFInterpreter, BFOp};
mod binterp;

use Type::EmptyCell as EC;
// use Type as T;
use EmptyType::EmptyCell as EEC;

use crate::bfasm::BfasmError::TypeMismatch;

macro_rules! label {
    ($dst:expr, $($arg:tt)*) => {
        write!($dst, $($arg)*).unwrap();

        // if let BfasmWriter::BFInterp(binterp, _) = &mut $dst {
        //     binterp.instructions.push(BFOp::Lable)
        // }

        $dst.as_mut_bfops().push(BFOp::Lable)
    };
}


// https://minond.xyz/brainfuck/ was used for testing code when it broke

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    U32(u32),
    I32(i32),
    Bool(bool),
    Char(u8),
    FString(Vec<u8>),
    IString(Vec<u8>),
    Array(Vec<u32>),
    EmptyCell,
}

impl Type {
    fn empty_slice(length: usize) -> Vec<Type> {
        (0..length).map(|_| Type::EmptyCell).collect()
    }

    // fn len(&self) -> usize {
    //     match self {
    //         Type::U32(_) => 1,
    //         Type::I32(_) => 2,
    //         Type::Bool(_) => 1,
    //         Type::Char(_) => 1,
    //         Type::FString(val) | Type::IString(val) => val.len() * 2 + 4,
    //         Type::EmptyCell => 1,
    //         Type::Array(val) => val.len() * 2 + 4,
    //     }
    // }

    fn len_slice(slice: &[Type]) -> usize {
        slice.iter().map(|x| {
            match x {
                Type::U32(_) => 1,
                Type::I32(_) => 2,
                Type::Bool(_) => 1,
                Type::Char(_) => 1,
                Type::FString(val) | Type::IString(val) => val.len() * 2 + 4,
                Type::EmptyCell => 1,
                Type::Array(val) => val.len() * 2 + 4,
            }
        }).sum()
    }
}

impl From<u32> for Type {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<i32> for Type {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<bool> for Type {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<char> for Type {
    fn from(value: char) -> Self {
        assert!(value.is_ascii());

        Self::Char(value as u8)
    }
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        Type::from(value.as_bytes())
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        Type::from(value.as_bytes())
    }
}

impl From<&[u8]> for Type {
    fn from(value: &[u8]) -> Self {
        assert!(value.is_ascii(), "String contained non Ascii values");

        assert!(
            !value.iter().any(|x| x == &0),
            "String contained null bytes"
        );

        Self::FString(Vec::from(value))
    }
}

impl From<Vec<u32>> for Type {
    fn from(value: Vec<u32>) -> Self {
        Type::Array(value)
    }
}

// impl From<&Type> for Vec<u32> {
//     fn from(bf_type: &Type) -> Self {
//         match bf_type {
//             Type::U32(x) => {
//                 vec![*x]
//             }
//             Type::I32(x) => {
//                 vec![x.is_negative() as u32, x.unsigned_abs()]
//             }
//             Type::Bool(x) => {
//                 vec![*x as u32]
//             }
//             Type::Char(x) => {
//                 vec![*x as u32]
//             }
//             Type::FString(x) | Type::IString(x) => [
//                 vec![0_u32, 0_u32],
//                 x.iter()
//                     .rev()
//                     .flat_map(|char| [*char as u32, 0_u32])
//                     .collect(),
//                 vec![0_u32, x.len() as u32],
//             ]
//             .into_iter()
//             .flatten()
//             .collect(),
//             Type::Array(x) => [
//                 vec![0_u32, 0_u32],
//                 x.iter().flat_map(|val| [*val + 1, 0_u32]).collect(),
//                 vec![0_u32, x.len() as u32],
//             ]
//             .into_iter()
//             .flatten()
//             .collect(),
//             Type::EmptyCell => {
//                 vec![0]
//             }
//         }
//     }
// }

impl From<&Type> for String {
    fn from(value: &Type) -> Self {
        match value {
            Type::U32(val) => (*val).to_string(),
            Type::I32(val) => (*val).to_string(),
            Type::Bool(val) => String::from(if *val { 't' } else { 'f' }),
            Type::Char(val) => (*val).to_string(),
            Type::FString(val) => String::from_utf8(val.clone()).unwrap(),
            Type::IString(val) => String::from_utf8(val.clone()).unwrap(),
            Type::EmptyCell | Type::Array(_) => {
                unimplemented!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmptyType {
    U32,
    I32,
    Bool,
    Char,
    FString,
    IString,
    EmptyCell,
    Any,
    Array,
}

impl EmptyType {
    pub fn from_vec(array: &[Type]) -> Vec<EmptyType> {
        array.iter().map(EmptyType::from).collect()
    }
}

impl From<&Type> for EmptyType {
    fn from(value: &Type) -> Self {
        match value {
            Type::U32(_) => EmptyType::U32,
            Type::I32(_) => EmptyType::I32,
            Type::Bool(_) => EmptyType::Bool,
            Type::Char(_) => EmptyType::Char,
            Type::FString(_) => EmptyType::FString,
            Type::IString(_) => EmptyType::IString,
            Type::Array(_) => EmptyType::Array,
            Type::EmptyCell => EmptyType::EmptyCell,
        }
    }
}

// If a non type error is thrown the array types should still be changed and filled with dummy vals maybe options?
#[derive(Debug, Clone)]
pub enum BfasmError {
    // type errors
    TypeMismatch(Vec<EmptyType>, Vec<Type>),
    InvalidMatchArm(usize),

    // value errors
    OpError(OpError),
}

#[derive(Debug, Clone)]
pub enum OpError {
    InvalidStringIndex(usize),
    ErrorsInMatch(Vec<OpError>),
    Underflow,
}

impl Display for BfasmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeMismatch(expected, found) => {
                write!(
                    f,
                    "Type Mismatch: Expected: {:?} Found: {:?}",
                    expected, found
                )
            }
            // BfasmError::InvalidIndex(index) => write!(f, "Invalid array index of {index}"),
            BfasmError::OpError(OpError::InvalidStringIndex(index)) => write!(f, "Invalid string index of {index}"),
            BfasmError::InvalidMatchArm(index) => {
                write!(f, "Invalid match arm {index} with mismatching array types")
            }
            BfasmError::OpError(OpError::Underflow) => {
                write!(f, "Underflow")
            }
            BfasmError::OpError(OpError::ErrorsInMatch(err)) => {
                write!(f, "Error(s) inside block with the first as {:?}", err[0])
            }
        }
    }
}

impl std::error::Error for BfasmError {}

#[derive(Clone, Debug)]
pub enum BfasmOps {
    Set(usize, Type),
    MoveTo(usize),
    MoveType(usize, usize),
    Clear(usize),
    CopyVal(usize),
    I32Add(usize),
    Input(usize, Type),
    StrIndex(usize),
    Print(usize),
    StrPushF(usize),
    StrPush(usize),
    ArrayPush(usize),
    ArrayPushF(usize),
    ArrayIndexF(usize),
    ArrayIndex(usize),
    ArraySet(usize),
    Len(usize),
    U32Add(usize),
    U32SubUnchecked(usize),
    InsertEC(usize, usize),
    CharMatch(usize, Vec<(u8, Vec<BfasmOps>)>),
    BoolIf(usize, Vec<BfasmOps>),
    BoolWhile(usize, Vec<BfasmOps>),
    GreaterThan(usize),
    LessThan(usize),
    Equals(usize),
    CharToU32(usize),
}

impl BfasmOps {
    pub fn exec_instruct(&self, bfasm: &mut Bfasm) -> Result<(), BfasmError> {
        let res = match self {
            BfasmOps::Set(index, bftype) => bfasm.set(*index, bftype.clone()),
            BfasmOps::MoveTo(index) => {
                bfasm.move_to(*index);
                Ok(())
            }
            BfasmOps::MoveType(index, targetindex) => bfasm.move_type(*index, *targetindex),
            BfasmOps::Clear(index) => {
                bfasm.clear(*index);
                Ok(())
            }
            BfasmOps::CopyVal(index) => bfasm.copy_val(*index),
            BfasmOps::I32Add(index) => bfasm.add_i32(*index),
            BfasmOps::Input(index, bftype) => bfasm.input(*index, bftype.clone()),
            BfasmOps::StrIndex(index) => bfasm.index_str(*index),
            BfasmOps::Print(index) => bfasm.print(*index),
            BfasmOps::StrPushF(index) => bfasm.str_push_front(*index),
            BfasmOps::StrPush(index) => bfasm.str_push(*index),
            BfasmOps::ArrayPush(index) => bfasm.array_push(*index),
            BfasmOps::ArrayPushF(index) => bfasm.array_push_front(*index),
            BfasmOps::ArrayIndexF(index) => bfasm.array_index(*index),
            BfasmOps::ArrayIndex(index) => bfasm.array_index_back(*index),
            BfasmOps::ArraySet(index) => bfasm.array_set_back(*index),
            BfasmOps::Len(index) => bfasm.get_len(*index),
            BfasmOps::U32Add(index) => bfasm.add_u32(*index),
            BfasmOps::U32SubUnchecked(index) => bfasm.unsafe_sub_u32(*index),
            BfasmOps::InsertEC(index, num) => bfasm.insert_ec(*index, *num),
            BfasmOps::CharMatch(index, arms) => bfasm.match_char(*index, arms),
            BfasmOps::BoolIf(index, code) => bfasm.bool_if(*index, code),
            BfasmOps::BoolWhile(index, code) => bfasm.bool_while(*index, code),
            BfasmOps::GreaterThan(index) => bfasm.greater_than(*index),
            BfasmOps::LessThan(index) => bfasm.less_than(*index),
            BfasmOps::Equals(index) => bfasm.equals(*index),
            BfasmOps::CharToU32(index) => bfasm.char_to_u32(*index),
        };

        if let BfasmWriter::BFInterp(binterp, _) = &mut bfasm.output {
            binterp.input = bfasm.expected_input.clone();

            dbg!(self, &bfasm.array, bfasm.index);

            binterp.label_run().unwrap();

            let interp = mem::take(binterp);

            assert!(bfasm.cmp_to_interp(&interp));

            let BfasmWriter::BFInterp(binterp, _) = &mut bfasm.output else {unreachable!()};

            let _ = mem::replace(binterp, interp);
        }

        res
    }

    pub fn exec(code: &[BfasmOps], bfasm: &mut Bfasm) -> Result<(), (usize, BfasmError)> {

        for (index, oper) in code.iter().enumerate() {
            oper.exec_instruct(bfasm).map_err(|err| (index, err))?;
        }

        Ok(())
    }

    pub fn full_exec(code: &[BfasmOps], bfasm: &mut Bfasm) -> Result<Option<Vec<OpError>>, BfasmError> {

        // let errs: Vec<OpError> = code.iter().filter_map(|oper| {
        //     match oper.exec_instruct(bfasm) {
        //         Ok(()) => {None}
        //         Err(BfasmError::OpError(err)) => {Some(err)}
        //         Err(x) => Err(x)?,
        //     }
        // }).collect();

        let mut errs = Vec::new();

        for oper in code {
            match oper.exec_instruct(bfasm) {
                Ok(()) => {}
                Err(BfasmError::OpError(err)) => {errs.push(err)}
                Err(x) => {return Err(x);},
            }
        }

        if errs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(errs))
        }
    }
}

#[derive(Debug, Clone)]
pub enum BfasmWriter {
    BFOps(Vec<BFOp>, bool),
    BFInterp(BFInterpreter, bool),
}

// make it look good before use
// use debug fmt else
// impl Display for BfasmWriter {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//
//         f.write_str(&BFOp::as_str(self.as_bfops()))
//
//     }
// }

impl BfasmWriter {
    fn code(&mut self, s: &str) {
        let program = match self {
            BfasmWriter::BFOps(program, true) => { program }
            BfasmWriter::BFInterp(binterp, true) => { &mut binterp.instructions }
            // BfasmWriter::None => {}
            _ => { return; }
        };

        for char in s.chars() {
            match char {
                '+' => program.push(BFOp::Plus),
                '-' => program.push(BFOp::Minus),
                '<' => program.push(BFOp::Left),
                '>' => program.push(BFOp::Right),
                ',' => program.push(BFOp::Comma),
                '.' => program.push(BFOp::Period),
                '[' => program.push(BFOp::OpenBracket),
                ']' => program.push(BFOp::CloseBracket),
                _ => {}
            }
        }
    }

    fn extend(&mut self, code: Vec<BFOp>) {

        // let bfops = self.as_bfops();
        // code.iter().for_each(|x| bfops.push(x.clone()))

        self.as_mut_bfops().extend(code);

    }

    // fn push(&mut self, s: char) {
    //     match self {
    //         BfasmWriter::String(str, true) => {str.push(s)}
    //         BfasmWriter::BFInterp(binterp, true) => {
    //             match s {
    //                 '+' => binterp.instructions.push(BFOp::Plus),
    //                 '-' => binterp.instructions.push(BFOp::Minus),
    //                 '<' => binterp.instructions.push(BFOp::Left),
    //                 '>' => binterp.instructions.push(BFOp::Right),
    //                 ',' => binterp.instructions.push(BFOp::Comma),
    //                 '.' => binterp.instructions.push(BFOp::Period),
    //                 '[' => binterp.instructions.push(BFOp::OpenBracket),
    //                 ']' => binterp.instructions.push(BFOp::CloseBracket),
    //                 _ => {}
    //             };
    //         }
    //         // BfasmWriter::None => {}
    //         _ => {}
    //     }
    // }

    // TODO: doesnt have to be an option
    fn as_bfops(&self) -> &Vec<BFOp> {
        match self {
            BfasmWriter::BFOps(str, _) => {str}
            BfasmWriter::BFInterp(binterp, _) => {&binterp.instructions}
            // BfasmWriter::None => {None}
        }
    }
    fn as_mut_bfops(&mut self) -> &mut Vec<BFOp> {
        match self {
            BfasmWriter::BFOps(str, _) => {str}
            BfasmWriter::BFInterp(binterp, _) => {&mut binterp.instructions}
            // BfasmWriter::None => {None}
        }
    }

    fn is_enabled(&self) -> bool {
        match self {
            BfasmWriter::BFOps(_, b)| BfasmWriter::BFInterp(_, b) => {*b}
        }
    }

    fn enabled(&mut self, val: bool) {
        match self {
            BfasmWriter::BFOps(_, x) | BfasmWriter::BFInterp(_, x) => {*x = val}
        }
    }
}

impl Write for BfasmWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.code(s);
        Ok(())
    }
}

// TODO make a doc comment
// if the pointer is at a type it will be at the first cell of it
// if a execution errs it should err as late as possible
#[derive(Debug, Clone)]
pub struct Bfasm {
    pub array: Vec<Type>,
    pub output: BfasmWriter,
    pub index: usize,
    pub expected_input: String,
    pub expected_output: String,
}

// pub type BfasmCode = Vec<Box<dyn Fn(&mut Bfasm) -> Result<(), BfasmError>>>;

// impl From<&Bfasm> for Vec<u32> {
//     fn from(bunf: &Bfasm) -> Self {
//         bunf.array
//             .iter()
//             .flat_map(<&Type as Into<Vec<u32>>>::into)
//             .collect()
//     }
// }

impl Default for Bfasm {
    fn default() -> Self {
        // Self::new(BfasmWriter::String(String::new()))
        Self::new(BfasmWriter::BFInterp(BFInterpreter::default(), true))
    }
}

impl Bfasm {
    pub fn new(output: BfasmWriter) -> Self {
        Self {
            array: vec![],
            output,
            index: 0,
            expected_input: String::new(),
            expected_output: String::new(),
        }
    }

    pub fn test_run(&mut self) -> Result<bool, BFError> {

        let mut interp = BFInterpreter::new(self.output.as_bfops().clone(), self.expected_input.chars().collect());

        interp.run()?;

        // println!("Found output: {}", output);
        println!("Expected output: \"{}\"", self.expected_output);

        Ok(self.cmp_to_interp(&interp))
    }

    fn cmp_to_interp(&mut self, interp: &BFInterpreter) -> bool {

        self.get(self.index);

        let expected_index = dbg!(Type::len_slice(dbg!(&self.array[0..self.index])));
        dbg!(self.index, interp.array_index);

        // cmp the array, output, and pointer

        let mut index = 0;

        // dbg!(&self.array);

        for item in &self.array {

            if !match item {
                Type::U32(x) => {
                    let res = x == interp.array.get(index).unwrap_or(&0);
                    index += 1;
                    res
                }
                Type::I32(x) => {
                    let mut res = {
                        if *x != 0 {

                            x.is_positive() == (*interp.array.get(index).unwrap_or(&0) == 0)

                        } else {
                            true
                        }
                    };

                    res &= x.unsigned_abs() == *interp.array.get(index+1).unwrap_or(&0);

                    index += 2;

                    res
                }
                Type::Bool(x) => {
                    let res = *interp.array.get(index).unwrap_or(&0) == *x as u32;

                    index += 1;

                    res
                }
                Type::Char(x) => {
                    let res = *interp.array.get(index).unwrap_or(&0) == *x as u32;

                    index += 1;

                    res
                }
                Type::FString(x) | Type::IString(x) => {
                    let mut res = *interp.array.get(index).unwrap_or(&0) == 0;

                    res &= *interp.array.get(index+1).unwrap_or(&0) == 0;

                    index += 2;

                    for val in x.iter().rev() {
                        res &= *interp.array.get(index).unwrap_or(&0) == *val as u32;
                        res &= *interp.array.get(index+1).unwrap_or(&0) == 0;
                        index += 2;
                    }

                    res &= *interp.array.get(index).unwrap_or(&0) == 0;
                    res &= *interp.array.get(index+1).unwrap_or(&0) == x.len() as u32;

                    index += 2;

                    res
                }
                Type::Array(x) => {
                    let mut res = *interp.array.get(index).unwrap_or(&0) == 0;

                    res &= *interp.array.get(index+1).unwrap_or(&0) == 0;

                    index += 2;

                    for val in x.iter() {
                        res &= *interp.array.get(index).unwrap_or(&0) == (*val + 1) ;
                        res &= *interp.array.get(index+1).unwrap_or(&0) == 0;
                        index += 2;
                    }

                    res &= *interp.array.get(index).unwrap_or(&0) == 0;
                    res &= *interp.array.get(index+1).unwrap_or(&0) == x.len() as u32;

                    index += 2;

                    res
                }
                Type::EmptyCell => {
                    let res = interp.array.get(index).unwrap_or(&0) == &0;

                    index += 1;

                    res
                }
            } {
                dbg!(item, index);
                return false;
            }

        };

        // make sure x did miss any values
        if index < interp.array.len() {
            if !interp.array[index..].iter().all(|x| *x == 0){
                dbg!();
                return false;
            }
        }

        dbg!(interp.array_index == expected_index) && dbg!(interp.output == self.expected_output)
    }

    fn get_slice(&mut self, index: usize, length: usize) -> &mut [Type] {
        // loop{
        //     if let Some(x) = self.array.get(index..index+length){
        //         return  x;
        //     };
        //
        //     self.array.push(Type::EmptyCell);
        // };

        // while index + length < self.array.len(){self.array.push(Type::EmptyCell);};

        while index + length > self.array.len() {
            self.array.push(Type::EmptyCell);
        }

        self.array.get_mut(index..index + length).unwrap()
    }

    // fn get_empty(&mut self, index: usize, length: usize) -> Vec<EmptyType> {
    //
    //     while index + length < self.array.len() {self.array.push(Type::EmptyCell);};
    //
    //     EmptyType::from_vec(self.array.get(index .. index+length).unwrap())
    // }

    fn get(&mut self, index: usize) -> &mut Type {
        while self.array.len() <= index {
            self.array.push(Type::EmptyCell);
        }

        self.array.get_mut(index).unwrap()
    }

    fn trim_ec(&mut self) {

        todo!();

        while let Some(&Type::EmptyCell) = self.array.last() {

            self.array.pop();

        }
    }

    fn move_to(&mut self, expected_index: usize) {
        let str = self.traverse(self.index, expected_index);

        self.output.code(&str);

        self.index = expected_index;
    }

    fn traverse(&mut self, mut index: usize, goal: usize) -> String {
        let mut output = String::new();

        while let order @ (Ordering::Greater | Ordering::Less) = index.cmp(&goal) {
            match order {
                Ordering::Less => {
                    let str = match self.get(index) {
                        Type::U32(_) | Type::Bool(_) | Type::Char(_) | Type::EmptyCell => ">",
                        Type::I32(_) => ">>",
                        Type::FString(_) | Type::IString(_) | Type::Array(_) => ">>[>>]>>",
                    };

                    output.push_str(str);

                    index += 1;
                }
                Ordering::Greater => {
                    let str = match self.get(index - 1) {
                        Type::U32(_) | Type::Bool(_) | Type::Char(_) | Type::EmptyCell => "<",
                        Type::I32(_) => "<<",
                        Type::FString(_) | Type::IString(_) | Type::Array(_) => "<<<<[<<]",
                    };

                    output.push_str(str);

                    index -= 1;
                }
                Ordering::Equal => {
                    unreachable!()
                }
            }
        }

        output
    }

    pub fn set(&mut self, index: usize, item: Type) -> Result<(), BfasmError> {

        label!(self.output, "Setting at {}\n", index);
        // write!(self.output, "Setting {} to {:?}\n", index, item).unwrap();

        self.move_to(index);

        match item {
            Type::U32(val) => {
                let x = self.get_slice(index, 1);
                if x == [EC] {
                    self.array[index] = Type::U32(val);
                    writeln!(self.output, "{}>", "+".repeat(val as usize)).unwrap();
                } else {
                    return Err(TypeMismatch(vec![EEC], Vec::from(x)));
                }
            }
            Type::I32(val) => {
                let x = self.get_slice(index, 2);
                if x == [EC, EC] {
                    self.array.remove(index);
                    self.array[index] = Type::I32(val);
                    writeln!(self.output,
                        "{}{}>",
                        if val.is_negative() { "+>" } else { ">" },
                        "+".repeat(val.unsigned_abs() as usize)
                    ).unwrap();
                } else {
                    return Err(TypeMismatch(vec![EEC, EEC], Vec::from(x)));
                }
            }
            Type::Bool(val) => {
                let x = self.get_slice(index, 1);
                if x == [EC] {
                    self.array[index] = Type::Bool(val);
                    self.output.code(if val { "+>\n" } else { ">\n" });
                } else {
                    return Err(TypeMismatch(vec![EEC], Vec::from(x)));
                }
            }
            Type::Char(val) => {
                let x = self.get_slice(index, 1);
                if x == [EC] {
                    self.array[index] = Type::Char(val);
                    writeln!(self.output, "{}>", "+".repeat(val as usize)).unwrap();
                } else {
                    return Err(TypeMismatch(vec![EEC], Vec::from(x)));
                }
            }
            Type::FString(ref str) | Type::IString(ref str) => {
                let len = str.len() * 2 + 4;
                let slice = self.get_slice(index, len);
                let expected = (0..len).map(|_| EC).collect::<Vec<Type>>();

                if slice == expected {
                    str.iter().rev().for_each(|char| {
                        write!(self.output, ">>{}", "+".repeat(*char as usize)).unwrap()
                    });
                    writeln!(self.output, ">>>{}>", "+".repeat(str.len())).unwrap();
                    (0..len).for_each(|_| {
                        self.array.remove(index);
                    });
                    self.array.insert(index, item);
                } else {
                    return Err(TypeMismatch(
                        expected.iter().map(|_| EEC).collect(),
                        Vec::from(slice),
                    ));
                }
            }

            Type::Array(ref array) => {
                let len = array.len() * 2 + 4;
                let slice = self.get_slice(index, len);
                let expected = (0..len).map(|_| EC).collect::<Vec<Type>>();

                if slice == expected {
                    // self.output.push_str(&format!(
                    //     "{}>>>{}>\n",
                    //     array.iter()
                    //         .map(|x| format!(">>{}", "+".repeat(*x as usize + 1)))
                    //         .collect::<String>(), // add each char
                    //     "+".repeat(array.len())
                    // ));
                    array.iter().for_each(|x| {
                        write!(self.output, ">>{}", "+".repeat(*x as usize + 1)).unwrap()
                    });
                    writeln!(self.output, ">>>{}>", "+".repeat(array.len())).unwrap();
                    (0..len).for_each(|_| {
                        self.array.remove(index);
                    });
                    self.array.insert(index, item);
                } else {
                    return Err(TypeMismatch(
                        expected.iter().map(|_| EEC).collect(),
                        Vec::from(slice),
                    ));
                }
            }

            Type::EmptyCell => {
                panic!("why?")
            }
        };

        self.index += 1;

        Ok(())
    }

    // Doesn't actually do anything in BF just for BFASM use
    pub fn char_to_u32(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Changing char at {} to u32\n", index);

        let slice = self.get(index);

        if let cell @ Type::Char(_) = slice {
            let Type::Char(x) = *cell else { unreachable!() };

            *cell = Type::U32(x as u32);

            Ok(())
        } else {
            Err(TypeMismatch(vec![EmptyType::Char], vec![slice.clone()]))
        }
    }

    // Todo Test
    pub fn move_type(&mut self, index: usize, target_index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Moving {} to {}\n", index, target_index);

        self.move_to(index);

        if *self.get(target_index) != Type::EmptyCell {
            return Err(TypeMismatch(
                vec![EEC],
                vec![self.get(target_index).clone()],
            ));
        }

        let target = self.get(index);

        match target {
            bftype @ (Type::U32(_) | Type::Bool(_) | Type::Char(_)) => {
                let move_val = bftype.clone();

                *bftype = EC;

                self.array[target_index] = move_val;

                let to_target = self.traverse(index, target_index);
                let to_index = self.traverse(target_index, index);

                writeln!(self.output, "[-{to_target}+{to_index}]").expect("TODO: panic message");

                Ok(())
            }
            Type::I32(_)
            | Type::FString(_)
            | Type::IString(_)
            | Type::Array(_)
            | Type::EmptyCell => Err(TypeMismatch(vec![EmptyType::U32], vec![target.clone()])),
        }
    }

    pub fn clear(&mut self, index: usize) {

        label!(self.output, "Clearing {}\n", index);

        match self.get(index) {
            Type::U32(_) | Type::Bool(_) | Type::Char(_) => {
                self.move_to(index);

                self.array[index] = Type::EmptyCell;
                self.output.code("[-]\n");
            }
            Type::I32(_) => {
                self.move_to(index);

                self.output.code("[-]>[-]\n");
                self.array[index] = Type::EmptyCell;
                self.array.insert(index, Type::EmptyCell);

                self.index += 1;
            }
            Type::FString(val) => {

                let len = val.len() * 2 + 4;

                self.move_to(index);

                self.output.code(">>[[-]>>]>[-]\n");
                self.array[index] = Type::EmptyCell;

                (0..len).for_each(|_| self.array.insert(index, Type::EmptyCell));

                self.index += len - 1;
            }
            Type::EmptyCell => {
                panic!()
            }
            Type::IString(_) | Type::Array(_) => {

                self.move_to(index+1);

                let len = self.array.len();
                let rest = &self.array[index+1..len];

                if rest.iter().all(|x| *x == Type::EmptyCell) {

                    self.array[index] = Type::EmptyCell;
                    self.output.code("<[-]<<<[[-]<<]\n");

                    self.index = index;

                } else {
                    panic!()
                }
            }
        };
    }

    pub fn copy_val(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Copying value at {}\n", index);

        self.move_to(index);

        match self.get(index) {
            Type::U32(_) => {
                let found = self.get_slice(index, 3);
                if let [val @ Type::U32(_), EC, EC] = found {
                    self.array[index + 1] = val.clone();
                    self.output.code("[->+>+<<]>>[-<<+>>]\n");
                    self.index += 2;
                } else {
                    return Err(TypeMismatch(
                        vec![EmptyType::U32, EEC, EEC],
                        Vec::from(found),
                    ));
                }
            }
            Type::I32(_) => {
                let found = self.get_slice(index, 4);
                if let [val @ Type::I32(_), EC, EC, EC] = found {
                    self.array[index + 1] = val.clone();
                    self.array.remove(index + 2);
                    self.index += 2;

                    self.output.code("[->>+>>+<<<<]>>>>[-<<<<+>>>>]");
                    self.output.code("<<<[->>+>+<<<]>>>[-<<<+>>>]\n");
                } else {
                    return Err(TypeMismatch(
                        vec![EmptyType::I32, EEC, EEC, EEC],
                        Vec::from(found),
                    ));
                }
            }
            Type::Bool(_) => {
                let found = self.get_slice(index, 3);
                if let [val @ Type::Bool(_), EC, EC] = found {
                    self.array[index + 1] = val.clone();
                    self.output.code("[->+>+<<]>>[-<<+>>]\n");
                    self.index += 2;
                } else {
                    return Err(TypeMismatch(
                        vec![EmptyType::Bool, EEC, EEC],
                        Vec::from(found),
                    ));
                }
            }
            Type::Char(_) => {
                let found = self.get_slice(index, 3);
                if let [val @ Type::Char(_), EC, EC] = found {
                    self.array[index + 1] = val.clone();
                    self.output.code("[->+>+<<]>>[-<<+>>]\n");
                    self.index += 2;
                } else {
                    return Err(TypeMismatch(
                        vec![EmptyType::Char, EEC, EEC],
                        Vec::from(found),
                    ));
                }
            }
            Type::FString(_) | Type::IString(_) | Type::Array(_) | Type::EmptyCell => {
                unimplemented!()
            }
        }

        Ok(())
    }

    pub fn add_i32(&mut self, index: usize) -> Result<(), BfasmError> {
        label!(self.output, "Adding I32s at {index}");

        self.move_to(index);

        let found = self.get_slice(index, 9);

        if let [Type::I32(x), Type::I32(y), EC, EC, EC, EC, EC, EC, EC] = found {
            self.array[index] = Type::I32(*x + *y);

            self.array[index + 1] = Type::EmptyCell;
            self.array.insert(index + 1, Type::EmptyCell);

            // copy the two signs
            self.output
                .code("[->>>>+>+<<<<<]>>>>>[-<<<<<+>>>>>]<<<[->>>+>+<<<<]>>>>[-<<<<+>>>>]<\n");
            self.output.code("[<[->-<]>[-<+>]]<\n");
            // XOR them
            // idk dont look at me. How did i write this beauty? PS it didnt work the first time lol
            // if the signs are different subtract the u32s
            self.output
                .code("[[<[<<[->>->>]>>>>]>[>]<[>]<[->>>>]<<<<]\n");
            // and if the remaining one and copy the sign over
            self.output
                .code("<[[-<<+>>]<<<[-]>>[-<<+>>]>]<[-]>>]\n");
            // add (with nothing if diffe rence in signs) and delete extra sign
            self.output.code("<[-<<+>>]<[-]<<\n");
        } else {
            return Err(TypeMismatch(
                vec![
                    EmptyType::I32,
                    EmptyType::I32,
                    EEC,
                    EEC,
                    EEC,
                    EEC,
                    EEC,
                    EEC,
                    EEC,
                ],
                Vec::from(found),
            ));
        }

        Ok(())
    }

    pub fn input(&mut self, index: usize, input_val: Type) -> Result<(), BfasmError> {

        label!(self.output, "Inputing at {}\n", index);
        // write!(self.output, "Input {:?} at {}\n", input_val, index).unwrap();
        self.move_to(index);

        match input_val {
            Type::U32(_) | Type::I32(_) | Type::Bool(_) => {
                todo!()
            }

            Type::Char(char) => {
                self.expected_input.push(char as char);

                let val = self.get(index);

                if *val == Type::EmptyCell {
                    *val = Type::Char(char);
                    self.output.code(",");
                } else {
                    return Err(TypeMismatch(vec![EEC], vec![val.clone()]));
                }
            }

            Type::IString(str) => {
                // self.expected_input.push_str(&String::from_utf8(str).unwrap());

                self.expected_input
                    .push_str(&String::from_utf8(str.clone()).unwrap());
                self.expected_input.push('\0');

                self.output
                    .code(">>,[[>>]>[->>+<<]>>+<<<<<[[->>+<<]<<]>>,]\n");

                self.output.code(">>[[-<<+>>]>>]>[-<<+>>]<\n");

                let end = &self.array[index..];

                if end == Type::empty_slice(end.len()) {
                    (0..end.len()).for_each(|_| {
                        self.array.pop();
                    });

                    self.array.push(Type::IString(str));
                }

                self.index += 1;
            }

            Type::FString(_) | Type::EmptyCell | Type::Array(_) => {
                unimplemented!()
            }
        }

        Ok(())
    }

    pub fn input_str(&mut self, index: usize, str: &str) -> Result<(), BfasmError> {
        self.input(index, Type::IString(Vec::from(str.as_bytes())))
    }

    pub fn index_str(&mut self, index: usize) -> Result<(), BfasmError> {
        label!(self.output, "Indexing at {index}\n");
        self.move_to(index + 1);

        let found = self.get_slice(index, 3);

        if let [Type::IString(val) | Type::FString(val), Type::U32(str_index), EC] = found {
            let str_index = *str_index as usize;

            // dbg!(index, str_index);

            let ret = match val.get(str_index) {
                None => {
                    self.array[index + 1] = Type::Char(0);
                    Err(BfasmError::OpError(OpError::InvalidStringIndex(str_index)))
                }
                Some(val) => {
                    self.array[index + 1] = Type::Char(*val);
                    Ok(())
                }
            };

            // fill ones
            self.output.code("[-<<<[<]+[>]>>]\n");
            // grab the indexed value and copy it
            self.output
                .code("<<<[<]<[->>[>]>>+>+<<<<[<]<]>>[>]>>>\n");
            // put the value back abd remove the ones
            self.output
                .code("[-<<<<[<]<+>>[>]>>>]<<<<[<]>[>->]>>\n");

            ret
        } else {
            Err(TypeMismatch(
                vec![EmptyType::IString, EmptyType::U32, EEC],
                Vec::from(found),
            ))
        }
    }

    pub fn print(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Printing at {index}\n");

        self.move_to(index);

        match self.get(index) {
            Type::U32(val) => {
                let char = *val as u8 as char;
                self.expected_output.push(char);

                self.output.code(".");
                Ok(())
            }

            Type::Char(val) => {
                let char = *val as char;
                self.expected_output.push(char);

                self.output.code(".");
                Ok(())
            }

            bf_type @ (Type::I32(_)
            | Type::Bool(_)
            | Type::FString(_)
            | Type::IString(_)
            | Type::Array(_)
            | Type::EmptyCell) => Err(TypeMismatch(vec![EmptyType::Char], vec![bf_type.clone()])),
        }
    }

    pub fn str_push_front(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Pushing front at {index}\n");

        self.move_to(index + 1);

        if let [Type::FString(array) | Type::IString(array), Type::Char(char), EC] =
            self.get_slice(index, 3)
        {
            array.insert(0, *char);

            self.array.remove(index + 1);
            self.array.remove(index + 1);

            self.output.code("[-<<+>>]<[->>+<<]>>+>\n");
        } else {
            return Err(TypeMismatch(
                vec![EmptyType::Array, EmptyType::U32, EEC],
                Vec::from(self.get_slice(index, 3)),
            ));
        }

        Ok(())
    }

    pub fn str_push(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Pushing at {index}\n");

        self.move_to(index - 1);

        let found = self.get_slice(index - 2, 3);

        if let [EC, Type::Char(char), Type::FString(array) | Type::IString(array)] = found {
            array.push(*char);

            self.array.remove(index - 2);
            self.array.remove(index - 2);

            self.output.code("[->+<]>[>>]>+>\n");
        } else {
            return Err(TypeMismatch(
                vec![EEC, EmptyType::Char, EmptyType::IString],
                Vec::from(found),
            ));
        }

        Ok(())
    }

    pub fn array_push(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Pushing at {index}\n");

        self.move_to(index + 1);

        let found = self.get_slice(index, 3);

        if let [Type::Array(array), Type::U32(val), EC] = found {
            array.push(*val);

            self.array.remove(index + 1);
            self.array.remove(index + 1);

            self.output.code("+[-<<+>>]<[->>+<<]>>+>\n");
        } else {
            return Err(TypeMismatch(
                vec![EmptyType::Array, EmptyType::U32, EEC],
                Vec::from(found),
            ));
        }

        Ok(())
    }

    pub fn array_push_front(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Pushing front at {index}\n");

        self.move_to(index - 1);

        let found = self.get_slice(index - 2, 3);

        if let [EC, Type::U32(val), Type::Array(array)] = found {
            array.insert(0, *val);

            self.array.remove(index - 2);
            self.array.remove(index - 2);

            self.output.code("+[->+<]>[>>]>+>\n");
        } else {
            return Err(TypeMismatch(
                vec![EEC, EmptyType::U32, EmptyType::Array],
                Vec::from(found),
            ));
        }

        Ok(())
    }

    pub fn array_index(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Indexing at {index}\n");

        self.move_to(index + 1);

        let found = self.get_slice(index, 3);

        if let [EC, Type::U32(val), Type::Array(array)] = found {
            *val = array[*val as usize];

            // fill the ones
            self.output.code("[->>[>]+[<]<]\n");
            // copy the value
            self.output.code(">>[>]>[-<<[<]<+<+>>>[>]>]<<[<]<<\n");
            // put the value back and remove the ones
            self.output.code("[->>>[>]>+<<[<]<<]>>>[->>]<[<<]<-\n");
        } else {
            return Err(TypeMismatch(
                vec![EEC, EmptyType::U32, EmptyType::Array],
                Vec::from(found),
            ));
        }

        Ok(())
    }

    // just like the string index
    pub fn array_index_back(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Indexing back at {index}\n");

        self.move_to(index + 1);

        let found = self.get_slice(index, 3);

        if let [Type::Array(array), Type::U32(val), EC] = found {
            *val = array[array.len() - *val as usize - 1];

            // fill ones
            self.output.code("d[-<<<[<]+[>]>>]\n");
            // grab the indexed value and copy it
            self.output
                .code("<<<[<]<[->>[>]>>+>+<<<<[<]<]>>[>]>>>\n");
            // put the value back abd remove the ones
            self.output
                .code("[-<<<<[<]<+>>[>]>>>]<<<<[<]>[>->]>>-\n");
        } else {
            return Err(TypeMismatch(
                vec![EmptyType::Array, EmptyType::U32, EEC],
                Vec::from(found),
            ));
        }

        Ok(())
    }

    // just like the string index
    pub fn array_set_back(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Setting back at {index}\n");

        self.move_to(index + 1);

        let found = self.get_slice(index, 3);

        if let [Type::Array(array), Type::U32(bf_index), Type::U32(val)] = found {
            let array_index = array.len() - *bf_index as usize - 1;
            array[array_index] = *val;

            self.array[index + 1] = EC;
            self.array[index + 2] = EC;

            self.index = index + 1;

            // fill ones and clear value
            self.output.code("d[-<<<[<]+[>]>>]<<<[<]<[-]+>>[>]>>>\n");

            // set the value
            self.output.code("[-<<<<[<]<+>>[>]>>>]\n");

            // clear the ones
            self.output.code("<<<<[<]>[>->]>>")
        } else {
            return Err(TypeMismatch(
                vec![EmptyType::Array, EmptyType::U32, EEC],
                Vec::from(found),
            ));
        }

        Ok(())
    }

    // Todo Test
    pub fn get_len(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Getting the length at {index}\n");

        self.move_to(index + 1);

        let slice = self.get_slice(index, 3);

        if let [val @ (Type::IString(_) | Type::FString(_) | Type::Array(_)), target @ Type::EmptyCell, EC] =
            slice
        {
            let len = match val {
                Type::FString(str) | Type::IString(str) => str.len(),
                Type::Array(array) => array.len(),
                _ => {
                    unreachable!()
                }
            };

            *target = Type::U32(len as u32);

            self.output.code("<[->+>+<<]>>[-<<+>>]\n");

            self.index += 1;

            Ok(())
        } else {
            Err(TypeMismatch(
                vec![EmptyType::IString, EEC, EEC],
                Vec::from(slice),
            ))
        }
    }

    pub fn add_u32(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Adding U32s at {index}\n");

        self.move_to(index);

        let slice = self.get_slice(index, 2);

        if let [Type::U32(x), Type::U32(y)] = slice {
            *x += *y;
            self.array[index + 1] = EC;
            self.output.code(">[-<+>]<\n");
            Ok(())
        } else {
            Err(TypeMismatch(
                vec![EmptyType::U32, EmptyType::U32],
                Vec::from(slice),
            ))
        }
    }
    pub fn unsafe_sub_u32(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Unsafe Subtracting U32s at {index}\n");

        self.move_to(index);

        let slice = self.get_slice(index, 2);

        if let [Type::U32(x), Type::U32(y)] = slice {
            let (x, y) = (*x, *y);

            // *x = x.checked_sub(*y).ok_or(BfasmError::Underflow)?;

            self.array[index + 1] = EC;
            self.output.code(">[-<->]<\n");

            if x < y {
                self.array[index] = Type::U32(0);
                Err(BfasmError::OpError(OpError::Underflow))
            } else {
                self.array[index] = Type::U32(x - y);
                Ok(())
            }
        } else {
            Err(TypeMismatch(
                vec![EmptyType::U32, EmptyType::U32],
                Vec::from(slice),
            ))
        }
    }

    pub fn insert_ec(&mut self, index: usize, number: usize) -> Result<(), BfasmError> {

        label!(self.output, "Inserting {number} ECs at {index}\n");

        let mut ending_index = self.array.len();
        while *self.get(ending_index - 1) == EC {
            ending_index -= 1;
        }

        self.move_to(ending_index);

        let l = "<".repeat(number);
        let r = ">".repeat(number);

        while ending_index != index {
            ending_index -= 1;
            self.output.code("<");

            match self.get(ending_index) {
                Type::U32(_) | Type::Bool(_) | Type::Char(_) => {
                    write!(self.output, "[-{r}+{l}]").unwrap();
                }
                Type::I32(_) => {
                    write!(self.output, "[-{r}+{l}]<[-{r}+{l}]").unwrap();
                }
                Type::FString(_) | Type::IString(_) | Type::Array(_) => {
                    write!(self.output, "[-{r}+{l}]<<<[-{r}+{l}<<]").unwrap();
                }
                Type::EmptyCell => {}
            }
        }

        self.output.code("\n");

        self.index = index;

        (0..number).for_each(|_| self.array.insert(index, EC));

        Ok(())
    }

    pub fn match_char(
        &mut self,
        index: usize,
        match_arms: &[(u8, Vec<BfasmOps>)],
    ) -> Result<(), BfasmError> {

        label!(self.output, "Matching chars at {index}\n");

        self.move_to(index);

        let mut init_val = 0;
        for (index, (val, _)) in match_arms.iter().enumerate() {
            if init_val >= *val {
                return Err(BfasmError::InvalidMatchArm(index));
            };

            init_val = *val
        }

        let slice = self.get_slice(index, 6);

        if let [Type::Char(val), EC, EC, EC, EC, EC] = slice {
            let val = *val;

            let mut previous_cond = 0;

            self.output.code(">>>>+<<");

            self.index += 4; // ???

            // string would be cleared if match is succesfull
            self.array[index] = EC;

            let mut errs = None;

            // validate the arms
            for (match_index, (cond, code)) in match_arms.iter().enumerate() {
                self.output
                    .code(&"+".repeat((*cond - previous_cond) as usize));
                self.output.code("[-<<[->]>]>>[<<<<[>]>>>>[\n");

                // after the func, move to the correct location to continue matching
                let bunf_index = self.index + 1;

                // dbg!(match_index, code, &self.array);

                let str = self
                    .test_arm(code, bunf_index)
                    .ok_or(BfasmError::InvalidMatchArm(match_index))?;

                if *cond == val {

                    // dbg!("yay", val);

                    let output = self.output.is_enabled();
                    self.output.enabled(false);

                    // code.iter().for_each(|oper| {
                    //     oper.exec_instruct(self).expect("Any error should have been caught when validating")
                    // });

                    errs = BfasmOps::full_exec(code, self).unwrap();

                        // .expect("Any error should have been caught when validating");

                    // for op in code {
                    //     op.exec_instruct(self).unwrap();
                    //     // dbg!(&self.array);
                    // }

                    // dbg!("match over");

                    self.index = bunf_index;

                    self.output.enabled(output);
                }
                self.output.extend(str);

                self.output.code("\n]<<<\n");

                previous_cond = *cond;
            }

            self.output.code(&"]".repeat(match_arms.len()));
            self.output.code(">[<]>[-]<<[-]<<[-]\n");

            self.index = index;

            self.array[index] = EC;

            // +++++
            //     >>>>+<<
            //     (+++) [-<<[->]>]>>[<<<<[>]>>>>[>func1>,.<]<<<
            //     (++) [-<<[->]>]>>[<<<<[>]>>>>[>func1>,.<]<<<
            //     (+++)[-<<[->]>]>>[<<<<[>]>>>>[>func1>,.<]<<<]
            // ]
            // ]>[<]<<<

            match errs {
                None => {Ok(())}
                Some(errs) => {Err(BfasmError::OpError(OpError::ErrorsInMatch(errs)))}
            }

        } else {
            Err(TypeMismatch(
                vec![EmptyType::Char, EEC, EEC, EEC, EEC, EEC],
                Vec::from(slice),
            ))
        }

    }

    fn test_arm(
        &mut self,
        code: &[BfasmOps],
        ret_index: usize,
    ) -> Option<Vec<BFOp>> {

        // dbg!(&self.array, "check start");

        let mut bfasm = Bfasm {
            array: self.array.clone(),
            output: BfasmWriter::BFOps(Vec::new(), true),
            index: self.index,
            expected_input: String::new(),
            expected_output: String::new(),
        };

        // for oper in code {
        //     oper.exec_instruct(&mut bfasm).ok()?;
        // }

        for op in code {
            match op.exec_instruct(&mut bfasm) {
                Ok(()) => {}
                Err(TypeMismatch(_, _) | BfasmError::InvalidMatchArm(_)) => {
                    dbg!(op, code, &bfasm.array, "inner err");
                    return None;
                },
                Err(BfasmError::OpError(_)) => {}
            }
        }
        // match dbg!(op) {
        //     BfasmOps::CharMatch(ind, _) | BfasmOps::BoolWhile(ind, _) | BfasmOps::BoolIf(ind, _) => {
        //         todo!("Generate the code")
        //     }
        //     op => {
        //         match op.exec_instruct(&mut bfasm) {
        //             Ok(_) => {}
        //             Err(TypeMismatch(_, _) | BfasmError::InvalidMatchArm(_)) => {
        //                 dbg!(op, code, &bfasm.array, "inner err");
        //                 return None;
        //             },
        //             Err(_) => {}
        //         }
        //     }
        // }

        bfasm.move_to(ret_index);

        while bfasm.array.len() > self.array.len() {
            self.array.push(Type::EmptyCell);
        }

        // todo
        assert_eq!(&bfasm.array.len(), &self.array.len());

        if EmptyType::from_vec(&self.array) == EmptyType::from_vec(&bfasm.array) {
            // add better formatting
            let BfasmWriter::BFOps(output, true) = bfasm.output else {unreachable!()};
            // Some(output.replace('\n', "\n  "))
            Some(output)
        } else {
            dbg!(&self.array, &bfasm.array, code, "match fail");
            None
        }
    }

    pub fn bool_if(&mut self, index: usize, code: &[BfasmOps]) -> Result<(), BfasmError> {

        label!(self.output, "If at {index}\n");

        self.move_to(index);

        let slice = self.get(index);

        if let Type::Bool(cond) = slice {
            let cond = *cond;

            // after the func, move to the correct location to continue matching

            // correct the array
            self.array[index] = EC;

            let str = self.test_arm(code, index)
                .ok_or(BfasmError::InvalidMatchArm(0))?;

            let mut errs = None;

            if cond {
                let output = self.output.is_enabled();
                self.output.enabled(false);

                // code.iter().for_each(|oper| {
                //     oper.exec_instruct(self).expect("Any error should have been caught when validating")
                // });

                errs = BfasmOps::full_exec(code, self).unwrap();
                    // .expect("Any error should have been caught when validating"); // this panic for a while inside a while

                self.index = index;

                self.output.enabled(output);
            }

            write!(self.output, "[[-]\n{:?}]\n", str).unwrap();

            match errs{
                None => {Ok(())}
                Some(errs) => {Err(BfasmError::OpError(OpError::ErrorsInMatch(errs)))}
            }

        } else {
            Err(TypeMismatch(
                vec![EmptyType::Bool, EEC],
                vec![slice.clone()],
            ))
        }
    }

    pub fn bool_while(&mut self, index: usize, code: &[BfasmOps]) -> Result<(), BfasmError> {

        label!(self.output, "While at {index}\n");

        self.move_to(index);

        let slice = self.get(self.index);

        if let Type::Bool(bool) = slice {
            let mut cond = *bool;

            let str = self.test_arm(code, self.index).ok_or(BfasmError::InvalidMatchArm(0))?;

            let output = self.output.is_enabled();
            self.output.enabled(false);

            // dbg!("while start");

            let mut errs = None;

            while cond {
                errs = BfasmOps::full_exec(code, self).unwrap();

                self.move_to(index);

                // self.index = index;

                if errs.is_some(){
                    break
                }

                if let Type::Bool(bool) = self.get(index) {
                    cond = *bool;
                } else {
                    panic!();
                    // only tests if it will change the target maybe expand to check all of the array?
                    // return Err(BfasmError::InvalidMatchArm(0, None));
                }
            }

            self.output.enabled(output);
            write!(self.output, "[\n{:?}]\n", str).unwrap();
            self.array[index] = EC;

            match errs {
                None => {Ok(())}
                Some(errs) => {Err(BfasmError::OpError(OpError::ErrorsInMatch(errs)))}
            }

        } else {
            Err(TypeMismatch(
                vec![EmptyType::Bool, EEC],
                vec![slice.clone()],
            ))
        }
    }

    pub fn greater_than(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Greater than at {index}\n");

        self.move_to(index + 4);

        let slice = self.get_slice(index, 5);

        if let [Type::U32(val1), EC, Type::U32(val2), EC, EC] = slice {
            self.array[index] = Type::Bool(val1 > val2);
            self.array[index + 2] = EC;
            self.index = index;

            self.output
                .code("+<<[-<<[->]>]>>[<<<<[>+<[-]]>>>]>-<<[-]<[-<+>]<\n")
        } else {
            return Err(TypeMismatch(
                vec![EmptyType::U32, EEC, EmptyType::U32, EEC, EEC],
                Vec::from(slice),
            ));
        }

        Ok(())
    }

    pub fn less_than(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Less than at {index}\n");

        self.move_to(index + 3);

        let slice = self.get_slice(index, 5);

        if let [Type::U32(val1), EC, Type::U32(val2), EC, EC] = slice {
            self.array[index] = Type::Bool(val1 < val2);
            self.array[index + 2] = EC;
            self.index = index;

            self.output
                .code("+<[-<<[->]>]>>[<<+>>>]<-<[-]<<[-]>[-<+>]<\n")
        } else {
            return Err(TypeMismatch(
                vec![EmptyType::U32, EEC, EmptyType::U32, EEC, EEC],
                Vec::from(slice),
            ));
        }

        Ok(())
    }

    pub fn equals(&mut self, index: usize) -> Result<(), BfasmError> {

        label!(self.output, "Equals at {index}\n");

        self.move_to(index + 4);

        let slice = self.get_slice(index, 5);

        if let [Type::U32(val1), EC, Type::U32(val2), EC, EC] = slice {
            self.array[index] = Type::Bool(val1 == val2);
            self.array[index + 2] = EC;
            self.index = index;

            //                    +<<[-<<[->]>]>>[<<<<[-]>+>>]>-<<[-]<[-<+>]<?
            self.output
                .code("+<<[-<<[->]>]>>[<<<+<[>-<[-]]>>>]>-<<[-]<[-<+>]<\n");
        } else {
            return Err(TypeMismatch(
                vec![EmptyType::U32, EEC, EmptyType::U32, EEC, EEC],
                Vec::from(slice),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bf() {

        // let mut bunf = BFASM::new();
        //
        // bunf.input_str(",[->+<]."); // input the string

        // bunf.set(?, Type::U32(0)) // set the program index to 0

        // set up array index

        // set up the array

        // loop

        //   index string

        //   match char

        //   execute instruction

        //   if the end of the string is reached break
    }

    #[test]
    fn move_test2() {

        let mut bfasm = Bfasm::default();

        bfasm.set(0, Type::IString(vec![44,43,46])).unwrap();

        bfasm.set(3, Type::U32(0)).unwrap();
        // bfasm.set(4, Type::U32(0)).unwrap();

        bfasm.set(6, Type::Array(vec![0])).unwrap();

        bfasm.set(9, Type::U32(0)).unwrap();
        bfasm.set(12, Type::Bool(false)).unwrap();

        bfasm.move_to(5);

        bfasm.copy_val(3).unwrap();

        assert!(bfasm.test_run().unwrap())
    }

    #[test]
    fn array_test2() {

        let mut bfasm = Bfasm::default();

        bfasm.set(0, Type::IString(vec![44,43,46])).unwrap();

        bfasm.set(3, Type::U32(2)).unwrap();

        bfasm.set(6, Type::Array(vec![98])).unwrap();
        bfasm.set(7, Type::U32(0)).unwrap();

        bfasm.move_to(17);

        bfasm.array_index_back(6).unwrap();

        assert!(bfasm.test_run().unwrap())
    }

    #[test]
    fn insert_test() {
        let mut bfasm = Bfasm::default();

        bfasm.set(0, Type::U32(2)).unwrap();
        bfasm.set(1, Type::from(' ')).unwrap();

        bfasm.insert_ec(0, 2).unwrap();

        bfasm.set(0, Type::I32(-3)).unwrap();

        bfasm.insert_ec(0, 5).unwrap();

        bfasm.set(0, Type::Array(Vec::new())).unwrap();

        bfasm.insert_ec(0, 13).unwrap();

        bfasm.set(0, Type::from("abcd")).unwrap();

        assert!(bfasm.test_run().unwrap());
    }

    #[test]
    fn array_set() {
        let mut bunf = Bfasm::default();

        bunf.set(0, Type::Array(vec![0, 1, 2, 3, 4])).unwrap();

        bunf.set(1, Type::U32(3)).unwrap();
        bunf.set(2, Type::U32(5)).unwrap();

        bunf.array_set_back(0).unwrap();

        assert!(bunf.test_run().unwrap())
    }

    #[test]
    fn comparison_tests() {
        for func in [Bfasm::greater_than, Bfasm::less_than, Bfasm::equals] {
            for (x, y) in [(1, 3), (3, 1), (3, 3)] {
                let mut bunf = Bfasm::default();

                bunf.set(0, Type::U32(x)).unwrap();

                bunf.set(2, Type::U32(y)).unwrap();

                func(&mut bunf, 0).unwrap();

                assert!(bunf.test_run().unwrap())
            }
        }
    }

    #[test]
    fn while_test() {
        let mut bunf = Bfasm::default();

        bunf.set(0, Type::Bool(true)).unwrap();

        bunf.set(1, Type::U32(0)).unwrap();

        bunf.bool_while(
            0,
            &vec![
                BfasmOps::Clear(1),
                BfasmOps::Set(1, Type::U32(1)),
                BfasmOps::Clear(0),
                BfasmOps::Set(0, Type::Bool(false)),
            ],
        )
        .unwrap();

        assert!(bunf.test_run().unwrap());
    }

    #[test]
    fn if_test() {
        let mut bunf = Bfasm::default();

        bunf.set(0, Type::Bool(true)).unwrap();
        bunf.set(1, Type::I32(-1)).unwrap();

        bunf.bool_if(0, &vec![BfasmOps::Clear(1), BfasmOps::Set(1, Type::I32(1))])
            .unwrap();

        assert!(bunf.test_run().unwrap())
    }

    #[test]
    fn match_test() {
        let mut bunf = Bfasm::default();

        bunf.set(0, Type::U32(0)).unwrap();

        bunf.set(1, Type::Char(2)).unwrap();

        let mut arms = vec![
            (1, vec![BfasmOps::Clear(0), BfasmOps::Set(0, Type::U32(1))]),
            (2, vec![BfasmOps::Clear(0), BfasmOps::Set(0, Type::U32(3))]),
            (3, vec![BfasmOps::Clear(0), BfasmOps::Set(0, Type::U32(9))]),
        ];

        arms.sort_by_key(|(x, _)| *x);

        bunf.match_char(1, &arms).unwrap();

        assert!(bunf.test_run().unwrap())
    }

    #[test]
    fn copy_test() {
        let mut bunf = Bfasm::default();

        bunf.set(0, Type::U32(2)).unwrap();

        bunf.copy_val(0).unwrap();

        bunf.set(2, Type::from(-3)).unwrap();

        bunf.copy_val(2).unwrap();

        bunf.set(4, Type::from(true)).unwrap();

        bunf.copy_val(4).unwrap();

        bunf.set(6, Type::from('a')).unwrap();

        bunf.copy_val(6).unwrap();

        assert!(bunf.test_run().unwrap())
    }

    #[test]
    fn array_index() {
        let mut bunf = Bfasm::default();

        bunf.set(2, Type::Array(vec![1, 2, 3])).unwrap();

        bunf.set(1, Type::U32(0)).unwrap();

        bunf.array_index(0).unwrap();

        assert!(bunf.test_run().unwrap());
    }

    #[test]
    fn array_test() {
        let mut bunf = Bfasm::default();

        bunf.set(2, Type::Array(vec![1, 2, 3])).unwrap();

        bunf.set(1, Type::U32(0)).unwrap();

        bunf.array_push_front(2).unwrap();

        bunf.set(1, Type::U32(4)).unwrap();

        bunf.array_push(0).unwrap();

        bunf.set(1, Type::U32(0)).unwrap();

        bunf.array_index_back(0).unwrap();

        assert!(bunf.test_run().unwrap())
    }

    #[test]
    fn str_index() {
        let mut bunf = Bfasm::default();

        bunf.set(2, Type::from("hello world")).unwrap(); //

        bunf.set(3, Type::U32(1)).unwrap();

        bunf.index_str(2).unwrap();

        bunf.clear(3);

        bunf.set(3, Type::from('!')).unwrap();

        bunf.str_push_front(2).unwrap();

        bunf.set(1, Type::from('!')).unwrap();

        bunf.str_push(2).unwrap();

        assert!(bunf.test_run().unwrap())
    }

    #[test]
    fn str_input() {

        let mut bunf = Bfasm::default();

        bunf.input_str(0, "hello").unwrap();

        assert!(bunf.test_run().unwrap())
    }

    #[test]
    fn i32_addition() {
        for x in -3..3 {
            for y in -3..3 {
                dbg!(x, y);

                let mut bunf = Bfasm::default();

                bunf.set(0, Type::from(x)).unwrap();

                bunf.set(1, Type::from(y)).unwrap();

                bunf.add_i32(0).unwrap();

                assert!(bunf.test_run().unwrap())
            }
        }
    }

    #[test]
    fn set_and_clear() {
        let mut bunf = Bfasm::default();

        bunf.set(0, Type::U32(5)).unwrap();

        bunf.set(1, Type::from(-2)).unwrap();

        bunf.set(2, Type::from(true)).unwrap();

        bunf.set(3, Type::from('a')).unwrap();

        bunf.set(4, Type::from("tac ")).unwrap();

        bunf.move_to(0);

        bunf.clear(0);
        bunf.clear(2);

        bunf.set(0, Type::from(true)).unwrap();
        bunf.clear(1);
        bunf.clear(5);
        bunf.clear(4);

        assert!(bunf.test_run().unwrap());
    }
}
