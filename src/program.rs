use crate::program::bfstd::*;
mod bfstd;

pub(crate) fn main() {
    let program = input_str();

    let mut program_index = 0;

    let mut array = new_array();

    array.push(0);

    let mut array_index = 0;

    while program_index < program.len() {
        match program.chars().nth(program_index).unwrap() {
            '>' => {
                array_index += 1;
                if array_index == array.len() {
                    array.push(0);
                }
            }
            '<' => {
                array_index -= 1;
            }
            '+' => {
                array[array_index] += 1;
            }
            '-' => {
                array[array_index] -= 1;
            }
            ',' => {
                array[array_index] = input_u32();
            }
            '.' => {
                print_u32(array[array_index]);
            }
            '[' => {
                if array[array_index] == 0 {
                    let mut bracket_count = 1;

                    program_index += 1;

                    while bracket_count > 0 {
                        match program.chars().nth(program_index).unwrap() {
                            '[' => {
                                bracket_count += 1;
                            }
                            ']' => {
                                bracket_count -= 1;
                            }
                            _ => {}
                        }

                        program_index += 1;
                    }
                }
            }
            ']' => {
                if array[array_index] > 1 {
                    let mut bracket_count = 1;

                    program_index -= 1;

                    while bracket_count > 0 {
                        match program.chars().nth(program_index).unwrap() {
                            '[' => {
                                bracket_count -= 1;
                            }
                            ']' => {
                                bracket_count += 1;
                            }
                            _ => {}
                        }
                        program_index -= 1;
                    }
                }
            }
            _ => {}
        }

        program_index += 1;
    }
}
