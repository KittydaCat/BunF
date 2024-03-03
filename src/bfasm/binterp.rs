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

#[derive(Debug, Clone, PartialEq)]
pub enum BFOp {
    Plus,
    Minus,
    Left,
    Right,
    Comma,
    Period,
    OpenBracket,
    CloseBracket,
    Lable,
}

impl BFOp {
    pub fn from_str(s: &str) -> Vec<BFOp> {

        let mut program = Vec::new();

        s.chars().for_each(|char| match char {
            '+' => program.push(BFOp::Plus),
            '-' => program.push(BFOp::Minus),
            '<' => program.push(BFOp::Left),
            '>' => program.push(BFOp::Right),
            ',' => program.push(BFOp::Comma),
            '.' => program.push(BFOp::Period),
            '[' => program.push(BFOp::OpenBracket),
            ']' => program.push(BFOp::CloseBracket),
            _ => {}
        });

        program
    }

    pub fn as_str(code: &Vec<BFOp>) -> String {
        code.iter().map(|op|{
            match op {
                BFOp::Plus => '+',
                BFOp::Minus => '-',
                BFOp::Left => '<',
                BFOp::Right => '>',
                BFOp::Comma => ',',
                BFOp::Period => '.',
                BFOp::OpenBracket => '[',
                BFOp::CloseBracket => ']',
                BFOp::Lable => 'L',
            }
        }).collect()
    }
}

#[derive(Debug, Clone)]
pub struct BFInterpreter {
    pub array: Vec<u32>,
    pub array_pointer: usize,

    pub program: Vec<BFOp>,
    pub program_index: usize,

    pub input: Vec<char>,
    pub output: String,
}

impl BFInterpreter {
    pub fn new(
        program: Vec<BFOp>,
    ) -> Self {
        // change to create?

        Self {
            array: vec![0_u32],
            array_pointer: 0,
            program,
            program_index: 0,
            input: Vec::new(),
            output: String::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), BFError> {
        run_bf(
            &mut self.array,
            &mut self.array_pointer,
            &self.program,
            &mut self.input,
            &mut self.output,
            &mut self.program_index,
        )
    }

    pub fn label_run(&mut self) -> Result<(), BFError> {

        if let Some(BFOp::Lable) = self.program.get(self.program_index) {
            self.program_index += 1;
        }

        while let Some(instruction) = self.program.get(self.program_index) {

            if *instruction == BFOp::Lable {
                break
            }

            exec_bf_instruction(
                &mut self.array,
                &mut self.array_pointer,
                &self.program,
                &mut self.input,
                &mut self.output,
                &mut self.program_index,
                instruction,
            )?;

            self.program_index += 1;

        }

        Ok(())
    }

    pub fn exec_one(&mut self) -> Result<(), BFError> {
        if let Some(instruction) = self.program.get(self.program_index) {
            exec_bf_instruction(
                &mut self.array,
                &mut self.array_pointer,
                &self.program,
                &mut self.input,
                &mut self.output,
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
    instructions: &Vec<BFOp>,
    input: &mut Vec<char>,
    output: &mut String,
    instruct_index: &mut usize,
) -> Result<(), BFError> {
    while *array_index >= array.len() {
        array.push(0);
    } // make sure the index is valid

    while let Some(instruction) = instructions.get(*instruct_index) {
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
    instructions: &Vec<BFOp>,
    input: &mut Vec<char>,
    output: &mut String,
    instruct_index: &mut usize,
    instruction: &BFOp,
) -> Result<(), BFError> {
    match instruction {
        // increment (>) and decrement (>)
        BFOp::Plus => {
            array[*array_index] += 1;
        }
        BFOp::Minus => {
            if array[*array_index] > 0 {
                array[*array_index] -= 1;
            } else {
                let x = dbg!(*instruct_index);
                dbg!(&instructions[x..x+10]);
                return Err(BFError::NegativeCellValue)
            }
        }

        // pointer left and right
        BFOp::Right => {
            *array_index += 1;

            if *array_index == array.len() {
                array.push(0);
            } // make sure the index is valid
        }
        BFOp::Left => {
            if *array_index == 0 {
                return Err(BFError::NegativeArrayPointer);
            };
            *array_index -= 1;
        }

        // loop stuff
        BFOp::OpenBracket => {
            if array[*array_index] == 0 {
                *instruct_index = equalize_brackets(instructions, *instruct_index, 1)?
            };
        }
        BFOp::CloseBracket => {
            *instruct_index = equalize_brackets(instructions, *instruct_index, -1)? - 1;
        }

        // input (,) and output (.)
        BFOp::Comma => {
            if input.len() == 0 {
                return Err(BFError::InputFailed);
            }
            let char = input.remove(0);
            if char.is_ascii() {
                array[*array_index] = char as u32
            } else {
                return Err(BFError::NonASCIIChar);
            }
        }
        BFOp::Period => {
            if (array[*array_index] as u8).is_ascii() {
                output.push(array[*array_index] as u8 as char)
            } else {
                return Err(BFError::NonASCIIChar);
            }
        }
        BFOp::Lable => {}
    }

    Ok(())
}

fn equalize_brackets(program: &[BFOp], mut index: usize, direction: isize) -> Result<usize, BFError> {
    let mut depth = 0;

    'find_next_bracket: loop {
        match program.get(index) {
            Some(BFOp::OpenBracket) => {
                depth += 1;
            }
            Some(BFOp::CloseBracket) => {
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