mod bfasm;
mod program;

use crate::bfasm::{EmptyType, Type};

// #[derive(Debug)]
// struct Variable (
//     String,
//     // value: Type,
// );


// the last type is the return value
// #[derive(Debug)]
// struct Function (
//     Vec<EmptyType>,
//     Vec<Statement>
// );

#[derive(Debug)]
enum Value {
    Var(String),
    Func(String, Vec<Value>),
    StaticValue(Type)
}

#[derive(Debug)]
enum Statement{
    Match(Value, Vec<(Type, Vec<Statement>)>),
    While(Value, Vec<Statement>),
    Function(String, Vec<Value>),
    Assignment(String, Value)
    // Return(Value),
}

#[derive(Debug, PartialEq)]
enum Token {
    Let,
    Equal,
    // DoubleEqual,
    // PlusEquals,
    // MinusEquals,
    SemiColon,
    OpenBrace,
    CloseBrace,
    While,
    If,
    Else,
    Match,
    // Arrow,
    GreaterThan,
    LessThan,
    Comma,
    OpenParens,
    CloseParens,
    Name(String),
    Dot,
    OpenBracket,
    CloseBracket,
    SingleQuote,
    Plus,
    Minus,
    Mut,
}

impl From<&Token> for String {
    fn from(value: &Token) -> Self {
        String::from(Token::to_str(value))
    }
}

impl Token {
    fn to_str(token: &Token) -> &str {
        match token {Token::Let => {"let"}
            Token::Equal => {"="}
            Token::SemiColon => {";"}
            Token::OpenBrace => {"{"}
            Token::CloseBrace => {"}"}
            Token::While => {"while"}
            Token::If => {"if"}
            Token::Else => {"else"}
            Token::Match => {"match"}
            Token::GreaterThan => {">"}
            Token::LessThan => {"<"}
            Token::Comma => {","}
            Token::OpenParens => {"("}
            Token::CloseParens => {")"}
            Token::Name(_) => {todo!()}
            Token::Dot => {"."}
            Token::OpenBracket => {"{"}
            Token::CloseBracket => {"}"}
            Token::SingleQuote => {"'"}
            Token::Plus => {"+"}
            Token::Minus => {"-"}
            Token::Mut => {"mut"}}
    }
}

// impl Token {
//
//     fn as_str(tokens: &[Token]) -> String {
//         tokens.iter().map(<&Token as Into<&str>>::into).collect()
//     }
// }

use std::iter::Enumerate;
use std::str::Chars;
use crate::Statement::Assignment;
// at _ found (_ or nothing), expected _
// struct TokenizeError(usize, Option<char>, String);

// either returns the tokens or the point of failure
fn tokenize(code: &str) -> Option<Vec<Token>> {

    let mut char_iter = code.chars().enumerate();

    let mut tokens = Vec::new();

    loop {

        let Some((mut str, (_, mut char))) = next_word(&mut char_iter)
            else {return Some(tokens)};

        if str != "" {

            tokens.push(

                match str.as_str() {

                    "let" => Token::Let,
                    "while" => Token::While,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "match" => Token::Match,
                    "mut" => Token::Mut,
                    str => Token::Name(String::from(str)),
                }
            );
        }
        if char != ' ' && char != '\n' && char != '\r'{
            tokens.push(
                match char{
                    '=' => Token::Equal,
                    ';' => Token::SemiColon,
                    '{' => Token::OpenBrace,
                    '}' => Token::CloseBrace,
                    '<' => Token::LessThan,
                    '>' => Token::GreaterThan,
                    ',' => Token::Comma,
                    '(' => Token::OpenParens,
                    ')' => Token::CloseParens,
                    '[' => Token::OpenBracket,
                    ']' => Token::CloseBracket,
                    '\'' => {
                        let mut quote = String::new();

                        loop{

                            let source_quote = char_iter.next()?.1;

                            if source_quote == '\'' {
                                break
                            }

                            quote.push(source_quote);

                        }

                        Token::Name(format!("'{}'", quote))
                    },
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '.' => Token::Dot,
                    x => {dbg!(x); todo!()},
                }
            );
        }
    }
}

// returns a alphanumeric string and the non alphanumeric or None if the iter was ended before an
// non alphanumeric char was found
fn next_word(iter: &mut Enumerate<Chars>) -> Option<(String, (usize, char))> {

    let mut str = String::new();

    while let Some((index, char)) = iter.next() {

        if char.is_alphanumeric() || char == '_' {
            str.push(char);
        } else {
            return Some((str, (index, char)));
        }
    }

    None

}

fn to_statements(tokens: &[Token]) -> Result<Vec<Statement>, Option<usize>> {

    use Token as T;

    let mut index = 0;
    let mut starting_index = 0;

    let mut statements = Vec::new();

    loop{

        let Some(current_token) = tokens.get(index) else {
            return Ok(statements)
        };

        match current_token {
            Token::Let => {

                if let [T::Let, T::Name(ref var), T::Equal] = dbg!(&tokens[index..index+3]) {
                    index += 3;
                    starting_index = index;

                    while tokens[index-1] != Token::SemiColon{index+=1};

                    statements.push(Assignment(var.clone(), tokens_to_value(&tokens[starting_index..index]).unwrap()))
                }

                // // let _ = _;
                // if let [T::Let, T::Name(ref var), T::Equal, T::Name(ref value_str), T::SemiColon] = dbg!(&tokens[index..index+5]) {
                //
                //
                //     statements.push(Statement::Assignment(var.clone(), str_to_value(value_str)));
                //
                //     index += 5;
                //
                // } else if let [T::Let, T::Mut, T::Name(ref var), T::Equal, T::Name(ref value_str), T::SemiColon] = dbg!(&tokens[index..index+6]) {
                //
                //     statements.push(Statement::Assignment(var.clone(), str_to_value(value_str)));
                //
                //     index += 6;
                //
                // } else if let [T::Let, T::Name(ref var), T::Equal, T::Name(ref func_str),
                // T::OpenParens | T::OpenBracket] = dbg!(&tokens[index..index+5]) {
                //
                //     index += 5;
                //
                //     if let T::CloseParens | T::CloseBracket = tokens[index] {
                //         statements.push(Statement::Assignment(var.clone(), Value::Func(String::from(func_str), vec![])));
                //         index += 2;
                //     } else if let [T::Name(ref val), T::CloseParens | T::CloseBracket] = tokens[index..index+2] {
                //         statements.push(Statement::Assignment(var.clone(), Value::Func(String::from(func_str), vec![str_to_value(val)])));
                //         index += 3;
                //     } else {
                //         todo!()
                //     }
                // } else {
                //     todo!()
                // }
            }

            Token::While => {

                let starting_index = index;
            }

            _ => {unreachable!()}
        };

    }
}

fn tokens_to_value(tokens: &[Token]) -> Option<Value> {

    let mut index = 2;

    let Token::Name(ref str) = tokens[0] else {
        todo!();
        return None
    };

    let val = match tokens.get(1) {
        None => {
            return Some(str_to_value(str));
        }
        Some(Token::OpenBracket) => {
            while *tokens.get(index-1).unwrap() != Token::CloseBracket {index += 1;};
            Value::Func(
                String::from("[]"),
                vec![Value::Var(str.clone()),
                     tokens_to_value(&tokens[2..index]).unwrap()]
            )
        }
        Some(Token::OpenParens) => {
            while *tokens.get(index-1).unwrap() != Token::CloseParens {index += 1;};
            Value::Func(
                str.clone(),
                if index != 3 {
                    Vec::from([tokens_to_value(&tokens[2..index]).unwrap()])
                } else {Vec::new()}
            )
        }
        Some(_) => {str_to_value(str)}
    };

    use Token as T;

    let Some(operand) = tokens.get(index) else {
        return Some(val);
    };

    Some(match operand {
        oper @ (Token::GreaterThan | Token::LessThan) => {
            Value::Func(String::from(oper), vec![val, tokens_to_value(&tokens[index+1..]).unwrap()])
        }
        Token::Equal => {
            Value::Func(String::from("=="), vec![val, tokens_to_value(&tokens[index+2..]).unwrap()])
        }

        _ => {todo!()}
        // oper @ ([T::GreaterThan, T::Equal] | [T::LessThan, T::Equal] | [T::Equal, T::Equal] | [T::]) => {
        //     Value::Func(Token::into_str(oper), vec![val, tokens_to_value(&tokens[index+2..])?])
        // }


    })
}

// returns a static Type if the str is parseable as a type otherwise returns a var as the str
fn str_to_value(str: &str) -> Value{
    if let Some(bf_type) = str_to_type(str) {
        Value::StaticValue(bf_type)
    } else {
        Value::Var(String::from(str))
    }
}

fn str_to_type(value: &str) -> Option<Type> {

    match value.chars().next()? {
        '\'' => {

            if value.chars().count() != 3 {
                return None
            };

            Some(Type::Char(value.chars().nth(1)? as u8))
        },
        _ => {
            Some(Type::U32(value.parse().ok()?))
        }
    }
}
#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    # [test]
    fn program(){
        let file = fs::read_to_string("./src/program.rs").unwrap();

        let file = &file[file.find("fn main()").unwrap()..];

        let tokens = tokenize(file);

        println!("{:?}", tokens)
    }

    #[test]
    fn let_w_func(){
        let code = "let a = f();\
        let b = array[array_index];\
        let mut c = a;";

        // println!("{:?}", tokenize(code).unwrap());

        println!("{:?}", to_statements(&tokenize(code).unwrap()).unwrap());
    }

    # [test]
    fn let_to_statements(){
        let code = "let a = 100;\
        let b = 90;\
        let mut c = a;";

        println!("{:?}", to_statements(&tokenize(code).unwrap()).unwrap())
    }

    # [test]
    fn let_to_ast(){
        let code = "let a = 100;\
        let b = 90;\
        let c = a;";

        let x = tokenize(code);

        println!("{:?}", x)
    }
}