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
    IString(Vec<u8>),
    Array(Vec<u32>),
    EmptyCell,
}

impl Type{

    fn empty_slice(length: usize) -> Vec<Type> {
        (0..length).map(|_| Type::EmptyCell).collect()
    }

    fn len(&self) -> usize {
        match self {
            Type::U32(_) => {1}
            Type::I32(_) => {2}
            Type::Bool(_) => {1}
            Type::Char(_) => {1}
            Type::String(val) => {val.len()*2 + 4}
            Type::EmptyCell => {1}
            Type::IString(_) | Type::Array(_) => {unimplemented!()}
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
        Type::from(value.as_bytes())
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        Type::from(value.as_bytes())
    }
}

impl From<&[u8]> for Type{
    fn from(value: &[u8]) -> Self {
        assert!(value.is_ascii(), "String contained non Ascii values");

        assert!(
            !value.iter().any(|x| x == &0),
            "String contained null bytes"
        );

        Self::String(Vec::from(value))
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
            Type::String(x) | Type::IString(x) => [
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
            Type::Array(x) => [
                vec![0_u32, 0_u32],
                x.iter()
                    .map(|val| [*val + 1, 0_u32])
                    .flatten()
                    .collect(),
                vec![0_u32, x.len() as u32],
            ].into_iter().flatten().collect(),
            Type::EmptyCell => {
                vec![0]
            }
        }
    }
}

impl From<&Type> for String {
    fn from(value: &Type) -> Self {
        match value {
            Type::U32(val) => (*val).to_string(),
            Type::I32(val) => (*val).to_string(),
            Type::Bool(val) => String::from(if *val {'t'} else {'f'}),
            Type::Char(val) => (*val).to_string(),
            Type::String(val) => String::from_utf8(val.clone()).unwrap(),
            Type::IString(val) => String::from_utf8(val.clone()).unwrap(),
            Type::EmptyCell | Type::Array(_) => {unimplemented!()}
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
    IString,
    EmptyCell,
    Any,
    Array,
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
            Type::IString(_) => EmptyType::IString,
            Type::Array(_) => EmptyType::Array,
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
    pub expected_input: String,
    pub expected_output: String,
    // TODO:
    // Add BF code labeling !!!
    // if statements
    // matching chars | sort by decreasing ascii value or by most used?
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

impl Into<Vec<u32>> for &mut BunF {
    fn into(self) -> Vec<u32> {
        <&BunF as Into<Vec<u32>>>::into(self)
    }
}

impl BunF {
    pub fn new() -> Self {
        Self {
            array: vec![],
            output: String::new(),
            index: 0,
            expected_input: String::new(),
            expected_output: String::new(),
        }
    }

    pub fn run(&self) -> Result<(Vec<u32>, usize), bf::BFError> {
        self.run_io(&mut || unimplemented!(), &mut |_| unimplemented!())
    }

    pub fn str_run(&self, input: &str) -> Result<((Vec<u32>, usize), String), bf::BFError> {

        let mut x = 0;
        let mut input_fn = move || {
            let char = input.chars().nth(x).ok_or(bf::BFError::InputFailed);
            x += 1;
            char
        };

        let mut output = String::new();

        let mut output_fn = |char| {
            output.push(char);
            Ok(())
        };

        Ok((self.run_io(&mut input_fn, &mut output_fn)?, output))

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

    pub fn io_test_run(&mut self,
                       input: &mut dyn FnMut() -> Result<char, bf::BFError>,
                       output: &mut dyn FnMut(char) -> Result<(), bf::BFError>,
    ) -> Result<bool, bf::BFError> {

        // automagically moves the cursor to 0 until I can implement sizes for Types
        self.move_to(0);

        println!("{}", &self.output);

        let (mut found, index) = self.run_io(input, output)?;

        println!("Found: {index}  {:?}", &found);

        let mut expected: Vec<u32> = self.into();

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

    pub fn test_run(mut self) -> Result<bool, bf::BFError> {
        let mut x = 0;
        let input = self.expected_input.clone();
        let mut input_fn = move || {
            let char = input.chars().nth(x).ok_or(bf::BFError::InputFailed);
            x += 1;
            char
        };

        let mut found_output = String::new();

        let mut output_fn = |char| {
            found_output.push(char);
            Ok(())
        };

        Ok(self.io_test_run(&mut input_fn, &mut output_fn)? && (found_output == self.expected_output))
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

    pub fn move_to(&mut self, expected_index: usize) {

        // dbg!(&self);

        while expected_index != self.index {

            if expected_index > self.index {

                let str = match self.get(self.index) {
                    Type::U32(_) | Type::Bool(_) | Type::Char(_) | Type::EmptyCell => ">",
                    Type::I32(_) => ">>",
                    Type::String(_) | Type::IString(_) | Type::Array(_) => ">>[>>]>>",
                };

                self.output.push_str(str); // dbg!(str, index).0);

                self.index += 1;

            } else if expected_index < self.index {

                let str = match self.get(self.index - 1) {
                    Type::U32(_) | Type::Bool(_) | Type::Char(_) | Type::EmptyCell => "<",
                    Type::I32(_) => "<<",
                    Type::String(_)  | Type::IString(_) | Type::Array(_) => "<<<<[<<]"
                };

                self.output.push_str(str); //dbg!(str, index).0);

                self.index -= 1;
            } else {
                unreachable!()
            }
        }

        self.index = expected_index;
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
            Type::String(ref str) | Type::IString(ref str) => {
                let len = str.len() * 2 + 4;
                let slice = self.get_slice(index, len);
                let expected = (0..len).map(|_| EC).collect::<Vec<Type>>();

                if slice == expected {
                    self.output.push_str(&format!(
                            "{}>>>{}>\n",
                            str.iter()
                                .rev()
                                .map(|char| format!(">>{}", "+".repeat(*char as usize)))
                                .collect::<String>(), // add each char
                            "+".repeat(str.len())
                    ));
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
                    self.output.push_str(&format!(
                        "{}>>>{}>\n",
                        array.iter()
                            .map(|x| format!(">>{}", "+".repeat(*x as usize + 1)))
                            .collect::<String>(), // add each char
                        "+".repeat(array.len())
                    ));
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
                todo!() // ?
            }
        };

        self.index += 1;

        Ok(())
    }

    fn clear(&mut self, index: usize) {
        self.move_to(index);

        match self.get(index) {
            val @ (Type::U32(_) | Type::Bool(_) | Type::Char(_)) => {
                *val = Type::EmptyCell;
                self.output.push_str("[-]\n");
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
            Type::EmptyCell | Type::IString(_) | Type::Array(_) => {
                unimplemented!() //     Todo?
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

    pub fn input(&mut self, index: usize, input_val: Type) -> Result<(), BunFError> {

        self.move_to(index);

        match input_val {

            Type::U32(_) | Type::I32(_) | Type::Bool(_) => {todo!()}

            char @ Type::Char(_) => {

                self.expected_input.push_str(&String::from(&char));
                self.expected_input.push(0 as char);

                let val = self.get(index);

                if *val == Type::EmptyCell{
                    *val = char;
                    self.output.push_str(",");
                } else {
                    return Err(TypeMismatch(vec![EEC], vec![val.clone()]))
                }
            }

            Type::IString(str) => {

                // self.expected_input.push_str(&String::from_utf8(str).unwrap());
                self.expected_input.push_str(&String::from_utf8(str.clone()).unwrap());
                self.expected_input.push(0 as char);

                self.output.push_str(">>,[[>>]>[->>+<<]>>+<<<<<[[->>+<<]<<]>>,]\n");

                self.output.push_str(">>[[-<<+>>]>>]>[-<<+>>]<\n");

                let end = &self.array[index ..];

                if end == Type::empty_slice(end.len()){

                    (0..end.len()).for_each(|_| {self.array.pop();});

                    self.array.push(Type::IString(str));

                }

                self.index += 1;

            }

            Type::String(_) | Type::EmptyCell | Type::Array(_) => {unimplemented!()}
        }

        Ok(())
    }

    pub fn input_str(&mut self, index: usize, str: &str) -> Result<(), BunFError> {

        self.input(index, Type::IString(Vec::from(str.as_bytes())))

    }

    pub fn index_str(&mut self, index: usize) -> Result<(), BunFError> {

        self.move_to(index+1);

        let found = self.get_slice(index, 3);

        if let [Type::IString(val) | Type::String(val), Type::U32(str_index), EC] = found{

            self.array[index+1] = Type::Char(val[*str_index as usize]);

            // fill ones
            self.output.push_str("[-<<<[<]+[>]>>]\n");
            // grab the indexed value and copy it
            self.output.push_str("<<<[<]<[->>[>]>>+>+<<<<[<]<]>>[>]>>>\n");
            // put the value back abd remove the ones
            self.output.push_str("[-<<<<[<]<+>>[>]>>>]<<<<[<]>[>->]>>\n");
        } else {return Err(TypeMismatch(vec![EmptyType::IString, EmptyType::U32, EEC], Vec::from(found)))}

        Ok(())

    }

    pub fn str_push_front(&mut self, index: usize) -> Result<(), BunFError> {
        self.move_to(index + 1);

        if let [Type::String(array) | Type::IString(array), Type::Char(char), EC] = self.get_slice(index, 3) {
            array.insert(0, *char);

            self.array.remove(index + 1);
            self.array.remove(index + 1);

            self.output.push_str("[-<<+>>]<[->>+<<]>>+>\n");
        } else {
            return Err(TypeMismatch(vec![EmptyType::Array, EmptyType::U32, EEC], Vec::from(self.get_slice(index, 3))))
        }

        Ok(())
    }

    pub fn str_push(&mut self, index: usize) -> Result<(), BunFError> {

        self.move_to(index-1);

        let found = self.get_slice(index-2, 3);

        if let [EC, Type::Char(char), Type::String(array) | Type::IString(array)] = found {

            array.push(*char);

            self.array.remove(index-2);
            self.array.remove(index-2);

            self.output.push_str("[->+<]>[>>]>+>\n");
        } else {
            return Err(TypeMismatch(vec![EEC, EmptyType::Char, EmptyType::IString], Vec::from(found)))
        }

        Ok(())
    }

    pub fn array_push(&mut self, index: usize) -> Result<(), BunFError> {

        self.move_to(index + 1);

        let found = self.get_slice(index, 3);

        if let [Type::Array(array), Type::U32(val), EC] = found {

            array.push(*val);

            self.array.remove(index + 1);
            self.array.remove(index + 1);

            self.output.push_str("+[-<<+>>]<[->>+<<]>>+>\n");

        } else {
            return Err(TypeMismatch(vec![EmptyType::Array, EmptyType::U32, EEC], Vec::from(found)))
        }

        Ok(())
    }

    pub fn array_push_front(&mut self, index: usize) -> Result<(), BunFError> {

        self.move_to(index-1);

        let found = self.get_slice(index-2, 3);

        if let [EC, Type::U32(val), Type::Array(array)] = found {

            array.insert(0, *val);

            self.array.remove(index - 2);
            self.array.remove(index - 2);

            self.output.push_str("+[->+<]>[>>]>+>\n");
        } else {
            return Err(TypeMismatch(vec![EEC, EmptyType::U32, EmptyType::Array], Vec::from(found)))
        }

        Ok(())
    }

    pub fn array_index(&mut self, index: usize) -> Result<(), BunFError> {

        self.move_to(index + 1);

        let found = self.get_slice(index, 3);

        if let [EC, Type::U32(val), Type::Array(array), ] = found{

            *val = array[*val as usize];

            // fill the ones
            self.output.push_str("[->>[>]+[<]<]");
            // copy the value
            self.output.push_str(">>[>]>[-<<[<]<+<+>>>[>]>]<<[<]<<");
            // put the value back and remove the ones
            self.output.push_str("[->>>[>]>+<<[<]<<]>>>[->>]<[<<]<-");

        } else {
            return Err(TypeMismatch(vec![EEC, EmptyType::U32, EmptyType::Array], Vec::from(found)));
        }

        Ok(())
    }

    // same as array[-index] be careful of off by one index errors!
    pub fn array_index_back(&mut self, index: usize) -> Result<(), BunFError> {

        self.move_to(index + 1);

        let found = self.get_slice(index, 3);

        if let [Type::Array(array), Type::U32(val), EC] = found{

            *val = array[array.len() - *val as usize];

            // fill ones
            self.output.push_str("-[-<<<[<]+[>]>>]\n");
            // grab the indexed value and copy it
            self.output.push_str("<<<[<]<[->>[>]>>+>+<<<<[<]<]>>[>]>>>\n");
            // put the value back abd remove the ones
            self.output.push_str("[-<<<<[<]<+>>[>]>>>]<<<<[<]>[>->]>>-\n");

        } else {
            return Err(TypeMismatch(vec![EmptyType::Array, EmptyType::U32, EEC], Vec::from(found)));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bf() {

        // let mut bunf = BunF::new();
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
    fn array_index() {

        let mut bunf = BunF::new();

        bunf.set(2, Type::Array(vec![1,2,3])).unwrap();

        bunf.set(1, Type::U32(0)).unwrap();

        bunf.array_index(0).unwrap();

        assert!(bunf.test_run().unwrap());
    }

    #[test]
    fn array_test() {

        let mut bunf = BunF::new();

        bunf.set(2, Type::Array(vec![1,2,3])).unwrap();

        bunf.set(1, Type::U32(0)).unwrap();

        bunf.array_push_front(2).unwrap();

        bunf.set(1, Type::U32(4)).unwrap();

        bunf.array_push(0).unwrap();

        bunf.set(1, Type::U32(1)).unwrap();

        bunf.array_index_back(0).unwrap();

        assert!(bunf.test_run().unwrap())
    }

    #[test]
    fn str_index() {

        let mut bunf = BunF::new();

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

        let mut bunf = BunF::new();

        bunf.input_str(0, "hello").unwrap();

        assert!(bunf.test_run().unwrap())

    }

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
