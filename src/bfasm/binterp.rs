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
    Comment(char),
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
            x => program.push(BFOp::Comment(char)),
        });

        program
    }

    pub fn as_str(code: &[BFOp]) -> String {
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
                BFOp::Comment(x) => *x,
            }
        }).collect()
    }
}

#[derive(Debug, Clone)]
pub struct BFInterpreter {
    pub array: Vec<u32>,
    pub array_index: usize,

    pub instructions: Vec<BFOp>,
    pub instruction_index: usize,

    pub input: String,
    pub input_index: usize,
    pub output: String,
}

impl Default for BFInterpreter {
    fn default() -> Self {
        BFInterpreter::new(Vec::new(), String::new())
    }
}

impl BFInterpreter {
    pub fn new(
        instructions: Vec<BFOp>,
        input: String
    ) -> Self {
        // change to create?

        Self {
            array: vec![0_u32],
            array_index: 0,
            instructions,
            instruction_index: 0,
            input_index: 0,
            input,
            output: String::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), BFError> {
        while self.instruction_index < self.instructions.len() {
            self.exec_one()?;
        }

        Ok(())
    }

    pub fn label_run(&mut self) -> Result<(), BFError> {

        if let Some(BFOp::Lable) = self.instructions.get(self.instruction_index) {
            self.instruction_index += 1;
        }

        while let Some(instruction) = self.instructions.get(self.instruction_index) {

            if *instruction == BFOp::Lable {
                break
            }

            self.exec_one()?;
        }

        Ok(())
    }

    pub fn exec_one(&mut self) -> Result<(), BFError> {

        let Some(instruction) = self.instructions.get(self.instruction_index)
            else {return Err(BFError::InvalidInstructionIndex)};

        match instruction {
            // increment (>) and decrement (>)
            BFOp::Plus => {
                self.array[self.array_index] += 1;
            }
            BFOp::Minus => {
                if self.array[self.array_index] > 0 {
                    self.array[self.array_index] -= 1;
                } else {
                    let x = dbg!(self.instruction_index);
                    dbg!(&self.instructions[x..x + 10]);
                    return Err(BFError::NegativeCellValue)
                }
            }

            // pointer left and right
            BFOp::Right => {
                self.array_index += 1;

                if self.array_index == self.array.len() {
                    self.array.push(0);
                } // make sure the index is valid
            }
            BFOp::Left => {
                if self.array_index == 0 {
                    return Err(BFError::NegativeArrayPointer);
                };
                self.array_index -= 1;
            }

            // loop stuff
            BFOp::OpenBracket => {
                if self.array[self.array_index] == 0 {
                    self.instruction_index = equalize_brackets(&self.instructions, self.instruction_index, 1)?
                };
            }
            BFOp::CloseBracket => {
                self.instruction_index = equalize_brackets(&self.instructions, self.instruction_index, -1)? - 1;
            }

            // input (,) and output (.)
            BFOp::Comma => {
                // if self.input.is_empty() {
                //     return Err(BFError::InputFailed);
                // }
                let char = self.input.chars().nth(self.input_index).ok_or(BFError::InputFailed)?;
                if char.is_ascii() {
                    self.array[self.array_index] = char as u32
                } else {
                    return Err(BFError::NonASCIIChar);
                }

                self.input_index += 1;
            }
            BFOp::Period => {
                if (self.array[self.array_index] as u8).is_ascii() {
                    self.output.push(self.array[self.array_index] as u8 as char)
                } else {
                    return Err(BFError::NonASCIIChar);
                }
            }
            BFOp::Lable | BFOp::Comment(_) => {}
        }

        self.instruction_index += 1;

        Ok(())
    }

    // pub fn reset(&mut self) ->(){
    //     self.array = vec![0_u32];
    //     self.array_pointer = 0;
    //     self.program_index = 0;
    // }
}

fn equalize_brackets(program: &[BFOp], mut index: usize, direction: isize) -> Result<usize, BFError> {
    let mut depth = 0;

    loop {
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
            break;
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