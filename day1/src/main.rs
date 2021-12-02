/// Solves the Day 1 advent of code challenge for 2021
///
/// https://adventofcode.com/2021/day/1
///
/// Usage: `cat input.txt | cargo run day1`
use std::io::BufRead;

/// Calculates the number of increasing sonar measurements given an array of sonar measurements
fn num_increasing_measurements(measurements: &Vec<u32>) -> usize {
    let mut increases = 0;
    let mut last_measurement = match measurements.get(0) {
        Some(measurement) => measurement,
        // 0-length vec, so no increases possible
        None => return 0,
    };

    // Skip first measurement - it can't be an increase!
    for measurement in measurements.iter().skip(1) {
        if measurement > last_measurement {
            increases += 1;
        }
        last_measurement = measurement;
    }

    increases
}

#[test]
fn test_num_increasing_measurements() {
    assert_eq!(num_increasing_measurements(&vec![1, 2, 3, 4, 5, 6]), 5,);

    // Example provided on website
    assert_eq!(
        num_increasing_measurements(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
        7,
    );
}

/// Tracks increases between sums of 3 measurements
fn num_increasing_measurements_windowed(measurements: &Vec<u32>) -> usize {
    let mut increases = 0;
    let mut last_measurement: Option<u32> = None;

    // Skip the first two since we're working in triples
    for (idx, measurement) in measurements.iter().enumerate().skip(2) {
        let sum = measurement + measurements[idx - 1] + measurements[idx - 2];
        if let Some(last) = last_measurement {
            if sum > last {
                increases += 1;
            }
        }

        last_measurement = Some(sum);
    }

    increases
}

#[test]
fn test_num_increasing_measurements_windowed() {
    assert_eq!(
        num_increasing_measurements_windowed(&vec![1, 2, 3, 4, 5, 6]),
        3,
    );

    // Example provided on website
    assert_eq!(
        num_increasing_measurements_windowed(&vec![
            199, 200, 208, 210, 200, 207, 240, 269, 260, 263
        ]),
        5,
    );
}

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let measurements: Vec<u32> = stdin
        .lock()
        .lines()
        .filter_map(|s| match s {
            Ok(s) => {
                let s = s.trim();
                if s.len() > 0 {
                    Some(s.parse::<u32>().unwrap())
                } else {
                    None
                }
            }
            Err(e) => {
                eprintln!("error: {}", e);
                None
            }
        })
        .collect();

    let num_increases = num_increasing_measurements(&measurements);
    let num_increases_windowed = num_increasing_measurements_windowed(&measurements);
    println!(
        "{} increases; {} increases with 3-measurement windows",
        num_increases, num_increases_windowed
    );

    Ok(())
}
