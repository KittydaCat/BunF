#![allow(dead_code)]
mod bfasm;
mod program;

use crate::bfasm::{Type};

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
// at _ found (_ or nothing), expected _
// struct TokenizeError(usize, Option<char>, String);

// either returns the tokens or the point of failure
fn tokenize(code: &str) -> Option<Vec<Token>> {

    let mut char_iter = code.chars().enumerate();

    let mut tokens = Vec::new();

    loop {

        let Some((str, (_, char))) = next_word(&mut char_iter)
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

fn tokens_to_statements(tokens: &[Token]) -> Result<Vec<Statement>, Option<usize>> {

    use Token as T;

    let mut index = 0;

    let mut statements = Vec::new();

    loop{

        let Some(current_token) = tokens.get(index) else {
            return Ok(statements)
        };

        match current_token {
            Token::Let => {

                if let [T::Let, T::Name(ref var), T::Equal] = dbg!(&tokens[index..index+3]) {
                    index += 3;
                    let starting_index = index;

                    while tokens[index] != Token::SemiColon{
                        index+=1
                    };

                    statements.push(
                        Statement::Assignment(
                            var.clone(),
                            tokens_to_value(dbg!(&tokens[starting_index..index])).unwrap()
                        ));

                    index += 1;
                } else if let [T::Let, T::Mut, T::Name(ref var), T::Equal] = dbg!(&tokens[index..index+4]){
                    index += 4;
                    let starting_index = index;

                    while tokens[index] != Token::SemiColon{
                        index+=1
                    };

                    dbg!(starting_index, index, &tokens);

                    statements.push(
                        Statement::Assignment(
                            var.clone(),
                            tokens_to_value(dbg!(&tokens[starting_index..index])).unwrap()));

                    index += 1;
                }
            }

            Token::While => {

                let starting_index = index + 1; // move past the while token

                dbg!(index);
                dbg!(&tokens);

                while tokens[index] != Token::OpenBrace{
                    index+=1
                };

                let block_index = index;

                index = find_next_balanced(&Token::OpenBrace, &tokens, index);

                statements.push(Statement::While(
                    tokens_to_value(dbg!(&tokens[starting_index..block_index])).unwrap(),
                    tokens_to_statements(dbg!(&tokens[block_index+1..index])).unwrap() // remove the ending brace
                ));

                index += 1;
            }

            token => {panic!("{:?}", token)}
        };

    }
}

// set index ahead of 1st instance of the value
// the result will be the inverse of the token
fn find_next_balanced(target: &Token, tokens: &[Token], mut index: usize) -> usize {

    let inv_target = match target {

        Token::OpenBrace => {Token::CloseBrace}
        Token::OpenParens => {Token::CloseParens}
        Token::OpenBracket => {Token::CloseBracket}
        _ => {unimplemented!()}
    };

    let mut depth = 1;

    while depth > 0{

        index += 1; // move after if statements?

        if tokens[index] == *target{
            depth += 1;
        } else if tokens[index] == inv_target{
            depth -= 1;
        }
    }

    index
}

fn tokens_to_value(tokens: &[Token]) -> Option<Value> {

    let mut index = 1;

    let Token::Name(ref str) = dbg!(&tokens[0]) else {
        dbg!(&tokens[0]);
        todo!();
        // return None
    };

    let val = match dbg!(tokens.get(1)) {
        None => {
            return Some(str_to_value(str));
        }
        Some(Token::OpenBracket) => {
            while *tokens.get(index).unwrap() != Token::CloseBracket {index += 1;};
            Value::Func(
                String::from("[]"),
                vec![Value::Var(str.clone()),
                     tokens_to_value(&tokens[2..index]).unwrap()] // if [] will panic
            )
        }
        Some(Token::OpenParens) => {
            while *tokens.get(index).unwrap() != Token::CloseParens {index += 1;};
            Value::Func(
                str.clone(),
                if index != 2 {
                    Vec::from([tokens_to_value(&tokens[2..index]).unwrap()])
                } else {
                    index += 1;
                    Vec::new()
                }
            )
        }
        Some(Token::Dot) => {

            if let [Token::Name(ref func_name), Token::OpenParens] = &tokens[2..4] {

                while *tokens.get(index).unwrap() != Token::CloseParens {index += 1;}

                dbg!(index);

                Value::Func(
                    func_name.clone(),
                    if index != 4{
                        Vec::from([Value::Var(str.clone()), tokens_to_value(dbg!(&tokens[4..index])).unwrap()])
                    } else {
                        index += 1;
                        Vec::from([Value::Var(str.clone())])
                    }
                )

            } else {panic!()}

        }
        Some(_) => {str_to_value(str)}
    };

    dbg!(&val);


    let Some(operand) = dbg!(tokens.get(index)) else {
        return Some(val);
    };

    Some(match operand {
        oper @ (Token::GreaterThan | Token::LessThan) => {
            Value::Func(String::from(oper), vec![val, tokens_to_value(dbg!(&tokens[index+1..])).unwrap()])
        }
        Token::Equal => {
            Value::Func(String::from("=="), vec![val, tokens_to_value(dbg!(&tokens[index+2..])).unwrap()])
        }



        val => {panic!("{:?}", val);}
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

        let tokens = tokenize(file).unwrap();

        let statements = tokens_to_statements(&tokens[5..tokens.len()-1]).unwrap();

        println!("{:?}\n{:?}", tokens, statements)
    }

    # [test]
    fn program_test(){

        use crate::program::*;

        main()

    }

    #[test]
    fn while_test(){
        let code = "while program_index < program.len() {}";

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap());
    }

    #[test]
    fn program_lets(){
        let code = "let program = input_str();
        let mut program_index = 0;
        let mut array = new_array();
        let mut array_index = 0;
        let x = program_index < program.len();";

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap());
    }

    #[test]
    fn let_w_func(){
        let code = "let a = f();\
        let b = array[array_index];\
        let mut c = a;";

        // println!("{:?}", tokenize(code).unwrap());

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap());
    }

    # [test]
    fn let_to_statements(){
        let code = "let a = 100;\
        let b = 90;\
        let mut c = a;";

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap())
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