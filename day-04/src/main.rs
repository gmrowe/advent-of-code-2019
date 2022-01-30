#![allow(unused_variables)]
#![allow(dead_code)]

use std::str::FromStr;

fn main() {
    let input = "357253-892942";

    let start_1 = std::time::Instant::now();
    let answer_1 = part_1(&input);
    let elapsed_1 = std::time::Instant::now();
    println!("Day 04; Part 1 = {}", answer_1);
    println!("elapsed time: {:?}\n", elapsed_1.duration_since(start_1));

    let start_2 = std::time::Instant::now();
    let answer_2 = part_2(&input);
    let elapsed_2 = std::time::Instant::now();
    println!("Day 04; Part 2 = {}", answer_2);
    println!("elapsed time: {:?}\n", elapsed_2.duration_since(start_2));

}

fn part_1(s: &str) -> usize {
    let (start, end) = parse_range(s);
    assert!(end > start);
    (start..=end)
        .map(digits)
        .filter(|ds| has_adjacent_matching_digits(ds))
        .filter(|ds| is_non_decreasing(ds))
        .count()
}

fn part_2(s: &str) -> usize {
    let (start, end) = parse_range(s);
    assert!(end > start);
    (start..=end)
        .map(digits)
        .filter(|ds| is_non_decreasing(ds))
        .map(|ds| run_length_encode(&ds))
        .filter(|rle| rle.iter().any(|(_, count)| *count == 2))
        .count()
}


fn parse_range(s: &str) -> (u32, u32) {
    let range = s.split("-")
        .map(|n| u32::from_str(n).expect("Malformed int input"))
        .collect::<Vec<u32>>();
    (range[0], range[1])
}

fn digits(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| u32::from_str(&c.to_string()).expect("Found non int char"))
        .collect()
}

fn run_length_encode(ns: &[u32]) -> Vec<(u32, u32)> {
    let mut run = 1;
    let mut result = Vec::new();
    for slice in ns.windows(2) {
        if slice[0] == slice[1] {
            run += 1;
        } else {
            result.push((slice[0], run));
            run = 1;
        }
    }
    result.push((*ns.last().expect("Digits are empty"), run));
    result
}

fn has_adjacent_matching_digits(ns: &[u32]) -> bool {
    ns.windows(2).any(|slice| slice[0] == slice[1])
}

fn is_non_decreasing(ns: &[u32]) -> bool {
    ns.windows(2).all(|slice| slice[0] <= slice[1])
}

#[cfg(test)]
mod day_04_tests {
    use super::*;

    #[test]
    fn parse_range_parses_a_dash_separated_range() {
        let range = "123-456";
        assert_eq!((123, 456), parse_range(range));
    }

    #[test]
    fn has_adjacent_matching_digits_returns_false_if_n_has_none() {
        let n = 123;
        assert!(!has_adjacent_matching_digits(&digits(n)));
    }

    #[test]
    fn has_adjacent_digits_returns_true_if_n_has_some() {
        let n = 1223;
        assert!(has_adjacent_matching_digits(&digits(n)));
    }

    #[test]
    fn is_non_decreasing_returns_false_if_2_decreasing_elements() {
        let n = 12324;
        assert!(!is_non_decreasing(&digits(n)));
    }

    #[test]
    fn is_non_decreasing_returns_true_if_no_2_decreasing_elements() {
        let n = 12334;
        assert!(is_non_decreasing(&digits(n)));
    }

    #[test]
    fn can_run_length_encode_a_number() {
        let n = 12233345;
        assert_eq!(
            vec![(1, 1), (2, 2), (3, 3), (4, 1), (5, 1)],
            run_length_encode(&digits(n))
        );
    }
}
