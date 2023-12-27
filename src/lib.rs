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

impl Sized for dyn Type{}

struct Block<Return: Type + ?Sized> {
    statements : Vec<Statements<dyn Type>>,
    return_type: PhantomData<Return>
}

struct Var<T: Type>{
    phantom: PhantomData<T>,
}
struct Assign<T: Type>(pub Option<Var<T>>,pub Block<T> );

enum Statements <Return: Type  + ?Sized> {
    IfElse(Block<Bool>, Block<Return>, Block<Return>),
    While(Block<Bool>, Block<()>),
    Match(Match<Return, dyn Type>),
    Assign(Assign<dyn Type>),
}

struct Match<Return: Type, Item: Type + ?Sized>(pub Block<Item>, pub Vec<MatchClause<Item, Return>>);

struct MatchClause<Item, Return: Type> {
    item: Item,
    block: Block<Return>,
}

pub fn parse() {}