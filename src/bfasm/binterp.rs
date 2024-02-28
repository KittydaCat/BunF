#[derive(Debug, Clone)]
pub enum BFError {
    UnbalancedBrackets,
    NegativeArrayPointer,
    NonASCIIChar,
    InvalidInstructionIndex,
    NegativeCellValue,
    InputFailed,
    OutputFailed,
}

// impl fmt::Display for BFError{
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//
//         let error_string = match self{
//             BFError::UnbalancedBrackets => {"UnbalancedBrackets"}
//             BFError::NegativeArrayPointer => {"NegativeArrayPointer"}
//             BFError::NonASCIIChar => {"NonASCIIChar"}
//             BFError::InvalidInstructionIndex => {"InvalidInstructionIndex"}
//             BFError::InputFailed => {"InputFailed"}
//             BFError::OutputFailed => {"OutputFailed"}
//         };
//         write!(f, "{}", error_string)
//     }
// }

#[derive(Debug, Clone)]
pub struct BFInterpreter {
    pub array: Vec<u32>,
    pub array_pointer: usize,

    pub program: String,
    pub program_index: usize,

    pub input: Box<dyn FnMut() -> Result<char, BFError>>,
    pub output: Box<dyn FnMut(char) -> Result<(), BFError>>,
}

impl BFInterpreter {
    pub fn new(
        program: String,
        input: Box<dyn FnMut() -> Result<char, BFError>>,
        output: Box<dyn FnMut(char) -> Result<(), BFError>>,
    ) -> Self {
        // change to create?

        Self {
            array: vec![0_u32],
            array_pointer: 0,
            program,
            program_index: 0,
            input,
            output,
        }
    }

    pub fn run(&mut self) -> Result<(), BFError> {
        run_bf(
            &mut self.array,
            &mut self.array_pointer,
            &self.program,
            &mut *self.input,
            &mut *self.output,
            &mut self.program_index,
        )
    }

    pub fn exec_one(&mut self) -> Result<(), BFError> {
        if let Some(instruction) = self.program.chars().nth(self.program_index) {
            exec_bf_instruction(
                &mut self.array,
                &mut self.array_pointer,
                &self.program,
                &mut *self.input,
                &mut *self.output,
                &mut self.program_index,
                instruction,
            )?;

            self.program_index += 1;

            Ok(())
        } else {
            Err(BFError::InvalidInstructionIndex)
        }
    }

    // pub fn reset(&mut self) ->(){
    //     self.array = vec![0_u32];
    //     self.array_pointer = 0;
    //     self.program_index = 0;
    // }
}

pub fn run_bf(
    array: &mut Vec<u32>,
    array_index: &mut usize,
    instructions: &str,
    input: &mut dyn FnMut() -> Result<char, BFError>,
    output: &mut dyn FnMut(char) -> Result<(), BFError>,
    instruct_index: &mut usize,
) -> Result<(), BFError> {
    while *array_index >= array.len() {
        array.push(0);
    } // make sure the index is valid

    while let Some(instruction) = instructions.chars().nth(*instruct_index) {
        exec_bf_instruction(
            array,
            array_index,
            instructions,
            input,
            output,
            instruct_index,
            instruction,
        )?;

        *instruct_index += 1;
    }
    Ok(())
}

pub fn exec_bf_instruction(
    array: &mut Vec<u32>,
    array_index: &mut usize,
    instructions: &str,
    input: &mut dyn FnMut() -> Result<char, BFError>,
    output: &mut dyn FnMut(char) -> Result<(), BFError>,
    instruct_index: &mut usize,
    instruction: char,
) -> Result<(), BFError> {
    match instruction {
        // increment (>) and decrement (>)
        '+' => {
            array[*array_index] += 1;
        }
        '-' => {
            if array[*array_index] > 0 {
                array[*array_index] -= 1;
            } else {
                let x = dbg!(*instruct_index);
                dbg!(&instructions[x..x+10]);
                return Err(BFError::NegativeCellValue)
            }
        }

        // pointer left and right
        '>' => {
            *array_index += 1;

            if *array_index == array.len() {
                array.push(0);
            } // make sure the index is valid
        }
        '<' => {
            if *array_index == 0 {
                return Err(BFError::NegativeArrayPointer);
            };
            *array_index -= 1;
        }

        // loop stuff
        '[' => {
            if array[*array_index] == 0 {
                *instruct_index = equalize_brackets(instructions, *instruct_index, 1)?
            };
        }
        ']' => {
            *instruct_index = equalize_brackets(instructions, *instruct_index, -1)? - 1;
        }

        // input (,) and output (.)
        ',' => {
            let char = input()?;
            if char.is_ascii() {
                array[*array_index] = char as u32
            } else {
                return Err(BFError::NonASCIIChar);
            }
        }
        '.' => {
            if (array[*array_index] as u8).is_ascii() {
                output(array[*array_index] as u8 as char)?
            } else {
                return Err(BFError::NonASCIIChar);
            }
        }
        // 'd' => {dbg!(array, array_index);}
        _ => {}
    }

    Ok(())
}

fn equalize_brackets(string: &str, mut index: usize, direction: isize) -> Result<usize, BFError> {
    let mut depth = 0;

    'find_next_bracket: loop {
        match string.chars().nth(index) {
            Some('[') => {
                depth += 1;
            }
            Some(']') => {
                depth -= 1;
            }

            Some(_) => {}

            None => {
                return Err(BFError::UnbalancedBrackets);
            }
        };
        if depth == 0 {
            break 'find_next_bracket;
        };

        index = match index.checked_add_signed(direction) {
            Some(x) => x,
            None => {
                return Err(BFError::UnbalancedBrackets);
            }
        };
    }
    Ok(index)
}