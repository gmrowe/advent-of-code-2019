use std::io::Read;
use std::fs::File;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let input_data = contents.lines()
        .map(|line| i32::from_str(line).expect("Failed to parse string"))
        .collect::<Vec<i32>>();

    let answer_1 = part_1(&input_data);
    let answer_2 = part_2(&input_data);

    println!("Day 01; Part 1 = {}", answer_1);
    println!("Day 01; Part 2 = {}", answer_2);
    Ok(())
}

fn part_1(input_data: &[i32]) -> i32 {
    input_data.iter()
        .map(|n| (n / 3) - 2)
        .sum()
}

fn part_2(input_data: &[i32]) -> i32 {
    input_data.iter()
        .map(|&n| {
            let mut total = 0;
            let mut current = n;
            while current > 0 {
                current = i32::max(0, (current / 3) - 2);
                total += current;
            }
            total
        })
        .sum()
}
