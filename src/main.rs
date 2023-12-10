#![allow(dead_code, unused_assignments, unused_imports, unused_variables)]
use core::panic;
use std::collections::vec_deque::VecDeque;
use std::collections::HashSet;
use std::env::args;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn read_lines(day: u8) -> impl Iterator<Item = Result<String, impl std::error::Error>> {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = format!("{}/inputs/day_{}", cargo_dir, day);
    let Ok(file) = File::open(&dir) else {
        panic!("File for day {day} not found at {dir}");
    };
    BufReader::new(file).lines().into_iter()
}

fn day_1() {
    let inputs = read_lines(1);
    let lines: Vec<String> = inputs.into_iter().map(|s| s.unwrap()).collect();

    let mut sum: u32 = 0u32;

    for line in lines {
        let mut number: u32 = 0u32;
        let chars: Vec<char> = line.chars().collect();
        let dynstrlen: usize = chars.len();
        for index in 0usize..dynstrlen {
            let safelen: usize = dynstrlen - index;

            if number / 10 == 0 && chars[index].is_numeric() {
                number += match chars[index].to_digit(10) {
                    Some(val) => val,
                    None => 0,
                } * 10;
            }
            if number / 10 == 0 && safelen >= 3 {
                let three = &line.as_str().clone()[index..index + 3];

                number += match three {
                    "one" => 1,
                    "two" => 2,
                    "six" => 6,
                    _ => 0,
                } * 10;
            }
            if number / 10 == 0 && safelen >= 4 {
                let four = &line.as_str().clone()[index..index + 4];

                number += match four {
                    "four" => 4,
                    "five" => 5,
                    "nine" => 9,
                    _ => 0,
                } * 10;
            }
            if number / 10 == 0 && safelen >= 5 {
                let five = &line.as_str().clone()[index..index + 5];

                number += match five {
                    "three" => 3,
                    "seven" => 7,
                    "eight" => 8,
                    _ => 0,
                } * 10;
            }

            if number % 10 == 0 && chars[dynstrlen - index - 1].is_numeric() {
                number += match chars[dynstrlen - index - 1].to_digit(10) {
                    Some(val) => val,
                    None => 0,
                };
            }

            if number % 10 == 0 && safelen >= 3 {
                let three = &line.as_str().clone()[dynstrlen - index - 3..dynstrlen - index];

                number += match three {
                    "one" => 1,
                    "two" => 2,
                    "six" => 6,
                    _ => 0,
                };
            }

            if number % 10 == 0 && safelen >= 4 {
                let four = &line.as_str().clone()[dynstrlen - index - 4..dynstrlen - index];

                number += match four {
                    "four" => 4,
                    "five" => 5,
                    "nine" => 9,
                    _ => 0,
                };
            }

            if number % 10 == 0 && safelen >= 5 {
                let five = &line.as_str().clone()[dynstrlen - index - 5..dynstrlen - index];

                number += match five {
                    "three" => 3,
                    "seven" => 7,
                    "eight" => 8,
                    _ => 0,
                };
            }
        }

        sum += number;
    }

    println!("Result for day 1: {}", sum);
}

fn day_2() {
    println!("Result for day 2: {}", 0);
    let inputs = read_lines(2);
}

fn day_3() {
    println!("Result for day 3: {}", 0);
    let inputs = read_lines(3);
}

fn day_4() {
    println!("Result for day 4: {}", 0);
    let inputs = read_lines(4);
}

fn day_5() {
    println!("Result for day 5: {}", 0);
    let inputs = read_lines(5);
}

fn day_6() {
    println!("Result for day 6: {}", 0);
    let inputs = read_lines(6);
}

fn day_7() {
    println!("Result for day 7: {}", 0);
    let inputs = read_lines(7);
}

fn day_8() {
    println!("Result for day 8: {}", 0);
    let inputs = read_lines(8);
}

fn day_9() {
    println!("Result for day 9: {}", 0);
    let inputs = read_lines(9);
}

fn day_10() {
    println!("Result for day 10: {}", 0);
    let inputs = read_lines(10);
}

fn day_11() {
    println!("Result for day 11: {}", 0);
    let inputs = read_lines(11);
}

fn day_12() {
    println!("Result for day 12: {}", 0);
    let inputs = read_lines(12);
}

fn day_13() {
    println!("Result for day 13: {}", 0);
    let inputs = read_lines(13);
}

fn day_14() {
    println!("Result for day 14: {}", 0);
    let inputs = read_lines(14);
}

fn day_15() {
    println!("Result for day 15: {}", 0);
    let inputs = read_lines(15);
}

fn day_16() {
    println!("Result for day 16: {}", 0);
    let inputs = read_lines(16);
}

fn day_17() {
    println!("Result for day 17: {}", 0);
    let inputs = read_lines(17);
}

fn day_18() {
    println!("Result for day 18: {}", 0);
    let inputs = read_lines(18);
}

fn day_19() {
    println!("Result for day 19: {}", 0);
    let inputs = read_lines(19);
}

fn day_20() {
    println!("Result for day 20: {}", 0);
    let inputs = read_lines(20);
}

fn day_21() {
    println!("Result for day 21: {}", 0);
    let inputs = read_lines(21);
}

fn day_22() {
    println!("Result for day 22: {}", 0);
    let inputs = read_lines(22);
}

fn day_23() {
    println!("Result for day 23: {}", 0);
    let inputs = read_lines(23);
}

fn day_24() {
    println!("Result for day 24: {}", 0);
    let inputs = read_lines(24);
}

fn main() {
    let args: Vec<String> = args().collect();
    match args[1].parse() {
        Ok(day) if (1u8..=24).contains(&day) => {
            println!("Running impl of day {} challenge..", { day });
            match day {
                1 => day_1(),
                2 => day_2(),
                3 => day_3(),
                4 => day_4(),
                5 => day_5(),
                6 => day_6(),
                7 => day_7(),
                8 => day_8(),
                9 => day_9(),
                10 => day_10(),
                11 => day_11(),
                12 => day_12(),
                13 => day_13(),
                14 => day_14(),
                15 => day_15(),
                16 => day_16(),
                17 => day_17(),
                18 => day_18(),
                19 => day_19(),
                20 => day_20(),
                21 => day_21(),
                22 => day_22(),
                23 => day_23(),
                24 => day_24(),
                _ => unreachable!(),
            }
        }
        _ => {
            println!("Provide day argument between 1 and 24");
        }
    }
}

#[test]
fn all_files_are_ok_open_test() {
    for day in 1..=24 {
        // no panicing here
        drop(read_lines(day));
    }
}
