#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;


    let start_1 = std::time::Instant::now();
    let output_1 = part_1(&contents.trim());
    let elapsed_1 = std::time::Instant::now();
    println!("Day 05; Part 1 = {}", output_1);
    println!("elapsed time: {:?}\n", elapsed_1.duration_since(start_1));

    let start_2 = std::time::Instant::now();
    let output_2 = part_2(&contents.trim());
    let elapsed_2 = std::time::Instant::now();
    println!("Day 05; Part 2 = {}", output_2);
    println!("elapsed time: {:?}\n", elapsed_2.duration_since(start_2));

    Ok(())
}

fn io_execution(comp: &mut IComp, input: isize) -> String {
    comp.execute(input);
    comp.output.join("\n")
}

fn part_1(s: &str) -> String {
    let parse_fail_msg = |n| {
        format!("Could not parse >{}<", n)
    };
    let input = s.split(",")
        .map(|n| isize::from_str(n).expect(&parse_fail_msg(n)))
        .collect::<Vec<isize>>();
    let mut comp = IComp::from_program(input);
    comp.execute(1);
    comp.output.join("\n")
}

fn part_2(s: &str) -> String {
    let parse_fail_msg = |n| {
        format!("Could not parse >{}<", n)
    };
    let input = s.split(",")
        .map(|n| isize::from_str(n).expect(&parse_fail_msg(n)))
        .collect::<Vec<isize>>();

    let mut comp = IComp::from_program(input);
    comp.execute(5);
    comp.output.join("\n")
}

#[derive(PartialEq, Debug)]
enum Cont {
    Halt,
    Continue,
}

#[derive(Debug)]
struct IComp {
    program: Vec<isize>,
    iptr: usize,
    output: Vec<String>,
    exec_state: Cont,
}

impl IComp {
    fn from_program(program: Vec<isize>) -> Self {
        Self {
            program,
            iptr: 0,
            output: Vec::new(),
            exec_state: Cont::Continue,
        }
    }

    fn get_op_values(&self) -> (isize, Vec<usize>) {
        const OPCODE_FACTOR: isize = 100;
        const PARAM_MODE_FACTOR: isize = 10;
        let opval = self.program[self.iptr];
        let op_code = opval % OPCODE_FACTOR;
        let mut param_modes = Vec::new();
        let mut params = opval / OPCODE_FACTOR;
        while params > 0 {
            param_modes.push((params % PARAM_MODE_FACTOR) as usize);
            params /= PARAM_MODE_FACTOR;
        }
        (op_code, param_modes)
    }

    fn param_val(&self, param: isize, param_mode: Option<&usize>) -> isize {
        match param_mode.unwrap_or(&0) {
            0 => self.program[param as usize],
            1 => param,
            n => unreachable!(format!("Unknown param mode: {}", n)),
        }
    }

    fn bin_op<F>(&mut self, param_modes: Vec<usize>, op: F)
    where
        F: Fn((isize,isize)) -> isize
    {
        const BINOP_CHUNK_SIZE: usize = 4;
        let param1 = self.program[self.iptr + 1];
        let param2 = self.program[self.iptr + 2];
        let param3 = self.program[self.iptr + 3];    
        let operand1 = self.param_val(param1, param_modes.get(0));
        let operand2 = self.param_val(param2, param_modes.get(1));
        let dest = param3 as usize;
        self.program[dest] = op((operand1, operand2));
        self.iptr += BINOP_CHUNK_SIZE;
    }


    fn jmp_op<F>(&mut self, param_modes: Vec<usize>, test: F)
    where
        F: Fn(isize) -> bool
    {
        const JMPOP_CHUNK_SIZE: usize = 3;
        let param1 = self.program[self.iptr + 1];
        let param2 = self.program[self.iptr + 2];
        let test_val = self.param_val(param1, param_modes.get(0));
        let jump_dest = self.param_val(param2, param_modes.get(1));
 
        if test(test_val) {
            self.iptr = jump_dest as usize;
        } else {
            self.iptr += JMPOP_CHUNK_SIZE;
        }  
    }

    fn input(&mut self, user_input: isize) {
        const IOOP_CHUNK_SIZE: usize = 2;
        let param = self.program[self.iptr + 1];
        self.program[param as usize] = user_input;
        self.iptr += IOOP_CHUNK_SIZE;
    }

    fn output(&mut self, param_modes: Vec<usize>) {
        const IOOP_CHUNK_SIZE: usize = 2;
        let param = self.program[self.iptr + 1];
        let out_val = self.param_val(param, param_modes.get(0));
        self.output.push(out_val.to_string());
        self.iptr += IOOP_CHUNK_SIZE;
    }

    fn execute(&mut self, user_input: isize) {
        const IOOP_CHUNK_SIZE: usize = 2;
        while self.exec_state != Cont::Halt {
            let (opcode, param_modes) = self.get_op_values();
            match opcode {
                1 => self.bin_op(param_modes, |(m, n)| m + n),
                2 => self.bin_op(param_modes, |(m, n)| m * n),
                3 => self.input(user_input),
                4 => self.output(param_modes),
                5 => self.jmp_op(param_modes, |n| n != 0),
                6 => self.jmp_op(param_modes, |n| n == 0),
                7 => self.bin_op(param_modes, |(m, n)| (m < n) as isize),
                8 => self.bin_op(param_modes, |(m, n)| (m == n) as isize),
                99 => self.exec_state = Cont::Halt,       
                _ => unreachable!(&format!("Unknown opcode: {}", opcode)),
            }

        }
    }
}

#[cfg(test)]
mod day_05_tests {
    use super::*;

    fn assert_final_program_state_input_0(init: Vec<isize>, expected: Vec<isize>) {
        let mut computer = IComp::from_program(init);
        computer.execute(0);
        assert_eq!(expected, computer.program);
    }

    #[test]
    fn executes_an_add_instruction() {
        assert_final_program_state_input_0(
            vec![1, 0, 0, 0, 99],
            vec![2, 0, 0, 0, 99]
        );
    }

    #[test]
    fn executes_multiply_instructions() {
        assert_final_program_state_input_0(
            vec![2,3,0,3,99],
            vec![2, 3, 0, 6, 99]
        );
    }

    #[test]
    fn can_store_a_number_in_last_pos() {
        assert_final_program_state_input_0(
            vec![2, 4, 4, 5, 99, 0],
            vec![2, 4, 4, 5, 99, 9801]
        );        
    }

    #[test]
    fn halts_immeditately_on_opcode_99() {
        assert_final_program_state_input_0(
            vec![99, 0, 0, 0, 99],
            vec![99, 0, 0, 0, 99]
        );
    }

    #[test]
    fn can_execute_two_sequential_operations() {
        assert_final_program_state_input_0(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn can_solve_the_example_problem() {
        assert_final_program_state_input_0(
            vec![1,9,10,3,2,3,11,0,99,30,40,50],
            vec![3500,9,10,70,2,3,11,0,99,30,40,50]
        );
    }
    
    #[test]
    fn should_execute_multiply_command_based_on_new_parameter_modes() {
        assert_final_program_state_input_0(
            vec![1002, 4, 3, 4, 33],
            vec![1002, 4, 3, 4, 99]
        );
    }

    #[test]
    fn can_handle_programs_with_negative_values() {
        assert_final_program_state_input_0(
            vec![1101,100,-1,4,0],
            vec![1101, 100, -1, 4, 99]
        );
    }
}
