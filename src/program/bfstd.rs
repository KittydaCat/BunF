pub fn input_char() -> u32 {
    let mut line = String::new();

    // use std::io::Write;
    //
    // let _ = std::io::stdout().flush();
    //
    // let _ = std::io::stdin().read_line(&mut line);

    std::io::stdin().read_line(&mut line).unwrap();

    return line.chars().next().unwrap() as u32;
}

pub fn input_str() -> Vec<char> {
    let mut line = String::new();
    //
    // // std::io::stdin().read_to_end()
    //
    // std::io::stdin().read_line(&mut line).unwrap();

    use std::io::{stdin, stdout, Write};

    print!("Please enter some text: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut line)
        .expect("Did not enter a correct string");

    println!("You typed: {}", &line);

    line.chars().collect()
}

pub fn new_array() -> Vec<u32> {
    Vec::new()
}

pub fn print_u32(str: u32) {
    print!("{}", str as u8 as char);
}

pub fn print_array(string: Vec<u32>) {
    println!("{:?}", string);
}
