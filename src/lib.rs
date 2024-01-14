#![allow(dead_code)]
mod bfasm;
mod program;

use crate::bfasm::{Type};

// #[derive(Debug)]
// struct Variable (
//     String,
//     // value: Type,
// );

// Todo:
// add += -= as funcs because they are sufficiently different


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
    If(Value, Vec<Statement>),
    Match(Value, Vec<(Type, Vec<Statement>)>),
    While(Value, Vec<Statement>),
    Function(String, Vec<Value>),
    // Assignment(String, Value) // replaced to make arrays it more consistent
    // Return(Value),
}

impl Statement {

    fn print(code: &[Statement]){

        for statement in code {

            match statement {
                Statement::If(ref cond, ref sub_code) => {
                    println!("If {:?}:", cond);
                    Statement::print(sub_code);
                }
                Statement::Match(ref cond, ref match_arms) => {
                    println!("Match {:?}:", cond);

                    for (cond, sub_code) in match_arms{
                        println!("{:?} =>", cond);
                        Statement::print(sub_code)
                    }

                }
                Statement::While(ref cond, ref sub_code) => {
                    println!("While {:?}:", cond);
                    Statement::print(sub_code);
                }
                Statement::Function(_, _) => {
                    println!("{:?}", statement);
                }
            }
        }

    }

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
    // Else,
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
            // Token::Else => {"else"}
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
                    // "else" => Token::Else,
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

                // todo combine

                if let [T::Let, T::Name(ref var), T::Equal] = dbg!(&tokens[index..index+3]) {
                    index += 3;
                    let starting_index = index;

                    while tokens[index] != Token::SemiColon{
                        index+=1;
                    };

                    statements.push(
                        Statement::Function(
                            String::from("="),
                            vec![
                                Value::Var(var.clone()),
                                tokens_to_value(dbg!(&tokens[starting_index..index])).unwrap()
                            ]
                        ));

                    index += 1;

                } else if let [T::Let, T::Mut, T::Name(ref var), T::Equal] = dbg!(&tokens[index..index+4]){
                    index += 4;
                    let starting_index = index;

                    while tokens[index] != Token::SemiColon{
                        index+=1;
                    };

                    statements.push(
                        Statement::Function(
                            String::from("="),
                            vec![
                                Value::Var(var.clone()),
                                tokens_to_value(dbg!(&tokens[starting_index..index])).unwrap()
                            ]
                        ));

                    index += 1;
                }
            }

            Token::While => {

                let starting_index = index + 1; // move past the while token

                dbg!(index);

                while tokens[index] != Token::OpenBrace{
                    index+=1
                };

                let block_index = index;

                index = find_next_balanced(&tokens, index);

                statements.push(Statement::While(
                    tokens_to_value(dbg!(&tokens[starting_index..block_index])).unwrap(),
                    tokens_to_statements(dbg!(&tokens[block_index+1..index])).unwrap() // remove the ending brace
                ));

                index += 1;

                // todo allow a semicolon after if while and match statements
            }

            Token::Name(ref var) => {
                index += 1;

                if let [oper @ (T::Plus | T::Minus), T::Equal, T::Name(ref val), T::SemiColon]
                    = &tokens[index..index + 4] {
                    statements.push(
                        Statement::Function(
                            String::from("="),
                            vec![
                                Value::Var(var.clone()),
                                Value::Func(oper.into(),
                                            vec![
                                                Value::Var(var.clone()),
                                                Value::StaticValue(str_to_type(val).unwrap())
                                            ])
                            ]
                        )
                    );

                    index += 4;
                } else if let [T::OpenBracket, T::Name(ref var_index), T::CloseBracket, T::Equal]
                    = &tokens[index..index + 4] {
                    index += 4;
                    let starting_index = index;

                    while Token::SemiColon != tokens[index] { index += 1; }

                    statements.push(
                        Statement::Function(
                            String::from("[]="),
                            vec![
                                Value::Var(var.clone()),
                                str_to_value(var_index),
                                tokens_to_value(&tokens[starting_index..index]).unwrap()
                            ]
                        )
                    );

                    index += 1;
                }else if let Some([T::OpenBracket, T::Name(ref var_index), T::CloseBracket, oper @ (T::Plus | Token::Minus), T::Equal])
                    = tokens.get(index..index + 5){

                        index += 5;
                        let starting_index = index;

                        while Token::SemiColon != tokens[index] { index += 1; }

                        statements.push(
                            Statement::Function(
                                String::from("[]="),// todo? if oper == T::Plus{"[]+="} else {"[]-="}),
                                vec![
                                    Value::Var(var.clone()),
                                    str_to_value(var_index),
                                    Value::Func(
                                        String::from(oper),
                                        vec![
                                            tokens_to_value(&tokens[starting_index..index]).unwrap(),
                                            Value::Func(
                                                String::from("[]"),
                                                vec![Value::Var(var.clone()), str_to_value(var_index)])
                                        ])
                                ]
                            )
                        );

                        index += 1;

                }else if let T::Dot | T::OpenParens = &tokens[index] {

                    let starting_index = index;

                    while tokens[index] != Token::SemiColon {index += 1;}

                    // let Value::Func(func, args) = dbg!(tokens_to_value(dbg!(&tokens[starting_index-1..index])).unwrap()) else {
                    //     todo!()
                    // };
                    //
                    // statements.push(Statement::Function(func, args));

                    let token = dbg!(&tokens[starting_index-1..index]);

                    match dbg!(tokens_to_value(token)) {
                        Some(Value::Func(func, args)) => {
                            statements.push(Statement::Function(func, args))
                        }

                        val => panic!("{:?}", val)
                    }

                    // statements.push(Statement::Function(
                    //     func_name.clone(),
                    //     if dbg!(index) != dbg!(starting_index) {
                    //         vec![
                    //             Value::Var(var.clone()),
                    //             tokens_to_value(&tokens[index..starting_index]).unwrap()
                    //         ]
                    //     } else {
                    //         vec![Value::Var(var.clone())]
                    //     }
                    // ));

                    index += 1;
                } else if T::OpenParens == tokens[index]{



                } else {panic!()}
            }

            Token::Match => {

                let val_index = index + 1;

                while tokens[index] != Token::OpenBrace{index+=1;}

                let val = tokens_to_value(&tokens[val_index..index]).unwrap();

                let mut clauses = Vec::new();

                index += 1;

                while let Some([T::Name(ref clause_val), T::Equal, T::GreaterThan, T::OpenBrace]) = dbg!(tokens.get(index..index+4)) {


                    index += 3;

                    let clause_index = index;

                    index = find_next_balanced(&tokens, index);

                    dbg!(index, clause_index);

                    if clause_val != "_"{
                        clauses.push((
                            str_to_type(clause_val).unwrap(),
                            tokens_to_statements(dbg!(&tokens[clause_index+1..index])).unwrap()
                        ));
                    } else {
                        assert_eq!(&tokens[clause_index + 1..index], []); // todo
                    }

                    index += 1;

                    if tokens[index] == Token::Comma {index+=1;}
                }

                statements.push(Statement::Match(val, clauses));

                index += 1;

                dbg!(index, &tokens);
            },

            Token::If => {
                index += 1;

                let val_index = index;

                while *dbg!(&tokens[index]) != Token::OpenBrace{index += 1;}

                let code_index = index;

                index = find_next_balanced(&tokens, index);

                statements.push(Statement::If(
                    tokens_to_value(&tokens[val_index..code_index]).unwrap(),
                    tokens_to_statements(&tokens[code_index+1..index]).unwrap(),
                ));

                index += 1;

            },

            token => {
                panic!("{:?} {:?}", token, index)
            }
        };

    }
}

// set index at 1st instance of the value
// the result will be the inverse of the token
fn find_next_balanced(tokens: &[Token], mut index: usize) -> usize {

    let target = &tokens[index];

    let inv_target = match tokens[index] {

        Token::OpenBrace => {Token::CloseBrace}
        Token::OpenParens => {Token::CloseParens}
        Token::OpenBracket => {Token::CloseBracket}
        _ => {unimplemented!()}
    };

    let mut depth = 1;

    index += 1;

    loop {

        if tokens[index] == *target{
            depth += 1;
        } else if tokens[index] == inv_target{
            depth -= 1;
        }

        if depth == 0 {break};

        index += 1;
    }

    index
}

fn tokens_to_value(tokens: &[Token]) -> Option<Value> {

    let mut index = 0;

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
                        Vec::from([Value::Var(str.clone())])
                    }
                )

            } else {panic!()}

        }
        Some(_) => {str_to_value(str)}
    };

    dbg!(&val);

    index += 1; // either then index is still on the str or its on the last bracket or brace from


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
    fn program() {
        let file = fs::read_to_string("./src/program.rs").unwrap();

        let file = &file[file.find("fn main()").unwrap()..];

        let tokens = tokenize(file).unwrap();

        let statements = tokens_to_statements(&tokens[5..tokens.len()-1]).unwrap();

        // println!("{:?}\n{:?}", tokens, statements)

        Statement::print(&statements)
    }

    // # [test]
    // fn program_test(){
    //
    //     use crate::program::*;
    //
    //     main()
    //
    // }

    #[test]
    fn match_test() {
        let code = "match program[program_index]{\
        'a' => {},\
        'b' => {index += 1;}\
        'c' => {\
            if index == 3 {
                index += 1;
            }
        }
        _ => {}
        }";

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap());
    }

    #[test]
    fn if_test() {
        let code = "if program_index < program.len() {
            let x = 5;
            if x {
                x -= 1;
            }
        }";

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap());
    }

    #[test]
    fn var_line() {
        let code = "let array = new_array();\
        array[0] = input();\
        array[x] = array[0];\
        array.push(0);";

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap());
    }

    #[test]
    fn while_test() {
        let code = "while program_index < program.len() {
            let x = 5;
            while x {
                x -= 1;
            }
        }";

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap());
    }

    #[test]
    fn program_lets() {
        let code = "let program = input_str();
        let mut program_index = 0;
        let mut array = new_array();
        let mut array_index = 0;
        let x = program_index < program.len();";

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap());
    }

    #[test]
    fn let_w_func() {
        let code = "let a = f();\
        let b = array[array_index];\
        let mut c = a;";

        // println!("{:?}", tokenize(code).unwrap());

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap());
    }

    # [test]
    fn let_to_statements() {
        let code = "let a = 100;\
        let b = 90;\
        let mut c = a;";

        println!("{:?}", tokens_to_statements(&tokenize(code).unwrap()).unwrap())
    }

    # [test]
    fn let_to_ast() {
        let code = "let a = 100;\
        let b = 90;\
        let c = a;";

        let x = tokenize(code);

        println!("{:?}", x)
    }
}