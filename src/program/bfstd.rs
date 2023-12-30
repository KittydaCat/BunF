pub fn input_char() -> u32 {

    let mut line = String::new();

    use std::io::Write;

    let _ = std::io::stdout().flush();

    let _ = std::io::stdin().read_line(&mut line);

    return line.chars().next().unwrap() as u32;
}

pub fn input_str() -> Vec<char> {
    let mut line = String::new();

    use std::io::Write;

    let _ = std::io::stdout().flush();

    let _ = std::io::stdin().read_line(&mut line);

    line.into_bytes().into_iter().map(|x| x as char).collect()
}

pub fn new_array() -> Vec<u32> {
    Vec::new()
}

pub fn print_u32(str: u32) {

    print!("{}", str as u8 as char);
}