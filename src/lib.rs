use crate::BunFError::TypeMismatch;
use std::fmt::{Display, Formatter};

use Type::EmptyCell as EC;

use EmptyType::EmptyCell as EEC;

mod bf;

// https://minond.xyz/brainfuck/ was used for testing code when it broke

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    U32(u32),
    I32(i32),
    Bool(bool),
    Char(u8),
    String(Vec<u8>),
    EmptyCell,
}

impl Type{
    fn len(&self) -> usize {
        match self {
            Type::U32(_) => {1}
            Type::I32(_) => {2}
            Type::Bool(_) => {1}
            Type::Char(_) => {1}
            Type::String(val) => {val.len()*2 + 4}
            Type::EmptyCell => {1}
        }
    }

    fn len_slice(slice: &[Type]) -> usize{
        slice.iter().map(Type::len).sum()
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
        assert!(value.is_ascii(), "String contained non Ascii values");

        assert!(
            !value.chars().any(|x| x == '\0'),
            "String contained null bytes"
        );

        Self::String(value.into_bytes())
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        Type::from(String::from(value))
    }
}

impl Into<Vec<u32>> for &Type {
    fn into(self) -> Vec<u32> {
        match self {
            Type::U32(x) => {
                vec![*x]
            }
            Type::I32(x) => {
                vec![x.is_negative() as u32, x.abs() as u32]
            }
            Type::Bool(x) => {
                vec![*x as u32]
            }
            Type::Char(x) => {
                vec![*x as u32]
            }
            Type::String(x) => [
                vec![0_u32, 0_u32],
                x.iter()
                    .rev()
                    .map(|char| [*char as u32, 0_u32])
                    .flatten()
                    .collect(),
                vec![0_u32, x.len() as u32],
            ]
            .into_iter()
            .flatten()
            .collect(),
            Type::EmptyCell => {
                vec![0]
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
    String,
    EmptyCell,
    Any,
}

impl EmptyType {
    pub fn from_vec(array: &[Type]) -> Vec<EmptyType> {
        array.iter().map(|x| EmptyType::from(x)).collect()
    }
}

impl From<&Type> for EmptyType {
    fn from(value: &Type) -> Self {
        match value {
            Type::U32(_) => EmptyType::U32,
            Type::I32(_) => EmptyType::I32,
            Type::Bool(_) => EmptyType::Bool,
            Type::Char(_) => EmptyType::Char,
            Type::String(_) => EmptyType::String,
            Type::EmptyCell => EmptyType::EmptyCell,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BunFError {
    TypeMismatch(Vec<EmptyType>, Vec<Type>),
    InvalidIndex(usize),
    InvalidStringIndex(usize),
}

impl Display for BunFError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            TypeMismatch(expected, found) => {
                format!("Type Mismatch: Expected: {:?} Found: {:?}", expected, found)
            }
            BunFError::InvalidIndex(index) => format!("Invalid array index of {index}"),
            BunFError::InvalidStringIndex(index) => format!("Invalid string index of {index}"),
        })
    }
}

impl std::error::Error for BunFError {}

// TODO make a doc comment
// if the pointer is at a type it will be at the first cell of it
#[derive(Debug, Clone)]
pub struct BunF {
    pub array: Vec<Type>,
    pub output: String,
    pub index: usize,
    // TODO:
    // Add BF code labeling !!!
    // Inputting values
    // if statements
    // matching chars| sort by decreasing ascii value or by most used?
}

impl Into<Vec<u32>> for &BunF {
    fn into(self) -> Vec<u32> {
        self.array
            .iter()
            .map(|x| <&Type as Into<Vec<u32>>>::into(x))
            .flatten()
            .collect()
    }
}

impl BunF {
    pub fn new() -> Self {
        Self {
            array: vec![],
            output: String::new(),
            index: 0,
        }
    }

    pub fn run(&self) -> Result<(Vec<u32>, usize), bf::BFError> {
        self.run_io(&mut || unimplemented!(), &mut |_| unimplemented!())
    }

    pub fn run_io(
        &self,
        input: &mut dyn FnMut() -> Result<char, bf::BFError>,
        output: &mut dyn FnMut(char) -> Result<(), bf::BFError>,
    ) -> Result<(Vec<u32>, usize), bf::BFError> {
        let mut array = Vec::new();

        let mut index = 0;

        bf::run_bf(&mut array, &mut index, &self.output, input, output, &mut 0)?;

        Ok((array, index))
    }

    pub fn test_run(mut self) -> Result<bool, bf::BFError> {
        println!("{}", &self.output);

        // automagically moves the cursor to 0 until I can implement sizes for Types
        self.move_to(0);

        let (mut found, index) = self.run()?;

        println!("Found: {index}  {:?}", &found);

        let mut expected: Vec<u32> = (&self).into();

        println!("Expected: {:?}", &expected);

        // make all i32 -0 -> 0
        for (index, val) in self.array.iter().enumerate() {
            if let Type::I32(0) = val {
                found[Type::len_slice(&self.array[0..index])] = 0;
            }
        }

        // let s_array = if found > expected {&mut expected} else {&mut found};
        //
        // (0 .. found.len().abs_diff(expected.len())).for_each(|_ |s_array.push(0));

        while found.len() != expected.len() {

            if found.len() > expected.len() {
                expected.push(0);
            } else {
                found.push(0);
            }
        };

        Ok(found == expected && index == 0)
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
        while index > self.array.len() {
            self.array.push(Type::EmptyCell);
        }

        self.array.get_mut(index).unwrap()
    }

    fn empty_slice(length: usize) -> Vec<Type> {
        (0..length).map(|_| Type::EmptyCell).collect()
    }

    pub fn move_to(&mut self, index: usize) {

        // dbg!(&self);

        while index != self.index {

            if index > self.index {

                let str = match self.get(self.index) {
                    Type::U32(_) | Type::Bool(_) | Type::Char(_) | Type::EmptyCell => ">",
                    Type::I32(_) => ">>",
                    Type::String(_) => ">>[>>]>>",
                };

                self.output.push_str(str); // dbg!(str, index).0);

                self.index += 1;

            } else if index < self.index {

                let str = match self.get(self.index - 1) {
                    Type::U32(_) | Type::Bool(_) | Type::Char(_) | Type::EmptyCell => "<",
                    Type::I32(_) => "<<",
                    Type::String(_) => "<<<<[<<]"
                };

                self.output.push_str(str); //dbg!(str, index).0);

                self.index -= 1;
            } else {
                unreachable!()
            }
        }

        self.index = index;
    }

    pub fn set(&mut self, index: usize, item: Type) -> Result<(), BunFError> {

        self.move_to(index);

        // if self.array.len() <= index {return Err(BunFError::InvalidIndex(index));}

        match item {
            Type::U32(val) => {
                let x = self.get_slice(index, 1);
                if x == [EC] {
                    self.array[index] = Type::U32(val);
                    self.output
                        .push_str(&format!("{}>\n", "+".repeat(val as usize)));
                } else {
                    return Err(TypeMismatch(vec![EEC], Vec::from(x)));
                }
            }
            Type::I32(val) => {
                let x = self.get_slice(index, 2);
                if x == [EC, EC] {
                    self.array.remove(index);
                    self.array[index] = Type::I32(val);
                    self.output.push_str(&format!(
                        "{}{}>\n",
                        if val.is_negative() { "+>" } else { ">" },
                        "+".repeat(val.abs() as usize)
                    ));
                } else {
                    return Err(TypeMismatch(vec![EEC, EEC], Vec::from(x)));
                }
            }
            Type::Bool(val) => {
                let x = self.get_slice(index, 1);
                if x == [EC] {
                    self.array[index] = Type::Bool(val);
                    self.output.push_str(if val { "+>\n" } else { "\n" });
                } else {
                    return Err(TypeMismatch(vec![EEC], Vec::from(x)));
                }
            }
            Type::Char(val) => {
                let x = self.get_slice(index, 1);
                if x == [EC] {
                    self.array[index] = Type::Char(val);
                    self.output
                        .push_str(&format!("{}>\n", "+".repeat(val as usize)));
                } else {
                    return Err(TypeMismatch(vec![EEC], Vec::from(x)));
                }
            }
            Type::String(val) => {
                let len = val.len() * 2 + 5;
                let x = self.get_slice(index, len);
                let expected = (0..len).map(|_| EC).collect::<Vec<Type>>();

                if x == expected {
                    self.output.push_str(&format!(
                        "{}>>>{}>\n",
                        val.iter()
                            .rev()
                            .map(|char| format!(">>{}", "+".repeat(*char as usize)))
                            .collect::<String>(), // add each char
                        "+".repeat(val.len())
                    ));
                    (0..len).for_each(|_| {
                        self.array.remove(index);
                    });
                    self.array.insert(index, Type::String(val));
                } else {
                    return Err(TypeMismatch(
                        expected.iter().map(|_| EEC).collect(),
                        Vec::from(x),
                    ));
                }
            }
            Type::EmptyCell => {
                todo!() // ?
            }
        };

        self.index += 1;

        Ok(())
    }

    fn clear(&mut self, index: usize) {
        self.move_to(index);

        match self.get(index) {
            Type::U32(_) | Type::Bool(_) | Type::Char(_) => {
                self.output.push_str("[-]\n");
                self.array[index] = Type::EmptyCell;
            }
            Type::I32(_) => {
                self.output.push_str("[-]>[-]\n");
                self.array[index] = Type::EmptyCell;
                self.array.insert(index, Type::EmptyCell);

                self.index += 1;
            }
            Type::String(val) => {
                let len = val.len() * 2 + 4;
                self.output.push_str(">>[[-]>>]>[-]\n");
                self.array[index] = Type::EmptyCell;

                (0 .. len).for_each(|_| self.array.insert(index, Type::EmptyCell));

                self.index += len-1;
            }
            Type::EmptyCell => {
                //     Todo?
            }
        };
    }

    pub fn add_i32(&mut self, index: usize) -> Result<(), BunFError> {
        self.move_to(index);

        let found = self.get_slice(index, 9);

        if let [Type::I32(x), Type::I32(y), EC, EC, EC, EC, EC, EC, EC] = found {
            self.array[index] = Type::I32(*x + *y);

            self.array[index + 1] = Type::EmptyCell;
            self.array.insert(index + 1, Type::EmptyCell);

            // copy the two signs
            self.output
                .push_str("[->>>>+>+<<<<<]>>>>>[-<<<<<+>>>>>]<<<[->>>+>+<<<<]>>>>[-<<<<+>>>>]<\n");
            self.output.push_str("[<[->-<]>[-<+>]]<\n");
            // XOR them
            // idk dont look at me. How did i write this beauty? PS it didnt work the first time lol
            // if the signs are different subtract the u32s
            self.output
                .push_str("[[<[<<[->>->>]>>>>]>[>]<[>]<[->>>>]<<<<]\n");
            // and if the remaining one and copy the sign over
            self.output
                .push_str("<[[-<<+>>]<<<[-]>>[-<<+>>]>]<[-]>>]\n");
            // add (with nothing if difference in signs) and delete extra sign
            self.output.push_str("<[-<<+>>]<[-]<<\n");
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i32_addition() {
        for x in -3 .. 3{

            for y in -3 .. 3{

                dbg!(x, y);

                let mut bunf = BunF::new();

                bunf.set(0, Type::from(x)).unwrap();

                bunf.set(1, Type::from(y)).unwrap();

                bunf.add_i32(0).unwrap();

                assert!(bunf.test_run().unwrap())
            }

        }
    }

    #[test]
    fn set_and_clear() {
        let mut bunf = BunF::new();

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
