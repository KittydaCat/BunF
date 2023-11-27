use std::fmt::format;

mod bf;

// https://minond.xyz/brainfuck/ was used for testing code when it broke

macro_rules! bf_func_pop {
    ($self: ident, $name:ident, $($type:ident($var:ident)),+, $code:block) => {
        pub fn $name(&mut $self) -> Result<(), BunFError>{
            match ($(discard!($var, $self.array.pop()),)*){
                ($(Some(Type::$type($var)),)*) => {$code Ok(())},
                ($($var,)*) => type_error!($($type,)* $($var,)*)
            }
        }
    };
}

macro_rules! discard {
  ($token:tt, $v:expr) => { $v }
}
macro_rules! type_error {
    ($ex1:ident, $ex2:ident, $f1:ident, $f2:ident $(,)?) => {
        Err(BunFError::TypesMismatch([EmptyType::$ex1, EmptyType::$ex2], [$f1, $f2]))
    };
    // ($ex1:ident, $ex2:ident, $f1:ident, $f2:ident,) => {
    //     Err(BunFError::TypesMismatch([EmptyType::$ex1, EmptyType::$ex2], [$f1, $f2]))
    // };
    ($ex1:ident, $f1:ident $(,)?) => {
        Err(BunFError::TypeMismatch([EmptyType::$ex1], [$f1]))
    };
    // ($ex1:ident, $f1:ident,) => {
    //     Err(BunFError::TypeMismatch([EmptyType::$ex1], [$f1]))
    // };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type{
    U32(u32),
    I32(i32),
    Bool(bool),
    Char(u8),
    String(Vec<u8>),
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
    Any,
}

impl From<Type> for EmptyType{
    fn from(value: Type) -> Self {
        match value{
            Type::U32(_) => {EmptyType::U32 }
            Type::I32(_) => {EmptyType::I32 }
            Type::Bool(_) => {EmptyType::Bool }
            Type::Char(_) => {EmptyType::Char }
            Type::String(_) => {EmptyType::String }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BunFError{
    TypeMismatch([EmptyType; 1], [Option<Type>; 1]),
    TypesMismatch([EmptyType; 2], [Option<Type>; 2]),// expected ... found ...
}

pub struct BunF{
    pub array: Vec<Type>,
    pub output: String,
    // TODO: Add BF code labeling !!!
    // TODO: Inputting values
}

impl Into<Vec<u32>> for BunF{
    fn into(self) -> Vec<u32>{
        self.array.into_iter().map(|x| <Type as Into<Vec<u32>>>::into(x)).flatten().collect()
    }
}

impl BunF{

    pub fn new() -> Self{
        Self{array: vec![], output: String::new()}
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

        dbg!(&self.output);

        let mut array = self.run()?;

        println!("Array:    {:?}", &array);

        let expected: Vec<u32> = self.into();

        println!("Expected: {:?}", &expected);

        array.truncate(expected.len());

        Ok(array == expected)
    }

    pub fn push(&mut self, item: Type){

        self.output.push_str(&*match &item { 

            Type::U32(n) => {format!(">{}", "+".repeat(*n as usize))}

            Type::I32(x) => {
                let mut output = String::from(">");

                if x.is_negative(){output.push_str("+")}
                else{output.push_str("")};

                output.push_str(&*format!(">{}", "+".repeat(x.abs() as usize)));

                output

            }
            Type::Bool(x) => {format!(">{}", {if *x{"+"} else{""}})}
            Type::Char(char) => {format!(">{}", "+".repeat(*char as usize))}

            // ahhhhh pls work
            Type::String(str) => {
                format!(">{}>>>{}",
                        str.iter().rev()
                            .map(|char|format!(">>{}", "+".repeat(*char as usize))).collect::<String>(), // add each char
                        "+".repeat(str.len())
                    /*Skips two cells then adds a char every other cell
                    ending the string with two empty cells then the length?
                    The String is backwards in memory so it can be indexed easier*/
                )}
        });

        self.array.push(item);
    }

    pub fn pop(&mut self) -> Result<(), BunFError>{

        self.output.push_str(match self.array.pop().ok_or(BunFError::TypeMismatch([EmptyType::Any],[None]))?{

            Type::U32(_) | Type::Bool(_) | Type::Char(_) => {"[-]<"}
            Type::I32(_) => {"[-]<[-]<"}

            Type::String(_) => {"[-]<<<[[-]<<]<"}
            /*Removes the length then jumps the gap and deletes the string*/
        });

        Ok(())

        // self.array.pop().ok_or(TypeMismatch(vec!(None),vec!(None)))
    }

    bf_func_pop!{self, add_u32, U32(x), U32(y), {
        self.output.push_str("[-<+>]<");
        self.array.push(Type::U32(x+y));
    }}


    bf_func_pop!{self, add_i32, I32(x), I32(y), {

        todo!()// TODO
        self.output.push_str("");
        self.array.push(Type::I32(x+y));

    }}

    bf_func_pop!{self, and, Bool(x), Bool(y), {
        self.output.push_str("[-<+>]<[->]<");
        /* Add the two values giving us |(0, 1, 2)|0| then if the result is positive subtract one */

        self.array.push(Type::Bool(x && y));
    }}
    bf_func_pop!{self, or, Bool(x), Bool(y), {
        self.output.push_str("[-<+>]<[[-]>+<]>[-<+>]<");
        /*Combines the two values then if the number is one or greater set it to one*/
        self.array.push(Type::Bool(x|y));
    }}

    bf_func_pop!{self, not, Bool(x), {
        self.output.push_str(">+<[->-<]>[-<+>]<");
        /*Add push one to the stack then if the bool is one subtract from both then combine the values*/
        self.array.push(Type::Bool(!x));
    }}

    bf_func_pop!{self, xor, Bool(x), Bool(y), {
        self.output.push_str("[<[->-<]>[-<+>]]<");
        /*
        if y{
            if x {
                case 1, 1
                subtract one from both x and y
                giving 0, 0
            }
            case 0, 1 or 0, 0
            combine the bits
            giving 1,0 or 0, 0
        }

        */
        self.array.push(Type::Bool(x^y));
    }}

    pub fn xnor(&mut self) -> Result<(), BunFError> {
        self.xor()?;
        self.not()
    }

    // generates the bf string to move over the given range
    // eg. [String, u32, i32] ->
    // true => forwards, false = backwards
    pub fn pointer_move(slice: &[Type]) -> (String, String){
        (
            slice.iter().map(|x|
            match x {
                Type::U32(_) | Type::Bool(_) | Type::Char(_) => ">",
                Type::I32(_) => ">>",
                Type::String(_) => ">>>[>>]>",
            }).collect::<String>(),

            slice.iter().rev().map(|x|
            match x {
                Type::U32(_) | Type::Bool(_) | Type::Char(_) => "<",
                Type::I32(_) => "<<",
                Type::String(_) => "<[<<]<<<",
            }).collect::<String>()
        )
    }

    pub fn copy(&mut self, rev_index: usize) -> Result<(), BunFError> {

        let (right, left) = BunF::pointer_move(&self.array[self.array.len() - rev_index..]);

        self.output.push_str(
            &match self.array.get(self.array.len() - rev_index - 1).ok_or(BunFError::TypeMismatch([EmptyType::Any], [None]))? {
                Type::U32(_) | Type::Char(_) | Type::Bool(_) => {
                    format!("{left}[-{right}>+>+<<{left}]{right}>>[-<<{left}+{right}>>]<")
                }
                Type::I32(_) => {
                    format!("{left}[-{right}>+>+<<{left}]{right}>[-<{left}+{right}>]\
                    <{left}<[->{right}>+>>+<<<{left}<]>{right}>>>[-<<<{left}<+>{right}>>>]<")
                }
                Type::String(_) => {todo!() /*LOL if this is happening*/}
            }
        );

        self.array.push(self.array.get(self.array.len() - rev_index - 1)
            .ok_or(BunFError::TypeMismatch([EmptyType::Any], [None]))?.clone());

        Ok(())

    }

    pub fn input(&mut self, value: Type) -> (){
        self.output.push_str(match value{
            Type::U32(_) => {""}
            Type::I32(_) => {""}
            Type::Bool(_) => {""}
            Type::Char(_) => {">,"}
            Type::String(_) => {""}
        });

        self.array.push(value);

        todo!()
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules!  bunf_test{
        (($(BunF::$fn_name:ident),+), $var1:expr, $var2:expr) => {
            for (func, func_name) in [$(BunF::$fn_name),*].iter().zip([$(stringify!($fn_name)),*].iter()){
                for x in $var1{ for y in $var2{
                    let mut bunf = BunF::new();
                    bunf.push(Type::from(*x));
                    bunf.push(Type::from(*y));
                    func(&mut bunf).unwrap();
                    println!("{:?}{:?}{:?}{:?}", func_name, x, y, bunf.array);
                    assert!(bunf.test_run().unwrap());
                }}
            }
        };
            (($(BunF::$fn_name:ident),+), $var:expr) => {
            for (func, func_name) in [$(BunF::$fn_name),*].iter().zip([$(stringify!($fn_name)),*].iter()){
                for x in $var1{
                    let mut bunf = BunF::new();
                    bunf.push(Type::from(*x));
                    func(&mut bunf).unwrap();
                    println!("{:?}{:?}{:?}", func_name, x, bunf.array);
                    assert!(bunf.test_run().unwrap());
                }
            }
        };
    }

    #[test]
    fn copy_i32(){

        let mut bunf = BunF::new();

        bunf.push(Type::from(1));
        bunf.push(Type::from(true));
        bunf.push(Type::from("Hello World"));

        bunf.copy(2).unwrap();
        bunf.copy(2).unwrap();

        println!("{:?}", bunf.array);

        assert!(bunf.test_run().unwrap())
    }

    #[test]
    fn traverse(){
        let mut bunf = BunF::new();

        bunf.push(Type::from(1));
        bunf.push(Type::from(true));
        bunf.push(Type::from("Hello World"));

        bunf.output.push_str(&BunF::pointer_move(&*bunf.array).1);

        bunf.output.push_str(">+");

        bunf.array[0] = Type::I32(-1);

        println!("{}\n{:?}", bunf.output, bunf.array);

        assert!(bunf.test_run().unwrap())
    }

    #[test]
    fn string(){
        let mut x = BunF::new();

        x.push(Type::from("abcd".to_owned()));

        x.push(Type::from("tacocat".to_owned()));

        x.pop().unwrap();
        x.pop().unwrap();

        // println!("{:?} {:?} {:?}", x.array, "abcd".as_bytes(), "tacocat".as_bytes());

        assert!(x.test_run().unwrap());
    }

    #[test]
    fn two_bit(){
        bunf_test!((BunF::and, BunF::or, BunF::xor, BunF::xnor), [true, false].iter(), [true, false].iter());
    }
    #[test]
    fn two_bit_op(){

        for (func_num, func) in [BunF::and, BunF::or, BunF::xor, BunF::xnor].iter().enumerate(){
            for bits in [(true,true),(true,false),(false,true),(false,false)]{
                let mut x = BunF::new();
                x.push(Type::Bool(bits.0));
                x.push(Type::Bool(bits.1));
                func(&mut x).unwrap();
                println!("{:?} {:?} {:?}", func_num, bits, x.array);
                assert!(x.test_run().unwrap())
            }
        }
    }

    #[test]
    fn u32_adding() {
        let mut x = BunF::new();
        x.push(Type::U32(1));
        x.push(Type::U32(2));
        x.add_u32().unwrap();
        assert!(x.test_run().unwrap())
    }
    #[test]
    fn or() {
        let mut x = BunF::new();
        x.push(Type::from(true));
        x.push(Type::from(false));
        x.or().unwrap();
        println!("{:?}", x.array);
        assert!(x.test_run().unwrap())
    }

    #[test]
    fn not() {

        let mut x = BunF::new();
        x.push(Type::from(true));
        x.not().unwrap();
        println!("{:?}", x.array);
        assert!(x.test_run().unwrap())
    }
}
