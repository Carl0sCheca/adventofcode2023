use std::collections::HashSet;

use crate::Run;

pub struct Day3Part2;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Numbers {
    value: u32,
    start: usize,
    end: usize,
}

impl Run for Day3Part2 {
    fn run() {
        let _input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let input = include_str!("../inputs/day3input.txt");

        const VALID_POSITIONS: [(i32, i32); 8] = [
            // up
            (-1, -1),
            (0, -1),
            (1, -1),
            // left
            (-1, 0),
            // right
            (1, 0),
            // down
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        let chars = input.chars().filter(|x| x != &' ').collect::<Vec<_>>();

        let width = chars.iter().position(|&x| x == '\n').unwrap() + 1;
        let height = chars.len() / width;

        let mut valid_numbers = vec![];

        for y in 0..=height {
            let mut start_number = None;
            for x in 0..=width {
                let position = x + (y * width);

                if position > chars.len() {
                    break;
                }

                // get number
                if position == chars.len()
                    || chars[position] == '.'
                    || chars[position] == '\n'
                    || !NUMBERS.contains(&chars[position])
                {
                    if let Some(st_number) = start_number {
                        let number_string = chars[st_number..position].iter().collect::<String>();

                        let number = number_string.parse::<u32>();
                        if let Ok(value) = number {
                            valid_numbers.push(Numbers {
                                value,
                                start: st_number,
                                end: position,
                            });
                        }

                        start_number = None;
                    }
                    continue;
                }

                if start_number.is_none() {
                    start_number = Some(position);
                }
            }
        }

        // valid_numbers.iter().for_each(|f| println!("{}", f.value));
        // 78*78+12*56
        let mut valid_gears = vec![];
        'outer: for y in 0..height {
            for x in 0..=width {
                let position = x + (y * width);

                if position >= chars.len() {
                    break 'outer;
                }

                if chars[position] == '*' {
                    // find adjacencies
                    let mut found_numbers = HashSet::new();
                    for valid_position in VALID_POSITIONS {
                        let new_x = x as i32 + valid_position.0;
                        let new_y = y as i32 + valid_position.1;

                        // if new_x < 0 || new_y < 0 || new_y >= height as i32 || new_x >= width as i32
                        // {
                        //     continue;
                        // }

                        let new_position = new_x + new_y * width as i32;

                        let found = valid_numbers
                            .iter()
                            .filter(|num| {
                                new_position as usize >= num.start
                                    && (new_position as usize) < num.end
                            })
                            .collect::<Vec<_>>();

                        if !found.is_empty() {
                            found_numbers.insert(found[0]);
                        }
                    }

                    if found_numbers.len() == 2 {
                        let mut iter = found_numbers.iter().take(2);
                        let first_value = iter.next().unwrap();
                        let second_value = iter.next().unwrap();
                        valid_gears.push(first_value.value * second_value.value);
                    }
                }
            }
        }

        println!("{}", valid_gears.iter().sum::<u32>());
    }
}
