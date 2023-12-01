use std::sync::Arc;

use crate::Run;

pub struct Day1Part2 {}

struct Numbers<'a> {
    key: &'a str,
    value: u32,
}

impl<'a> Numbers<'a> {
    fn new(key: &'a str, value: u32) -> Self {
        Self { key, value }
    }
}

impl Run for Day1Part2 {
    fn run() {
        // let input = r"two1nine
        // eightwothree
        // abcone2threexyz
        // xtwone3four
        // 4nineeightseven2
        // zoneight234
        // 7pqrstsixteen";

        let input = include_str!("../inputs/day1input.txt");
        let numbers_values = Arc::new(vec![
            Numbers::new("one", 1),
            Numbers::new("two", 2),
            Numbers::new("three", 3),
            Numbers::new("four", 4),
            Numbers::new("five", 5),
            Numbers::new("six", 6),
            Numbers::new("seven", 7),
            Numbers::new("eight", 8),
            Numbers::new("nine", 9),
        ]);

        let numbers = input
            .split('\n')
            .filter_map(|line| {
                let mut first_value = None;
                let mut last_value = None;

                let chars = line.chars().collect::<Vec<char>>();
                for i in 0..chars.len() {
                    let num = chars[i].to_digit(10);

                    if let Some(num) = num {
                        if first_value.is_none() {
                            first_value = Some(num);
                        }
                        last_value = Some(num);
                    } else {
                        for values in numbers_values.clone().as_ref() {
                            if i + values.key.len() > chars.len() {
                                continue;
                            }

                            let string =
                                chars[i..(i + values.key.len())].iter().collect::<String>();

                            if string == values.key {
                                if first_value.is_none() {
                                    first_value = Some(values.value);
                                }
                                last_value = Some(values.value);
                                break;
                            }
                        }
                    }
                }

                if let (Some(first_value), Some(last_value)) = (first_value, last_value) {
                    Some(first_value * 10 + last_value)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let sum: u32 = numbers.iter().sum();
        println!("{sum}");
    }
}
