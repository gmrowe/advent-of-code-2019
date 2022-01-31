#![allow(unused_variables)]
#![allow(dead_code)]

// use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;


    // let start_1 = std::time::Instant::now();
    // part_1(&contents.trim());
    // let elapsed_1 = std::time::Instant::now();
    // //println!("Day 03; Part 1 = {:?}", answer_1);
    // println!("elapsed time: {:?}\n", elapsed_1.duration_since(start_1));

    let start_2 = std::time::Instant::now();
    part_2(&contents.trim());
    let elapsed_2 = std::time::Instant::now();
    //println!("Day 03; Part 2 = {}", answer_2);
    println!("elapsed time: {:?}\n", elapsed_2.duration_since(start_2));

    
    Ok(())
}

fn part_1(s: &str) {
    let parse_fail_msg = |n| {
        format!("Could not parse >{}<", n)
    };
    let input = s.split(",")
        .map(|n| isize::from_str(n).expect(&parse_fail_msg(n)))
        .collect::<Vec<isize>>();
    execute(input, 1);
}

fn part_2(s: &str) {
    let parse_fail_msg = |n| {
        format!("Could not parse >{}<", n)
    };
    let input = s.split(",")
        .map(|n| isize::from_str(n).expect(&parse_fail_msg(n)))
        .collect::<Vec<isize>>();
    execute(input, 5);
}

#[derive(PartialEq)]
enum Cont {
    Halt,
    Continue,
}

fn bin_op<F>(
    mut program: Vec<isize>,
    op_idx: isize,
    param_modes: Vec<isize>,
    op: F
) -> Vec<isize>
where
    F: Fn((isize,isize)) -> isize
{
    let u_op_idx = op_idx as usize;
    let (op1, op2, dest) = (program[u_op_idx + 1], program[u_op_idx + 2], program[u_op_idx + 3]);
    let mut mode_iter = param_modes.iter();
    let operand1 = match mode_iter.next().unwrap_or(&0) {
        0 => program[op1 as usize],
        1 => op1,
        _ => unreachable!(),
    };
    let operand2 = match mode_iter.next().unwrap_or(&0) {
        0 => program[op2 as usize],
        1 => op2,
        _ => unreachable!(),
    };
    program[dest as usize] = op((operand1, operand2));
    program
}

fn jmp_op<F>(
    program: &[isize],
    op_idx: isize,
    param_modes: Vec<isize>,
    test: F
) -> isize
where
    F: Fn(isize) -> bool
{
    const JMPOP_CHUMK_SIZE: isize = 3;
    let mut mode_iter = param_modes.iter();
    let jmp_param_idx = match mode_iter.next().unwrap_or(&0) {
        0 => program[op_idx as usize + 1] as usize,
        1 => op_idx as usize + 1,
        _ => unreachable!(),
    };
    if test(program[jmp_param_idx]) {
        let jump_index = match mode_iter.next().unwrap_or(&0) {
            0 => program[op_idx as usize + 2] as usize,
            1 => op_idx as usize + 2,
            _ => unreachable!(),
        };
        program[jump_index] - op_idx
    } else {
        JMPOP_CHUMK_SIZE
    }  
}

fn cmp_op<F>(
    mut program: Vec<isize>,
    op_idx: isize,
    param_modes: Vec<isize>,
    test: F
) -> Vec<isize>
where
    F: Fn((isize, isize)) -> bool
{
    let u_op_idx = op_idx as usize;
    let mut mode_iter = param_modes.iter();
    let (op1, op2, dest) =
        (program[u_op_idx + 1], program[u_op_idx + 2], program[u_op_idx + 3]);
    
    let operand1 = match mode_iter.next().unwrap_or(&0) {
        0 => program[op1 as usize],
        1 => op1,
        _ => unreachable!(),
    };

    let operand2 = match mode_iter.next().unwrap_or(&0) {
        0 => program[op2 as usize],
        1 => op2,
        _ => unreachable!(),
    };

    let value = if test((operand1, operand2)) { 1 } else { 0 };
    program[dest as usize] = value;
    program
}

fn get_op_values(opval: isize) -> (isize, Vec<isize>) {
    const OPCODE_FACTOR: isize = 100;
    const PARAM_MODE_FACTOR: isize = 10;
    let op_code = opval % OPCODE_FACTOR;
    let mut param_modes = Vec::new();
    let mut params = opval / OPCODE_FACTOR;
    while params > 0 {
        param_modes.push(params % PARAM_MODE_FACTOR);
        params /= PARAM_MODE_FACTOR;
    }
    (op_code, param_modes)
}
    
fn execute_from(
    mut program: Vec<isize>,
    op_idx: isize,
    user_input: isize
) -> (Vec<isize>, Cont, isize) {
    const BINOP_CHUNK_SIZE: isize = 4;
    const IOOP_CHUNK_SIZE: isize = 2;
    const JMPOP_CHUMK_SIZE: isize = 3;
    let (opcode, param_modes) = get_op_values(program[op_idx as usize]);
    let mut continuation = Cont::Continue;
    let mut chunk_size = 1;
    let u_op_idx = op_idx as usize;
    match opcode {
        1 => {
            program = bin_op(program, op_idx, param_modes, |(m, n)| m + n);
            chunk_size = BINOP_CHUNK_SIZE;
        },
        
        2 => {
            program = bin_op(program, op_idx, param_modes, |(m, n)| m * n);
            chunk_size = BINOP_CHUNK_SIZE;
        },
        
        3 => {
            let dest = program[op_idx as usize + 1];
            program[dest as usize] = user_input;
            chunk_size = IOOP_CHUNK_SIZE;
        },

        4 => {
            let param_mode = param_modes.get(0).unwrap_or(&0);
            let output_val_idx = match param_mode {
                0 => program[op_idx as usize + 1] as usize,
                1 => op_idx as usize + 1,
                _ => unreachable!(),
            };
            let output_val = program[output_val_idx];
            println!("INTCODE OUTPUT: {}", output_val);
            chunk_size = IOOP_CHUNK_SIZE;
        },

        5 => chunk_size = jmp_op(&program, op_idx, param_modes, |n| n != 0),

        6 => chunk_size = jmp_op(&program, op_idx, param_modes, |n| n == 0),

        7 =>  {
            program = cmp_op(program, op_idx, param_modes, |(op1, op2)| op1 < op2);
            chunk_size = BINOP_CHUNK_SIZE;
        },

        8 =>  {
            program = cmp_op(program, op_idx, param_modes, |(op1, op2)| op1 == op2);            
            chunk_size = BINOP_CHUNK_SIZE;
        },

        99 => continuation = Cont::Halt,
        
        _ => unreachable!(&format!("Found opcode: {}", opcode)),
    }
    (program, continuation, chunk_size)
}

fn execute(program: Vec<isize>, user_input: isize) -> Vec<isize> {
    let mut cont = Cont::Continue;
    let mut output = program;
    let mut op_idx = 0;
    while cont != Cont::Halt {
        let (o, c, chunk_size) = execute_from(output, op_idx, user_input);
        output = o;
        cont = c;
        op_idx += chunk_size;
    }
    output
}

#[cfg(test)]
mod day_05_tests {
    use super::*;

    #[test]
    fn executes_an_add_instruction() {
        let program = vec![1, 0, 0, 0, 99];
        let output = execute(program, 0);
        assert_eq!(vec![2, 0, 0, 0, 99], output);
    }

    #[test]
    fn executes_multiply_instructions() {
        let program = vec![2,3,0,3,99];
        let output = execute(program, 0);
        assert_eq!(vec![2, 3, 0, 6, 99], output);
    }

    #[test]
    fn can_store_a_number_in_last_pos() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let output = execute(program, 0);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], output);
    }

    #[test]
    fn halts_immeditately_on_opcode_99() {
        let program = vec![99, 0, 0, 0, 99];
        let output = execute(program, 0);
        assert_eq!(vec![99, 0, 0, 0, 99], output);
        
    }

    #[test]
    fn can_execute_two_sequential_operations() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = execute(program, 0);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], output);
    }

    #[test]
    fn can_solve_the_example_problem() {
        let program = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let output = execute(program, 0);
        assert_eq!(vec![3500,9,10,70,2,3,11,0,99,30,40,50], output);
    }
    
    #[test]
    fn should_execute_multiply_command_based_on_new_parameter_modes() {
        let program = vec![1002, 4, 3, 4, 33];
        let output = execute(program, 0);
        assert_eq!(vec![1002, 4, 3, 4, 99], output);
    }

    #[test]
    fn can_handle_programs_with_negatove_values() {
        let program = vec![1101,100,-1,4,0];
        let output = execute(program, 0);
        assert_eq!(vec![1101, 100, -1, 4, 99], output);
    }
}
