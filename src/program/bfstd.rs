pub fn input_u32() -> u32 {
    // let mut line = String::new();
    //
    // std::io::stdin().read_line(&mut line).unwrap();
    //
    // return line.chars().next().unwrap() as u32;

    'a' as u32
}

pub fn input_str() -> String {
    // let mut line = String::new();
    //
    // use std::io::{stdin, stdout, Write};
    //
    // print!("Please enter some text: ");
    // let _ = stdout().flush();
    // stdin()
    //     .read_line(&mut line)
    //     .expect("Did not enter a correct string");
    //
    // println!("You typed: {}", &line);
    //
    // line

    String::from("+++[->+<]")
}

pub fn new_array() -> Vec<u32> {
    Vec::new()
}

pub fn print_u32(str: u32) {
    print!("{}", str as u8 as char);
}

// pub fn print_array(string: Vec<u32>) {
//     println!("{:?}", string);
// }
