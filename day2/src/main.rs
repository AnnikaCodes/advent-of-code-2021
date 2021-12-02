/// Solves the Day 2 advent of code challenge for 2021
///
/// https://adventofcode.com/2021/day/2
///
/// Usage: `cat input.txt | cargo run`
use std::io::BufRead;

struct Coordinates {
    depth: i32,
    horiz: i32,
    aim: i32,
}

/// Parses instructions for Part 1
fn parse_part1(instructions: &Vec<String>) -> Coordinates {
    let mut coords = Coordinates {
        depth: 0,
        horiz: 0,
        aim: 0,
    };

    for instruction in instructions {
        let mut split = instruction.split(' ');
        let i = split.next();
        let number = split.next().unwrap().to_owned().parse::<i32>().unwrap();

        match i {
            Some("forward") => coords.horiz += number as i32,
            Some("down") => coords.depth += number as i32,
            Some("up") => coords.depth -= number as i32,
            _ => panic!("Unknown instruction '{}'", instruction),
        }
    }

    coords
}

#[test]
fn test_part1() {
    let instructions = vec![
        "forward 5".to_owned(),
        "down 5".to_owned(),
        "forward 8".to_owned(),
        "up 3".to_owned(),
        "down 8".to_owned(),
        "forward 2".to_owned(),
    ];

    let coords = parse_part1(&instructions);

    assert_eq!(coords.horiz, 15);
    assert_eq!(coords.depth, 10);
    // Aim not used in part 1
    assert_eq!(coords.aim, 0);
}

/// Parses instructions for Part 1
fn parse_part2(instructions: &Vec<String>) -> Coordinates {
    let mut coords = Coordinates {
        depth: 0,
        horiz: 0,
        aim: 0,
    };

    for instruction in instructions {
        let mut split = instruction.split(' ');
        let i = split.next();
        let number = split.next().unwrap().to_owned().parse::<i32>().unwrap();

        match i {
            Some("forward") => {
                coords.horiz += number as i32;
                coords.depth += coords.aim * number as i32;
            }
            Some("down") => coords.aim += number as i32,
            Some("up") => coords.aim -= number as i32,
            _ => panic!("Unknown instruction '{}'", instruction),
        }
    }

    coords
}

#[test]
fn test_part2() {
    let instructions = vec![
        "forward 5".to_owned(),
        "down 5".to_owned(),
        "forward 8".to_owned(),
        "up 3".to_owned(),
        "down 8".to_owned(),
        "forward 2".to_owned(),
    ];

    let coords = parse_part2(&instructions);

    assert_eq!(coords.horiz, 15);
    assert_eq!(coords.depth, 60);
    assert_eq!(coords.aim, 10);
}

fn main() {
    let stdin = std::io::stdin();
    let instructions: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(|s| match s {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("error: {}", e);
                None
            }
        })
        .collect();

    for (name, coords) in [
        ("Part 1", parse_part1(&instructions)),
        ("Part 2", parse_part2(&instructions)),
    ] {
        println!(
            "{}: depth {} and horizontal position {} for a puzzle answer of {}",
            name,
            coords.horiz,
            coords.depth,
            coords.depth * coords.horiz
        );
    }
}
