use std::io::Read;
use std::fs::File;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let input_data = contents.trim()
        .split(',')
        .map(|n| usize::from_str(n).expect("Failed to parse string"))
        .collect::<Vec<usize>>();

    let answer_1 = part_1(&input_data);
    println!("Day 02; Part 1 = {:?}", answer_1);
    
    let answer_2 = part_2(&input_data).expect("Did not find solution");
    println!("Day 02; Part 2 = {}", answer_2);
    Ok(())
}

#[derive(PartialEq)]
enum Cont {
    Halt,
    Continue,
}

fn part_1(data: &[usize]) -> usize {
    const POS_1: usize = 12;
    const POS_2: usize = 2;
    let mut v = data.to_vec();
    v[1] = POS_1;
    v[2] = POS_2;
    let result = execute(v);
    result[0]
}

fn part_2(data: &[usize]) -> Option<usize> {
    const TARGET: usize = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut v = data.to_vec();
            v[1] = noun;
            v[2] = verb;
            let result = execute(v);
            if result[0] == TARGET {
                return Some(100 * noun + verb);
            }
        }
    }
    None
}

fn bin_op<F>(mut program: Vec<usize>, op_idx: usize, op: F) -> Vec<usize>
where
    F: Fn((usize,usize)) -> usize
{
    let (op1, op2, dest) = (program[op_idx + 1], program[op_idx + 2], program[op_idx + 3]);
    program[dest] = op((program[op1], program[op2]));
    program
}
    
fn execute_from(mut program: Vec<usize>, op_idx: usize) -> (Vec<usize>, Cont) {
    let opcode = program[op_idx];
    let mut continuation = Cont::Continue;
    match opcode {
        1 => program = bin_op(program, op_idx, |(m, n)| m + n),
        2 => program = bin_op(program, op_idx, |(m, n)| m * n),
        99 => continuation = Cont::Halt,
        _ => unreachable!(),
    }
    (program, continuation)
}

fn execute(program: Vec<usize>) -> Vec<usize> {
    const CHUNK_SIZE: usize = 4;
    let mut cont = Cont::Continue;
    let mut output = program;
    let mut op_idx = 0;
    while cont != Cont::Halt {
        let (o, c) = execute_from(output, op_idx);
        output = o;
        cont = c;
        op_idx += CHUNK_SIZE;
    }
    output
}

#[cfg(test)]
mod day_02_tests {
    use super::*;

    #[test]
    fn executes_an_add_instruction() {
        let program = vec![1, 0, 0, 0, 99];
        let output = execute(program);
        assert_eq!(vec![2, 0, 0, 0, 99], output);
    }

    #[test]
    fn executes_multiply_instructions() {
        let program = vec![2,3,0,3,99];
        let output = execute(program);
        assert_eq!(vec![2, 3, 0, 6, 99], output);
    }

    #[test]
    fn can_store_a_number_in_last_pos() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let output = execute(program);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], output);
    }

    #[test]
    fn halts_immeditately_on_opcode_99() {
        let program = vec![99, 0, 0, 0, 99];
        let output = execute(program);
        assert_eq!(vec![99, 0, 0, 0, 99], output);
        
    }

    #[test]
    fn can_execute_two_sequential_operations() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = execute(program);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], output);
    }

    #[test]
    fn can_solve_the_example_problem() {
        let program = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let output = execute(program);
        assert_eq!(vec![3500,9,10,70,2,3,11,0,99,30,40,50], output);
    }

}
