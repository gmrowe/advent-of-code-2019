#![allow(unused_variables)]
#![allow(dead_code)]

use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;


    let start_1 = std::time::Instant::now();
    let answer_1 = part_1(&contents);
    let elapsed_1 = std::time::Instant::now();
    println!("Day 03; Part 1 = {:?}", answer_1);
    println!("elapsed time: {:?}\n", elapsed_1.duration_since(start_1));

    let start_2 = std::time::Instant::now();
    let answer_2 = part_2(&contents);
    let elapsed_2 = std::time::Instant::now();
    println!("Day 03; Part 2 = {}", answer_2);
    println!("elapsed time: {:?}\n", elapsed_2.duration_since(start_2));    
    Ok(())
}


fn part_1(input: &str) -> i32 {
    let wires = parse_wires(input);
    assert!(wires.len() == 2);
    let mut path1 = get_points(&wires[0]);
    let mut path2 = get_points(&wires[1]);
    let intersections = intersections(&mut path1, &mut path2);
    intersections.iter()
        .map(|point| manhattan_distance(point))
        .min()
        .expect("Wires do not cross!")    
}

fn part_2(input: &str) -> i32 {
    let wires = parse_wires(input);
    assert!(wires.len() == 2);
    let path1 = get_points(&wires[0]);
    let path2 = get_points(&wires[1]);
    let intersections = intersections(&mut path1.clone(), &mut path2.clone());
    let steps1 = steps_to(&intersections, &path1);
    let steps2 = steps_to(&intersections, &path2);
    steps1.iter().zip(steps2.iter())
        .map(|(s1, s2)| s1 + s2)
        .min()
        .expect("wires do not cross")
}

type Point = (i32, i32);

#[derive(Debug, PartialEq)]
enum Dir {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Dir {
     fn from_str(s: &str) -> Self {
        let dir = &s[..1];
        let mag = u32::from_str(&s[1..]).expect("Malformed input");
        
        match dir {
            "U" => Self::Up(mag),
            "D" => Self::Down(mag),
            "L" => Self::Left(mag),
            "R" => Self::Right(mag),
            _ => unreachable!(),
        }
    }
}

fn steps_to(targets: &[Point], path: &[Point]) -> Vec<i32> {
    let mut result = vec![0; targets.len()];
    for (i, point) in path.iter().enumerate() {
        if let Some(j) = targets.iter().position(|p| p == point) {
            if result[j] == 0 {
                result[j] = i as i32 + 1;
            }
        }
    }
    result
}

fn parse_wires(input: &str) -> Vec<Vec<Dir>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let dirs = line.split(',')
            .map(Dir::from_str)
            .collect();
        result.push(dirs);
    }
    result    
}

fn trans<F: Fn(Point) -> Point>(init_pos: Point, n: u32, f: F) -> (Vec<Point>, Point) {
    let mut res = Vec::new();
    let mut pos = init_pos;
    let mut i = n;
    while i > 0 {
        pos = f(pos);
        i -= 1;
        res.push(pos);
    }
    (res, pos)
}

fn get_points(dirs: &[Dir]) -> Vec<Point> {
    let mut points = Vec::new();
    let mut pos = (0, 0);
    for d in dirs.iter() {
        let (mut pts, new_pos) = match d {
            Dir::Up(n) => trans(pos, *n, |(x, y)| (x, y + 1)),
            Dir::Down(n) => trans(pos, *n, |(x, y)| (x, y - 1)),
            Dir::Left(n) => trans(pos, *n, |(x, y)| (x - 1, y)),
            Dir::Right(n) => trans(pos, *n, |(x, y)| (x + 1, y)),
        };
        pos = new_pos;
        points.append(&mut pts);
    }
    points
}

fn intersections(path1: &mut [Point], path2: &mut [Point]) -> Vec<Point> {
    let comparator = |p1: &Point, p2: &Point| {
        match p1.0.cmp(&p2.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => p1.1.cmp(&p2.1),
        }
    };
    path1.sort_by(comparator);
    path2.sort_by(comparator);
    let mut path1_iter = path1.iter();
    let mut path2_iter = path2.iter();
    let mut p1 = path1_iter.next();
    let mut p2 = path2_iter.next();
    let mut results = Vec::new();
    while p1.is_some() && p2.is_some() {
        let a = p1.expect("p1 is checked before loop");
        let b = p2.expect("p2 is checked before loop");
        match comparator(a, b) {
            Ordering::Less => p1 = path1_iter.next(),
            Ordering::Greater => p2 = path2_iter.next(),
            Ordering::Equal => {
                results.push(*a);
                p1 = path1_iter.next();
                p2 = path2_iter.next();
            },
        }
    }
    results
}

fn manhattan_distance(point: &Point) -> i32 {
    point.0.abs() + point.1.abs()
}

#[cfg(test)]
mod day_03_tests {
    use super::*;

    fn can_parse_instruction(s: &str, expected: Dir) {
        let dir = Dir::from_str(s);
        assert_eq!(expected, dir);
    }

    #[test]
    fn can_parse_up_instruction() {
        can_parse_instruction("U8", Dir::Up(8));
    }

    #[test]
    fn can_parse_down_instruction() {
        can_parse_instruction("D10", Dir::Down(10));
    }

    #[test]
    fn can_parse_left_instruction() {
        can_parse_instruction("L42", Dir::Left(42));
    }

    #[test]
    fn can_parse_right_instruction() {
        can_parse_instruction("R69", Dir::Right(69));
    }

    #[test]
    fn can_parse_wire_input_format() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let wires = parse_wires(input);
        assert_eq!(vec![vec![Dir::Right(8), Dir::Up(5), Dir::Left(5), Dir::Down(3)],
                        vec![Dir::Up(7), Dir::Right(6), Dir::Down(4), Dir::Left(4)]],
                  wires)
    }

    #[test]
    fn an_empty_list_of_directions_results_in_empty_points() {
        let dirs = vec![];
        let points = get_points(&dirs);
        let empty: Vec<(i32, i32)> = vec![];
        assert_eq!(empty, points);
    }

    #[test]
    fn can_generate_a_list_of_points_from_up_direction() {
        let dirs = vec![Dir::Up(2)];
        let points = get_points(&dirs);
        assert_eq!(vec![(0, 1), (0, 2)], points);

    }

    #[test]
    fn can_generate_a_list_of_points_from_down_direction() {
        let dirs = vec![Dir::Down(2)];
        let points = get_points(&dirs);
        assert_eq!(vec![(0, -1), (0, -2)], points);

    }

    #[test]
    fn can_generate_a_list_of_points_from_left_direction() {
        let dirs = vec![Dir::Left(2)];
        let points = get_points(&dirs);
        assert_eq!(vec![(-1, 0), (-2, 0)], points);

    }

    #[test]
    fn can_generate_a_list_of_points_from_right_direction() {
        let dirs = vec![Dir::Right(2)];
        let points = get_points(&dirs);
        assert_eq!(vec![(1, 0), (2, 0)], points);

    }

    #[test]
    fn can_generate_a_list_of_points_from_slice_of_directions() {
        let dirs = vec![Dir::Right(2), Dir::Up(2)];
        let points = get_points(&dirs);
        assert_eq!(vec![(1, 0), (2, 0), (2, 1), (2, 2)], points);
    }

    #[test]
    fn returns_empty_list_if_no_intersections() {
        let mut path1 = vec![(1, 0), (2, 0)];
        let mut path2 = vec![(0, 1), (0, 2)];
        let intersection_points = intersections(&mut path1, &mut path2);
        assert!(intersection_points.is_empty());
    }

    #[test]
    fn returns_singleton_list_if_one_intersection() {
        let mut path1 = vec![(1, 0), (1, 1), (2, 1)];
        let mut path2 = vec![(0, 1), (1, 1), (1, 2)];
        let intersection_points = intersections(&mut path1, &mut path2);
        assert_eq!(vec![(1, 1)], intersection_points);
    }

    #[test]
    fn ca_calculate_manhattan_distance() {
        let point = (5, -6);
        assert_eq!(11, manhattan_distance(&point));
    }

    #[test]
    fn should_correctly_solve_part_1_sample_input() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let answer = part_1(input);
        assert_eq!(6, answer);
    }

    #[test]
    fn should_correctly_solve_part_2_sample_input() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                     U62,R66,U55,R34,D71,R55,D58,R83";
        let answer = part_2(input);
        assert_eq!(610, answer);
    }
}



