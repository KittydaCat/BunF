use std::marker::PhantomData;

mod bfasm;

trait Type{}

struct U32 {value: u32}
impl Type for U32{}
struct I32 {value: i32}
impl Type for I32{}
struct Bool {value: bool}
impl Type for Bool{}
struct Char {value: u8}
impl Type for Char{}
struct FString {value: Vec<u8>}
impl Type for FString{}
struct IString {value: Vec<u8>}
impl Type for IString{}
struct Array {value: Vec<u32>}
impl Type for Array{}

impl Type for (){}


struct Var<T: Type + ?Sized>{
    phantom: PhantomData<T>,
    num: usize,
}
struct Assign<T: Type + ?Sized>(pub Option<Var<T>>, pub Block<T> );

enum Return<Return>{
    Block(Block<Return>),
    Var(Var<Return>),
    Function(Function<Return>),
}

struct Function<Return>{
    signature: Vec<dyn Type>,

}

enum Statement<Return: Type + ?Sized> {
    IfElse(Block<Bool>, Block<()>, Block<()>),
    While(Block<Bool>, Block<()>),
    Match(Match<dyn Type>),
    Assign(Assign<dyn Type>),
    Return(Return<Return>),
}

struct Match<Item: Type + ?Sized>(pub Block<Item>, pub Vec<MatchClause<Item>>);

struct MatchClause<Item: ?Sized> {
    item: Box<Item>,
    block: Block<()>,
}

struct Block<Return: Type + ?Sized> {
    statements : Vec<Statement<dyn Type>>,
    return_type: PhantomData<Return>
}

impl<Return: Type> Block<Return> {

    fn run(&self, vars: &mut Vec<Box<dyn Type>>) -> Return {

        self.statements.iter()

    }
}

impl<Return: Type> Statement<Return> {

    fn run(&self, vars: &mut Vec<Box<dyn Type>>) -> Return {
        match self{
            Statement::IfElse(cond, _, _) => {}
            Statement::While(cond, _) => {}
            Statement::Match(Match(item, condition)) => {}
            Statement::Assign(_) => {}
            Statement::Return(_) => {}
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_parse() {
    }
}