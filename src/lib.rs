mod bfasm;

struct Block<T: Option<bfasm::Type>> {
    eval: T,
    statments
}

enum Statements{
    IfElse(Vec<Statements>, Vec<Statements>, Vec<Statements>),
    While(Vec<Statements>, Vec<Statements>),
    Match(Vec<Statements>, Vec)
}

pub fn