use crate::BunFError::TypeMismatch;
use crate::EmptyType::EmptyCell;

mod bf;

// https://minond.xyz/brainfuck/ was used for testing code when it broke

#[derive(Debug, Clone, PartialEq)]
pub enum Type{
    U32(u32),
    I32(i32),
    Bool(bool),
    Char(u8),
    String(Vec<u8>),
    EmptyCell,
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

        assert!(!value.chars().any(|x| x == '\0'), "String contained null bytes");

        Self::String(value.into_bytes())
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        Type::from(String::from(value))
    }
}

impl Into<Vec<u32>> for Type{
    fn into(self) -> Vec<u32> {
        match self{
            Type::U32(x) => {vec!(x)}
            Type::I32(x) => {vec!(x.is_negative() as u32, x.abs() as u32)}
            Type::Bool(x) => {vec!(x as u32)}
            Type::Char(x) => {vec!(x as u32)}
            Type::String(x) => {
                [vec!(0_u32, 0_u32),
                    x.iter().rev().map(|char| [*char as u32, 0_u32]).flatten().collect(),
                    vec!(0_u32, x.len() as u32)].into_iter().flatten().collect()}
            Type::EmptyCell => {vec!(0)}
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmptyType{
    U32,
    I32,
    Bool,
    Char,
    String,
    EmptyCell,
    Any,
}

impl From<Type> for EmptyType{
    fn from(value: Type) -> Self {
        match value{
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
pub enum BunFError{
    TypeMismatch(Vec<EmptyType>, Vec<Type>),
    InvalidIndex,
    InvalidStringIndex,
}

pub struct BunF{
    pub array: Vec<Type>,
    pub output: String,
    pub index: usize,
    // TODO:
    // Add BF code labeling !!!
    // Inputting values
    // if statements
    // matching chars| sort by decreasing ascii value or by most used?
}

impl Into<Vec<u32>> for BunF{
    fn into(self) -> Vec<u32>{
        self.array.into_iter().map(|x| <Type as Into<Vec<u32>>>::into(x)).flatten().collect()
    }
}

impl BunF{

    pub fn new() -> Self{
        Self{array: vec![], output: String::new(), index: 0}
    }

    pub fn run(&self) -> Result<Vec<u32>, bf::BFError>{
        self.run_io(&mut || {unimplemented!()}, &mut |_| {unimplemented!()})
    }

    pub fn run_io(&self, input: &mut dyn FnMut() -> Result<char, bf::BFError>,
                  output: &mut dyn FnMut(char) -> Result<(), bf::BFError>) -> Result<Vec<u32>, bf::BFError>{

        let mut array = Vec::new();

        bf::run_bf(&mut array, &mut 0, &self.output, input, output, &mut 0)?;

        // the first value is assumed to be filled and skipped

        array.remove(0);

        Ok(array)
    }

    pub fn test_run(self) -> Result<bool, bf::BFError>{

        println!("{}", &self.output);

        let mut array = self.run()?;

        println!("Array:    {:?}", &array);

        let expected: Vec<u32> = self.into();

        println!("Expected: {:?}", &expected);

        array.truncate(expected.len());

        Ok(array == expected)
    }

    fn get_slice(&mut self, index: usize, length: usize) -> &mut [Type] {
        // loop{
        //     if let Some(x) = self.array.get(index..index+length){
        //         return  x;
        //     };
        //
        //     self.array.push(Type::EmptyCell);
        // };

        while index + length < self.array.len(){self.array.push(Type::EmptyCell);};

        self.array.get_mut(index .. index+length).unwrap()
    }

    fn get(&mut self, index: usize) -> &mut Type {

        while index < self.array.len(){self.array.push(Type::EmptyCell);};

        self.array.get_mut(index).unwrap()
    }

    fn empty_slice(length: usize) -> Vec<Type> {
        (0..length).map(|_| Type::EmptyCell).collect()
    }

    pub fn move_to(&mut self, index: usize) -> Result<(), BunFError>{

        let length = index as i32 - self.index as i32;

        let (left, right) = BunF::pointer_move(
            self.array.get(std::cmp::min(self.index, index)..length as usize)
                .ok_or(BunFError::InvalidIndex)?);

        self.output.push_str(&if length.is_negative() { left } else { right });

        self.index = index;

        Ok(())
    }

    pub fn set(&mut self, item: Type, index: usize) -> Result<(), BunFError> {

        self.move_to(index)?;

        use Type::EmptyCell as EC;

        if self.array.len() <= index {return Err(BunFError::InvalidIndex);}

        match item{
            Type::U32(val) => {
                let x = self.get_slice(index, 1);
                if x == [EC] {
                    self.array[index] = Type::U32(val);
                    self.output.push_str(&format!(">{}\n", "+".repeat(val as usize)));
                } else {
                    return Err(TypeMismatch(vec!(EmptyCell), Vec::from(x)));
                }
            },
            Type::I32(val) => {
                let x = self.get_slice(index, 1);
                if x == [EC, EC] {
                    self.array.remove(index);
                    self.array[index] = Type::I32(val);
                    self.output.push_str(
                        &format!("{}{}\n", if val.is_negative() {"+>"} else {">"},
                        "+".repeat(val.abs() as usize)));
                } else{
                    return Err(TypeMismatch(vec!(EmptyCell, EmptyCell), Vec::from(x)));
                }
            },
            Type::Bool(val) => {
                let x = self.get_slice(index, 1);
                if x == [EC]{
                    self.array[index] = Type::Bool(val);
                    self.output.push_str(if val {"+\n"} else {"\n"});
                } else {
                    return Err(TypeMismatch(vec!(EmptyCell), Vec::from(x)));
                }
            },
            Type::Char(val) => {
                let x = self.get_slice(index, 1);
                if x == [EC]{
                    self.array[index] = Type::Char(val);
                    self.output.push_str(&format!("{}\n", "+".repeat(val as usize)));
                } else {
                    return Err(TypeMismatch(vec!(EmptyCell), Vec::from(x)));
                }
            },
            Type::String(val) => {
                let len = val.len()*2 + 5;
                let x = self.get_slice(index, len);
                let expected = (0..len).map(|_| EC).collect::<Vec<Type>>();

                if x == expected{

                    self.output.push_str(
                        &format!(">{}>>>{}\n",
                            val.iter().rev()
                                .map(|char|format!(">>{}", "+".repeat(*char as usize)))
                                .collect::<String>(), // add each char
                            "+".repeat(val.len())
                        )
                    );
                    (0..len).for_each(|_| {self.array.remove(index);});
                    self.array.insert(index, Type::String(val));

                } else {
                    return Err(TypeMismatch(expected.iter().map(|_| EmptyCell).collect(), Vec::from(x)))
                }
            },
            Type::EmptyCell => {
            //     Todo?
            },
        };

        Ok(())

    }

    fn clear(&mut self, index: usize) -> Result<(), BunFError>{

        self.move_to(index)?;

        match self.get(index) {
            Type::U32(_) | Type::Bool(_) | Type::Char(_) => {
                self.output.push_str("[-]\n");
                self.get_slice(index, 1).swap_with_slice(&mut BunF::empty_slice(1));
            }
            Type::I32(_) => {
                self.output.push_str("[-]<[-]\n");
                self.get_slice(index, 2).swap_with_slice(&mut BunF::empty_slice(2));
            }
            Type::String(val) => {
                let len = val.len()*2 + 5;
                self.output.push_str("[-]<<<[[-]<<]\n");
                self.get_slice(index, len).swap_with_slice(&mut BunF::empty_slice(len))
            }
            Type::EmptyCell => {
            //     Todo?
            }
        };

        Ok(())
    }

    // generates the bf string to move over the given range
    // eg. [String, u32, i32] ->
    // true => forwards, false = backwards
    pub fn pointer_move(slice: &[Type]) -> (String, String){
        (
            slice.iter().map(|x|
            match x {
                Type::U32(_) | Type::Bool(_) | Type::Char(_) | Type::EmptyCell => ">",
                Type::I32(_) => ">>",
                Type::String(_) => ">>>[>>]>",
            }).collect::<String>(),

            slice.iter().rev().map(|x|
            match x {
                Type::U32(_) | Type::Bool(_) | Type::Char(_) | Type::EmptyCell => "<",
                Type::I32(_) => "<<",
                Type::String(_) => "<[<<]<<<",
            }).collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
