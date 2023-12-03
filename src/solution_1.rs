use std::env;
use super::get_input::get_input;

pub fn part_one() {
    // env::set_var("sample", "true");
    let input = get_input(1).unwrap();
    let lines = input.lines();
    let solution = lines.map(|line| {
        let digits: Vec<&str> = line
            .split("")
            .filter(|s| {
                s.parse::<u8>().is_ok()
            })
            .collect::<Vec<&str>>();
        format!
            ("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<u32>()
            .unwrap()
    }).sum::<u32>();
    if env::var("should_print").is_ok() {
        println!("SOLUTION: {}", solution);
    }
    // assert_eq!(solution, 53974);
}

pub fn part_two() {
    // env::set_var("sample", "true");
    let input = get_input(1).unwrap();
    let lines = input.lines();
    let digit_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut solution: u32 = 0;
    for line in lines {
        let mut left_val = 0;
        let mut right_val = 0;
        let mut slce = &line[..];
        while left_val == 0 {
            if slce.starts_with(|x: char| x.is_digit(10)) {
                left_val = slce.chars().next().unwrap().to_string().parse::<u32>().unwrap();
                break;
            }
            for (u, &digit_str) in digit_strings.iter().enumerate() {
                if slce.starts_with(digit_str) {
                    left_val = u as u32 + 1;
                    break;
                }
            }
            slce = &slce[1..];
        }
        let mut slice_ind = slce.len();
        while right_val == 0 {
            slce = &slce[..slice_ind];
            slice_ind -= 1;
            if slce.ends_with(|x: char| x.is_digit(10)) {
                right_val = slce.chars().last().unwrap().to_string().parse::<u32>().unwrap();
                break;
            }
            for (u, &digit_str) in digit_strings.iter().enumerate() {
                if slce.ends_with(digit_str) {
                    right_val = u as u32 + 1;
                    break;
                }
            }
        }
        solution += (left_val * 10) + right_val;
    }
    if env::var("should_print").is_ok() {
        println!("SOLUTION :: {}", solution);
    }
    // assert_eq!(solution, 52840);
}
