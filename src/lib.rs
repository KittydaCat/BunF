mod bfasm;
mod program;

use crate::bfasm::{EmptyType, Type};

#[derive(Debug)]
struct Variable (
    String,
    // value: Type,
);


// the last type is the return value
#[derive(Debug)]
struct Function (
    Vec<EmptyType>,
    Vec<Statement>
);

#[derive(Debug)]
enum Value {
    Var(Variable),
    Func(Function),
}

#[derive(Debug)]
enum Statement{
    Match(Value, Vec<(Type, Vec<Statement>)>),
    While(Value, Vec<Statement>),
    Function(Function, Vec<Value>),
    Assignment(Variable, Value)
    // Return(Value),
}

// enum Token{
//     Let,
//     Equal,
//     DoubleEqual,
//     PlusEquals,
//     MinusEquals,
//     SemiColon,
//     OpenBrace,
//     CloseBrace,
//     Name(String),
//     Value(Type),
//     While,
//     If,
//     Else,
//     Match,
//     Arrow,
//     GreaterThan,
//     LessThan,
//     Comma,
//     FunctionCall(String)
// }
use std::iter::Enumerate;
use std::str::Chars;

// returns a alphanumeric string and the non alphanumeric or None if the iter was ended before an
// non alphanumeric char was found
fn next_word(iter: &mut Enumerate<Chars>) -> Option<(String, (usize, char))> {

    let mut str = String::new();

    while let Some((index, char)) = iter.next() {

        if char.is_alphanumeric() {
            str.push(char);
        } else {
            return Some((str, (index, char)));
        }
    }

    None

}

// found _ or nothing, expected _
// struct TokenizeError(usize, Option<char>, String);

// either returns the tokens or the point of failure
fn tokenize(code: &str) -> Result<Vec<Statement>, Option<usize>> {

    let mut char_iter = code.chars().enumerate();

    let mut token = String::new();

    let mut building_token = String::new;

    let mut tokens = Vec::new();

    loop{

        let (str, (index, char)) = next_word(&mut char_iter).ok_or(None)?;

        match str.as_str() {

            "let" => {
                if char != ' ' {
                    return Err(Some(index))
                }

                let (var_name, (_, mut char)) = next_word(&mut char_iter).ok_or(None)?;

                while char != '='{
                    (_, char) = char_iter.next().ok_or(None)?;
                };

                let value_char  = loop {
                    let (_, char) = char_iter.next().ok_or(None)?;

                    if char == ' ' {continue} else {break char}
                };

                let (mut value_name, (_, char)) = next_word(&mut char_iter).ok_or(None)?;

                if char == ';'{
                    value_name.insert(0, value_char);
                    tokens.push(Statement::Assignment(Variable(var_name), Value::Var(Variable(value_name))));
                }
            },
            _ => {}
        }

    }

    Ok(tokens)

}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    # [test]
    fn let_to_ast(){
        let code = "let a = 100;\
        let b = 90;\
        let c = a;";

        let x = tokenize(code);

        if let Ok(ref tree) = x {
            println!("{:?}", tree)
        }
        if let Err(num) = x {
            println!("{:?}", num)
        }
    }

    # [test]
    fn program(){
        let file = fs::read_to_string("./src/program.rs").unwrap();

        let program = file.chars();

        println!("Token {}", program.take_while(|x| x.is_alphanumeric()).collect::<String>());
    }
}