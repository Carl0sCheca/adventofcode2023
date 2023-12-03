use crate::Run;

pub struct Day3;

impl Run for Day3 {
    fn run() {
        //         let input = r"467..114..
        // ...*......
        // ..35..633.
        // ......#...
        // 617*......
        // .....+.58.
        // ..592.....
        // ......755.
        // ...$.*....
        // .664.598..";

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

        let width = input.find('\n').unwrap() + 1;
        let height = (input.len() / width) + 1;

        let chars = input.chars().collect::<Vec<_>>();

        let mut valid_numbers = vec![];

        for y in 0..height {
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
                        let number_string = input[st_number..position].to_owned();

                        let number = number_string.parse::<u32>();
                        if let Ok(number) = number {
                            // check valid number
                            'outer: for i in st_number..position {
                                for valid_position in VALID_POSITIONS {
                                    let x = i % width;
                                    let y = i / width;

                                    let new_position = (x as i32 + valid_position.0)
                                        + (y as i32 + valid_position.1) * width as i32;

                                    if new_position < 0 || new_position as usize >= chars.len() {
                                        continue;
                                    }

                                    if !NUMBERS.contains(&chars[new_position as usize])
                                        && chars[new_position as usize] != '.'
                                        && chars[new_position as usize] != '\n'
                                    {
                                        valid_numbers.push(number);
                                        break 'outer;
                                    }
                                }
                            }
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

        println!("{}", valid_numbers.iter().sum::<u32>());
    }
}
