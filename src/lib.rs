#![allow(dead_code)]
mod bfasm;
mod program;

use crate::bfasm::{Bfasm, BfasmCode, EmptyType, Type};

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

#[derive(PartialEq, Debug, Clone)]
enum Function {
    IndexStr(String, Value),
    Index(String, Value),
    IndexSet(String, Value, Value),
    Assign(String, Value),
    Add(Value, Value),
    Subtract(Value, Value),
    Equal(Value, Value),
    GreaterThan(Value, Value),
    LessThan(Value, Value),
    Len(String),
    Push(String, Value),
    InputStr,
    NewArray,
    InputChar,
    PrintU32(Value),
    CloneU32(String),
}

impl Function {

    fn return_type(&self) -> Option<EmptyType> {
        match self {
            Function::IndexStr(_, _) => {Some(EmptyType::Char)}
            Function::Index(_, _) => {Some(EmptyType::U32)}
            Function::IndexSet(_, _, _) => {None}
            Function::Assign(_, _) => {None}
            Function::Add(_, _) => {Some(EmptyType::Bool)}
            Function::Subtract(_, _) => {Some(EmptyType::Bool)}
            Function::Equal(_, _) => {Some(EmptyType::Bool)}
            Function::GreaterThan(_, _) => {Some(EmptyType::Bool)}
            Function::LessThan(_, _) => {Some(EmptyType::Bool)}
            Function::Len(_) => {Some(EmptyType::Bool)}
            Function::Push(_, _) => {None}
            Function::InputStr => {Some(EmptyType::IString)}
            Function::NewArray => {Some(EmptyType::Array)}
            Function::InputChar => {Some(EmptyType::Char)}
            Function::PrintU32(_) => {None}
            Function::CloneU32(_) => {Some(EmptyType::U32)}
        }
    }
}

impl Function {
    fn parens_call(fn_name: &str, value: Option<Value>) -> Self {
        match fn_name {
            "input_str" => {
                assert_eq!(value, None);
                Function::InputStr
            }
            "new_array" => {
                assert_eq!(value, None);
                Function::NewArray
            }
            "input_char" => {
                assert_eq!(value, None);
                Function::InputChar
            }
            "print_u32" => {
                if let Some(val) = value {
                    Function::PrintU32(val)
                } else {
                    panic!()
                }
            }
            _ => {
                panic!("Unknown function name: {}", fn_name)
            }
        }
    }

    // fn dot_call(fn_name: &str, var: &str, value: Option<Value>) -> Self {
    //     match fn_name {
    //         "len" => {
    //             assert_eq!(value, None);
    //             Function::Len(String::from(var))
    //         }
    //         "push" => {
    //             if let Some(val) = value {
    //                 Function::Push(String::from(var), val)
    //             } else {
    //                 panic!()
    //             }
    //         }
    //         _ => {
    //             panic!("Unknown function name: {}", fn_name)
    //         }
    //     }
    // }

    // amount of space after the variable needed for the function
    // including EC and values passed into the function
    fn len(&self) -> Option<usize> {
        match self {
            Function::Index(_, _) => {Some(2)}
            Function::IndexSet(_, _, _) => {Some(2)}
            Function::Assign(_, _) => {Some(0)}
            Function::Add(_, _) => {Some(0)}
            Function::Subtract(_, _) => {Some(0)}
            Function::Equal(_, _) => {Some(4)}
            Function::GreaterThan(_, _) => {Some(4)}
            Function::LessThan(_, _) => {Some(4)}
            Function::Len(_) => {Some(2)}
            Function::Push(_, _) => {Some(2)}
            Function::InputStr => {None} // ???
            Function::NewArray => {None} // 3?
            Function::InputChar => {Some(0)} // 1?
            Function::PrintU32(_) => {Some(0)}
            Function::CloneU32(_) => {Some(2)}
            Function::IndexStr(_, _) => {Some(2)}
        }
    }
}
#[derive(PartialEq, Debug, Clone)]
enum Value {
    Func(Box<Function>),
    Static(Type),
}

impl Value {

    fn bftype(&self) -> EmptyType {

        match self {
            Value::Func(func) => {
                (**func).return_type().unwrap()
            }
            Value::Static(bftype) => {
                EmptyType::from(bftype)
            }
        }
    }

}

#[derive(Debug)]
enum Statement {
    If(Value, Vec<Statement>),
    Match(Value, Vec<(Type, Vec<Statement>)>),
    While(Value, Vec<Statement>),
    Function(Function),
    // Assignment(String, Value) // replaced to make arrays it more consistent
    // Return(Value),
}

impl Statement {
    fn print(code: &[Statement]) {
        for statement in code {
            match statement {
                Statement::If(ref cond, ref sub_code) => {
                    println!("If {:?}:", cond);
                    Statement::print(sub_code);
                }
                Statement::Match(ref cond, ref match_arms) => {
                    println!("Match {:?}:", cond);

                    for (cond, sub_code) in match_arms {
                        println!("{:?} =>", cond);
                        Statement::print(sub_code)
                    }
                }
                Statement::While(ref cond, ref sub_code) => {
                    println!("While {:?}:", cond);
                    Statement::print(sub_code);
                }
                Statement::Function(_) => {
                    println!("{:?}", statement);
                }
            }
        }
    }
}

type Variable = (String, EmptyType, usize);

struct Scope<'a> {
    current: Vec<Variable>,
    above: Option<&'a mut Scope<'a>>
}

#[derive(Debug)]
enum AnnotatedStatement {
    If(Value, (Vec<AnnotatedStatement>, Vec<Variable>)),
    Match(Value, Vec<(Type, (Vec<AnnotatedStatement>, Vec<Variable>))>),
    While(Value, (Vec<AnnotatedStatement>, Vec<Variable>)),
    Function(Function),
}

impl AnnotatedStatement {
    fn print(code: &[AnnotatedStatement]) {
        for statement in code {
            match statement {
                AnnotatedStatement::If(ref cond, ref sub_code) => {
                    println!("If {:?}:", cond);
                    AnnotatedStatement::print(&sub_code.0);
                }
                AnnotatedStatement::Match(ref cond, ref match_arms) => {
                    println!("Match {:?}:", cond);

                    for (cond, sub_code) in match_arms {
                        println!("{:?} =>", cond);
                        AnnotatedStatement::print(&sub_code.0)
                    }
                }
                AnnotatedStatement::While(ref cond, ref sub_code) => {
                    println!("While {:?}:", cond);
                    AnnotatedStatement::print(&sub_code.0);
                }
                AnnotatedStatement::Function(_) => {
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

// impl From<&Token> for String {
//     fn from(value: &Token) -> Self {
//         String::from(match value {
//             Token::Let => "let",
//             Token::Equal => "=",
//             Token::SemiColon => ";",
//             Token::OpenBrace => "{",
//             Token::CloseBrace => "}",
//             Token::While => "while",
//             Token::If => "if",
//             Token::Match => "match",
//             Token::GreaterThan => ">",
//             Token::LessThan => "<",
//             Token::Comma => ",",
//             Token::OpenParens => "(",
//             Token::CloseParens => ")",
//             Token::Name(name) => {
//                 dbg!("why?");
//                 return name.clone();
//             }
//             Token::Dot => ".",
//             Token::OpenBracket => "{",
//             Token::CloseBracket => "}",
//             Token::Plus => "+",
//             Token::Minus => "-",
//             Token::Mut => "mut",
//         })
//     }
// }

// at _ found (_ or nothing), expected _
// struct TokenizeError(usize, Option<char>, String);

// either returns the tokens or the point of failure
fn tokenize(code: &str) -> Option<Vec<Token>> {
    let mut char_iter = code.chars().enumerate();

    let mut tokens = Vec::new();

    loop {
        let Some((str, (_, char))) = next_word(&mut char_iter) else {
            return Some(tokens);
        };

        if !str.is_empty() {
            tokens.push(match str.as_str() {
                "let" => Token::Let,
                "while" => Token::While,
                "if" => Token::If,
                // "else" => Token::Else,
                "match" => Token::Match,
                "mut" => Token::Mut,
                str => Token::Name(String::from(str)),
            });
        }
        if char != ' ' && char != '\n' && char != '\r' {
            tokens.push(match char {
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

                    loop {
                        let source_quote = char_iter.next()?.1;

                        if source_quote == '\'' {
                            break;
                        }

                        quote.push(source_quote);
                    }

                    Token::Name(format!("'{}'", quote))
                }
                '+' => Token::Plus,
                '-' => Token::Minus,
                '.' => Token::Dot,
                _val => {
                    panic!("Unknown non-alphanumeric char: {_val}")
                }
            });
        }
    }
}

// returns an alphanumeric string and the non-alphanumeric or None if the iter was ended before a
// non-alphanumeric char was found
fn next_word(iter: &mut std::iter::Enumerate<std::str::Chars>) -> Option<(String, (usize, char))> {
    let mut str = String::new();

    for (index, char) in iter.by_ref() {
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

    loop {
        let Some(current_token) = tokens.get(index) else {
            return Ok(statements);
        };

        match current_token {
            Token::Let => {
                // todo combine

                if let [T::Let, T::Name(ref var), T::Equal] = &tokens[index..index + 3] {
                    index += 3;
                    let starting_index = index;

                    while tokens[index] != Token::SemiColon {
                        index += 1;
                    }

                    statements.push(Statement::Function(Function::Assign(
                        var.clone(),
                        tokens_to_value(&tokens[starting_index..index]).unwrap(),
                    )));

                    index += 1;
                } else if let [T::Let, T::Mut, T::Name(ref var), T::Equal] =
                    &tokens[index..index + 4]
                {
                    index += 4;
                    let starting_index = index;

                    while tokens[index] != Token::SemiColon {
                        index += 1;
                    }

                    statements.push(Statement::Function(Function::Assign(
                        var.clone(),
                        tokens_to_value(&tokens[starting_index..index]).unwrap(),
                    )));

                    index += 1;
                }
            }

            Token::While => {
                let starting_index = index + 1; // move past the while token

                while tokens[index] != Token::OpenBrace {
                    index += 1
                }

                let block_index = index;

                index = find_next_balanced(tokens, index);

                statements.push(Statement::While(
                    tokens_to_value(&tokens[starting_index..block_index]).unwrap(),
                    tokens_to_statements(&tokens[block_index + 1..index]).unwrap(), // remove the ending brace
                ));

                index += 1;

                // todo allow a semicolon after if while and match statements
            }

            Token::Name(ref var) => {
                index += 1;

                // ex: x += 1;
                if let [oper @ (T::Plus | T::Minus), T::Equal, T::Name(ref val), T::SemiColon] =
                    &tokens[index..index + 4]
                {
                    statements.push(Statement::Function(Function::Assign(
                        var.clone(),
                        Value::Func(if *oper == T::Plus {
                            Box::new(Function::Add(
                                Value::Func(Box::new(Function::CloneU32(String::from(var)))),
                                Value::Static(str_to_type(val).unwrap()),
                            ))
                        } else if *oper == T::Minus {
                            Box::new(Function::Subtract(
                                Value::Func(Box::new(Function::CloneU32(String::from(var)))),
                                Value::Static(str_to_type(val).unwrap()),
                            ))
                        } else {
                            unreachable!()
                        }),
                    )));

                    index += 4;

                    // ex: x[1] = ..;
                } else if let [T::OpenBracket, T::Name(ref var_index), T::CloseBracket, T::Equal] =
                    &tokens[index..index + 4]
                {
                    index += 4;
                    let starting_index = index;

                    while Token::SemiColon != tokens[index] {
                        index += 1;
                    }

                    statements.push(Statement::Function(Function::IndexSet(
                        var.clone(),
                        str_to_value(var_index),
                        tokens_to_value(&tokens[starting_index..index]).unwrap(),
                    )));

                    index += 1;
                // ex x[1] += ..;
                } else if let Some(
                    [T::OpenBracket, T::Name(ref var_index), T::CloseBracket, oper @ (T::Plus | Token::Minus), T::Equal],
                ) = tokens.get(index..index + 5)
                {
                    index += 5;
                    let starting_index = index;

                    while Token::SemiColon != tokens[index] {
                        index += 1;
                    }

                    statements.push(Statement::Function(Function::IndexSet(
                        var.clone(),
                        str_to_value(var_index),
                        Value::Func(Box::from(if *oper == T::Plus {
                            Function::Add(
                                tokens_to_value(&tokens[starting_index..index]).unwrap(),
                                Value::Func(Box::new(Function::Index(
                                    var.clone(),
                                    str_to_value(var_index),
                                ))),
                            )
                        } else if *oper == T::Minus {
                            Function::Subtract(
                                tokens_to_value(&tokens[starting_index..index]).unwrap(),
                                Value::Func(Box::from(Function::Index(
                                    var.clone(),
                                    str_to_value(var_index),
                                ))),
                            )
                        } else {
                            unreachable!()
                        })),
                    )));

                    index += 1;
                // ex: x. or x(
                } else if let T::Dot | T::OpenParens = &tokens[index] {
                    let starting_index = index;

                    while tokens[index] != Token::SemiColon {
                        index += 1;
                    }

                    // let Value::Func(func, args) = dbg!(tokens_to_value(dbg!(&tokens[starting_index-1..index])).unwrap()) else {
                    //     todo!()
                    // };
                    //
                    // statements.push(Statement::Function(func, args));

                    let token = &tokens[starting_index - 1..index];

                    match tokens_to_value(token) {
                        Some(Value::Func(func)) => statements.push(Statement::Function(*func)),

                        val => panic!("{:?}", val),
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
                } else {
                    panic!()
                }
            }

            Token::Match => {
                let val_index = index + 1;

                while tokens[index] != Token::OpenBrace {
                    index += 1;
                }

                let val = tokens_to_value(&tokens[val_index..index]).unwrap();

                let mut clauses = Vec::new();

                index += 1;

                while let Some([T::Name(ref clause_val), T::Equal, T::GreaterThan, T::OpenBrace]) =
                    tokens.get(index..index + 4)
                {
                    index += 3;

                    let clause_index = index;

                    index = find_next_balanced(tokens, index);

                    if clause_val != "_" {
                        clauses.push((
                            str_to_type(clause_val).unwrap(),
                            tokens_to_statements(&tokens[clause_index + 1..index]).unwrap(),
                        ));
                    } else {
                        assert_eq!(&tokens[clause_index + 1..index], []); // todo
                    }

                    index += 1;

                    if tokens[index] == Token::Comma {
                        index += 1;
                    }
                }

                statements.push(Statement::Match(val, clauses));

                index += 1;
            }

            Token::If => {
                index += 1;

                let val_index = index;

                while tokens[index] != Token::OpenBrace {
                    index += 1;
                }

                let code_index = index;

                index = find_next_balanced(tokens, index);

                statements.push(Statement::If(
                    tokens_to_value(&tokens[val_index..code_index]).unwrap(),
                    tokens_to_statements(&tokens[code_index + 1..index]).unwrap(),
                ));

                index += 1;
            }

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
        Token::OpenBrace => Token::CloseBrace,
        Token::OpenParens => Token::CloseParens,
        Token::OpenBracket => Token::CloseBracket,
        _ => {
            unimplemented!()
        }
    };

    let mut depth = 1;

    index += 1;

    loop {
        if tokens[index] == *target {
            depth += 1;
        } else if tokens[index] == inv_target {
            depth -= 1;
        }

        if depth == 0 {
            break;
        };

        index += 1;
    }

    index
}

fn tokens_to_value(tokens: &[Token]) -> Option<Value> {
    let mut index = 0;

    let Some(Token::Name(ref str)) = tokens.first() else {
        return None;
    };

    let val = match tokens.get(1) {
        None => {
            return Some(str_to_value(str));
        }
        Some(Token::OpenBracket) => {
            while *tokens.get(index).unwrap() != Token::CloseBracket {
                index += 1;
            }
            Value::Func(Box::from(Function::Index(
                str.clone(),
                tokens_to_value(&tokens[2..index]).unwrap(), // if [] will panic
            )))
        }
        Some(Token::OpenParens) => {
            while *tokens.get(index).unwrap() != Token::CloseParens {
                index += 1;
            }
            Value::Func(Box::from(Function::parens_call(
                str,
                tokens_to_value(&tokens[2..index]),
            )))
        }
        Some(Token::Dot) => {
            if let [Token::Name(ref func_name), Token::OpenParens] = &tokens[2..4] {
                while *tokens.get(index).unwrap() != Token::CloseParens {
                    index += 1;
                }

                // Value::Func(Box::from(Function::dot_call(
                //     func_name,
                //     str,
                //     tokens_to_value(&tokens[4..index]),
                // )))

                let value = tokens_to_value(&tokens[4..index]);

                Value::Func(Box::new(match func_name.as_str() {
                    "len" => {
                        assert_eq!(value, None);
                        Function::Len(String::from(str))
                    }
                    "push" => {
                        if let Some(val) = value {
                            Function::Push(String::from(str), val)
                        } else {
                            panic!()
                        }
                    }
                    "chars" => {
                        use Token as T;
                        if let [T::Dot, T::Name(nth), T::OpenParens, T::Name(val), T::CloseParens, T::Dot,
                        T::Name(unwrap), T::OpenParens, T::CloseParens] = &tokens[index+1..index+10] {
                            assert_eq!(value, None);
                            assert_eq!(nth, "nth");
                            assert_eq!(unwrap, "unwrap");

                            index += 9;

                            Function::IndexStr(String::from(str), str_to_value(val))
                        } else {
                            panic!()
                        }
                    }
                    _ => {
                        panic!("Unknown function name: {}", func_name)
                    }
                }))
            } else {
                panic!()
            }
        }
        Some(_) => str_to_value(str),
    };

    index += 1; // either then index is still on the str or its on the last bracket or brace from

    let Some(operand) = tokens.get(index) else {
        return Some(val);
    };

    Some(match operand {
        Token::GreaterThan => Value::Func(Box::from(Function::GreaterThan(
            val,
            tokens_to_value(&tokens[index + 1..]).unwrap(),
        ))),
        Token::LessThan => Value::Func(Box::from(Function::LessThan(
            val,
            tokens_to_value(&tokens[index + 1..]).unwrap(),
        ))),
        Token::Equal => Value::Func(Box::from(Function::Equal(
            val,
            tokens_to_value(&tokens[index + 2..]).unwrap(),
        ))),

        val => {
            panic!("{:?}", val);
        } // oper @ ([T::GreaterThan, T::Equal] | [T::LessThan, T::Equal] | [T::Equal, T::Equal] | [T::]) => {
          //     Value::Func(Token::into_str(oper), vec![val, tokens_to_value(&tokens[index+2..])?])
          // }
    })
}

// returns a static Type if the str is parseable as a type otherwise returns a var as the str
fn str_to_value(str: &str) -> Value {
    if let Some(bf_type) = str_to_type(str) {
        Value::Static(bf_type)
    } else {
        Value::Func(Box::new(Function::CloneU32(String::from(str))))
    }
}

fn str_to_type(value: &str) -> Option<Type> {
    match value.chars().next()? {
        '\'' => {
            if value.chars().count() != 3 {
                return None;
            };

            Some(Type::Char(value.chars().nth(1)? as u8))
        }
        _ => Some(Type::U32(value.parse().ok()?)),
    }
}

// lables each variable with the amount of space it needs
fn annotate_statements(statements: &[Statement], scope: &mut Vec<Vec<Variable>>)
                           -> (Vec<AnnotatedStatement>, Vec<Variable>) {

    scope.push(Vec::new());

    let anno_states = statements.iter().map(|statement| {

        match statement {
            Statement::If(val, code) => {
                // annotate_value(val, &mut current_scope);

                let statement2 = annotate_statements(code, scope);

                AnnotatedStatement::If(val.clone(), statement2)

            }
            Statement::Match(val, match_arms) => {
                annotate_value(val, scope);

                // let scope = Some(&mut current_scope);

                // let anno_arms = match_arms.iter().map(
                //     move |(bftype, statements)|(bftype.clone(), annotate_statements(statements, scope))
                // ).collect();

                let mut anno_arms = Vec::new();

                for (bftype, statements) in match_arms {
                    anno_arms.push((bftype.clone(), annotate_statements(statements, scope)));
                }

                AnnotatedStatement::Match(val.clone(), anno_arms)
            }
            Statement::While(val, code) => {
                annotate_value(val, scope);
                AnnotatedStatement::While(val.clone(), annotate_statements(code, scope))
            }
            Statement::Function(func) => {
                annotate_func(func, scope);
                AnnotatedStatement::Function(func.clone())
            }
        }

    }).collect::<Vec<AnnotatedStatement>>();

    (anno_states, scope.pop().unwrap())
}

fn annotate_value(value: &Value, scope: &mut [Vec<Variable>]) {

    match value{
        Value::Func(func) => {
            annotate_func(func, scope)
        }
        Value::Static(_) => {}
    }
}

fn annotate_func(func: &Function, scope: &mut [Vec<Variable>]) {

    match func {
        Function::Assign(var, val) => {
            annotate_value(val, scope);
            match increase_req_space(scope, var, 0) {
                None => {scope.last_mut().unwrap().push((var.clone(), val.bftype(), 0))}
                Some(_) => {}
            }
        }

        Function::Len(var) | Function::CloneU32(var) => {
            increase_req_space(scope, var, 2).unwrap();
        }

        Function::PrintU32(val) => {
            annotate_value(val, scope);
        }

        Function::Index(var, val) | Function::Push(var, val) | Function::IndexStr(var, val) => {
            increase_req_space(scope, var, 2).unwrap();
            annotate_value(val, scope);
        }

        Function::Add(val1, val2) |
        Function::Subtract(val1, val2) |
        Function::Equal(val1, val2) |
        Function::GreaterThan(val1, val2) |
        Function::LessThan(val1, val2) => {
            annotate_value(val1, scope);
            annotate_value(val2, scope);
        }

        Function::IndexSet(var, val1, val2) => {
            annotate_value(val1, scope);
            annotate_value(val2, scope);
            increase_req_space(scope, var, 2).unwrap();
        }
        Function::InputStr | Function::NewArray | Function::InputChar => {}
    }
}

// returns Some if the value was updated or false if the value wasn't found
// we can just unwrap :|
fn increase_req_space(scope: &mut [Vec<Variable>], var_name: &str, min_val: usize) -> Option<()>{

    // for var in *scope.current {
    //     if *var.0 == var {
    //         var.2 = std::cmp::max(var.2, min_val);
    //         break
    //     }
    // }

    if let Some(var) = scope.last_mut()?.iter_mut().find(|(name, _, _)| name == var_name) {

        var.2 = std::cmp::max(var.2, min_val);

        Some(())

    } else {

        let second_last = scope.len()-1;

        // Todo will panic?
        increase_req_space(&mut scope[0..second_last], var_name, min_val)
    }
}

fn annostatements_to_bfasm(
    bf_array: &mut Vec<(Option<String>, EmptyType)>,
    anno_states: &(Vec<AnnotatedStatement>, Vec<Variable>)
) -> BfasmCode {

    anno_states.0.iter().flat_map(|statement| -> BfasmCode {
        match statement {
            AnnotatedStatement::If(_val, _code) => {todo!()}
            AnnotatedStatement::Match(_val, _code) => {todo!()}
            AnnotatedStatement::While(_val, _code) => {todo!()}
            AnnotatedStatement::Function(func) => {
                match func {
                    Function::Assign(var_name, val) => {

                        if let Some(array_index) = search_bf(bf_array, var_name) {

                        } else {

                            anno_states.1.iter().find(|(str, _, _)| str == var_name).unwrap();

                        }
                    }
                    Function::IndexSet(_, _, _) => {todo!()}
                    Function::Push(_, _) => {todo!()}
                    Function::PrintU32(_) => {todo!()}
                    func => {panic!("{:?}", func)}
                }
                todo!()
            }
        }
    }).collect()

}

fn eval_value(value: &Value, bf_array: &mut Vec<(Option<String>, EmptyType)>) -> BfasmCode {

    match value {
        Value::Func(func) => {
            match &**func {
                func @ (Function::IndexStr(var_name, val) | Function::Index(var_name, val)) => {
                    let mut code = eval_value(val, bf_array);

                    let (var_index, (_, EmptyType::Array | EmptyType::FString | EmptyType::IString))
                        = search_bf(bf_array, var_name).unwrap() else {panic!()};

                    let val_index = bf_array.len()-1;

                    match func {
                        Function::IndexStr(_, _) => {
                            code.push(
                                Box::new(move |x| {
                                    x.move_type(val_index, var_index + 1)?;
                                    x.index_str(var_index)?;
                                    x.move_type(var_index+1, val_index)
                                }));
                        }
                        Function::Index(_, _) => {
                            code.push(
                                Box::new(move |x| {
                                    x.move_type(val_index, var_index + 1)?;
                                    x.array_index_back(var_index)?;
                                    x.move_type(var_index+1, val_index)
                                }));
                        }
                        _ => {unreachable!()}
                    };

                    let len = bf_array.len()-1;

                    bf_array[len] = (None, EmptyType::Char);

                    code
                }
                func @ (Function::Add(val1, val2) | Function::Subtract(val1, val2)) => {
                    let mut code = eval_value(val1, bf_array);

                    code.append(&mut eval_value(val2, bf_array));

                    let target_index = bf_array.len() - 2;

                    assert_eq!([(None, EmptyType::U32), (None, EmptyType::U32)], bf_array[target_index..bf_array.len()]);

                    bf_array.pop();
                    bf_array[target_index] = (None, EmptyType::U32);

                    match func {
                        Function::Add(_, _) => {
                            code.push(Box::new(move |x| x.add_u32(target_index)));
                        },
                        Function::Subtract(_, _) => {
                            code.push(Box::new(move |x| x.unsafe_sub_u32(target_index)));
                        },
                        _ => {unreachable!()}
                    };

                    code
                }
                func @ (Function::Equal(val1, val2) | Function::GreaterThan(val1, val2) | Function::LessThan(val1, val2)) => {

                    // let (mut bf_func, oper): (Box<&fn(_, _) -> _>, fn(_, _) -> _) = match func {
                    //     Function::Equal(_, _) => (Box::new(&(bfasm::Bfasm::equals as fn(_, _) -> _)), PartialEq::eq),
                    //     Function::GreaterThan(_, _) => (Box::new(&(bfasm::Bfasm::greater_than as fn(_, _) -> _)), PartialOrd::gt),
                    //     Function::LessThan(_, _) => (Box::new(&(bfasm::Bfasm::less_than as fn(_, _) -> _)), PartialOrd::lt),
                    //     _ => unreachable!()
                    // };

                    let mut code = eval_value(val1, bf_array);
                    bf_array.push((None, EmptyType::EmptyCell));
                    code.append(&mut eval_value(val2, bf_array));

                    let target_index = bf_array.len() - 3;

                    assert_eq!([(None, EmptyType::U32), (None, EmptyType::EmptyCell), (None, EmptyType::U32)],
                        bf_array[target_index..bf_array.len()]);

                    match func {
                        Function::GreaterThan(_, _) => {
                            code.push(Box::new(move |x| Bfasm::greater_than(x, target_index)));
                        },
                        Function::LessThan(_, _) => {
                            code.push(Box::new(move |x| Bfasm::less_than(x, target_index)));
                        },
                        Function::Equal(_, _) => {
                            code.push(Box::new(move |x| Bfasm::equals(x, target_index)));
                        },
                        _ => {unreachable!()}
                    };

                    bf_array.pop();bf_array.pop();bf_array.pop();

                    bf_array.push((None, EmptyType::Bool));

                    code
                }
                Function::Len(var_name) => {
                    let (str_index, (_, EmptyType::IString | EmptyType::FString | EmptyType::Array)) = search_bf(bf_array, var_name)
                        .unwrap() else {panic!()};

                    let copy_index = str_index + 1;

                    let target_index = bf_array.len();

                    vec![
                        Box::new(move |x| x.get_len(str_index)),
                        Box::new(move |x| x.move_type(copy_index, target_index))
                    ]
                }
                Function::InputStr => {
                    let target_index = bf_array.len();

                    // Todo add non default values

                    bf_array.push((None, EmptyType::IString));

                    vec![
                        Box::new(move |x| x.input(target_index, Type::from(String::new())))
                    ]
                }
                Function::NewArray => {
                    let target_index = bf_array.len();

                    bf_array.push((None, EmptyType::Array));

                    vec![
                        Box::new(move |x| x.input(target_index, Type::from(Vec::new())))
                    ]
                }
                Function::InputChar => {
                    let target_index = bf_array.len();
                    // todo? Add options for defaults
                    bf_array.push((None, EmptyType::Char));

                    vec![
                        Box::new(move |x| x.input(target_index, Type::from('a')))
                    ]
                }
                Function::CloneU32(var_name) => {
                    let (target_u32, (_, EmptyType::U32)) = search_bf(bf_array, var_name).unwrap() else {panic!()};

                    let target = target_u32;

                    let copy_target = target_u32 + 1;

                    let goal_index = bf_array.len();

                    bf_array.push((None, EmptyType::U32));

                    vec![
                        Box::new(move |x| x.copy_val(target)),
                        Box::new(move |x| x.move_type(copy_target, goal_index))
                    ]
                }
                ref func => panic!("{:?}", func)
            }
        }
        Value::Static(val) => {

            let index = bf_array.len();
            let var =  val.clone();
            vec![Box::new(move |x| x.set(index, var.clone()))]
        }
    }
}

fn search_bf<'a>(bf_array: &'a mut Vec<(Option<String>, EmptyType)>, var_name: &str)
                 -> Option<(usize, &'a (Option<String>, EmptyType))> {
    bf_array.iter().enumerate().find(
        |(_, (x, _))| if let Some(str) = x { str == var_name} else {false})
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn program() {
        let file = fs::read_to_string("./src/program.rs").unwrap();

        let file = &file[file.find("fn main()").unwrap()..];

        let tokens = tokenize(file).unwrap();

        let statements = tokens_to_statements(&tokens[5..tokens.len() - 1]).unwrap();

        let mut vec = Vec::new();

        let anno = annotate_statements(&statements, &mut vec);

        dbg!(vec);

        dbg!(anno);

        // println!("{:?}\n{:?}", tokens, statements)

        // Statement::print(&statements)
    }

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

        println!(
            "{:?}",
            tokens_to_statements(&tokenize(code).unwrap()).unwrap()
        );
    }

    #[test]
    fn if_test() {
        let code = "if program_index < program.len() {
            let x = 5;
            if x {
                x -= 1;
            }
        }";

        println!(
            "{:?}",
            tokens_to_statements(&tokenize(code).unwrap()).unwrap()
        );
    }

    #[test]
    fn var_line() {
        let code = "let array = new_array();\
        array[0] = input_str();\
        array[x] = array[0];\
        array.push(0);";

        println!(
            "{:?}",
            tokens_to_statements(&tokenize(code).unwrap()).unwrap()
        );
    }

    #[test]
    fn while_test() {
        let code = "while program_index < program.len() {
            let x = 5;
            while x {
                x -= 1;
            }
        }";

        println!(
            "{:?}",
            tokens_to_statements(&tokenize(code).unwrap()).unwrap()
        );
    }

    #[test]
    fn program_lets() {
        let code = "let program = input_str();
        let mut program_index = 0;
        let mut array = new_array();
        let mut array_index = 0;
        let x = program_index < program.len();";

        println!(
            "{:?}",
            tokens_to_statements(&tokenize(code).unwrap()).unwrap()
        );
    }

    #[test]
    fn let_w_func() {
        let code = "let a = input_str();\
        let b = array[array_index];\
        let mut c = a;";

        // println!("{:?}", tokenize(code).unwrap());

        println!(
            "{:?}",
            tokens_to_statements(&tokenize(code).unwrap()).unwrap()
        );
    }

    #[test]
    fn let_to_statements() {
        let code = "let a = 100;\
        let b = 90;\
        let mut c = a;";

        println!(
            "{:?}",
            tokens_to_statements(&tokenize(code).unwrap()).unwrap()
        )
    }

    #[test]
    fn let_to_ast() {
        let code = "let a = 100;\
        let b = 90;\
        let c = a;";

        let x = tokenize(code);

        println!("{:?}", x)
    }
}
