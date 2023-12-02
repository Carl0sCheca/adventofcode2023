use crate::Run;

pub struct Day2;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct Cube {
    color: Color,
    quantity: u32,
}

impl Run for Day2 {
    fn run() {
        //         let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let input = include_str!("../inputs/day2input.txt");

        // 12 red cubes, 13 green cubes, and 14 blue cubes
        let initial_config = [
            Cube {
                color: Color::Red,
                quantity: 12,
            },
            Cube {
                color: Color::Green,
                quantity: 13,
            },
            Cube {
                color: Color::Blue,
                quantity: 14,
            },
        ];

        let valid_games = input.split('\n').filter_map(|line| {
            // id
            let colon = line.find(':')?;
            let id: u32 = line["Game ".len()..colon].to_owned().parse().unwrap();
            let string = line[colon + 1..].to_owned();
            let subset = string.split(';').collect::<Vec<_>>();

            // is valid
            let mut valid = true;
            for s in subset {
                let mut cubes = vec![];
                for element in s.split(',') {
                    let index = element[1..].find(' ').unwrap() + 1;
                    let quantity: u32 = element[1..index].to_owned().parse().unwrap();
                    let color = element[index + 1..].to_owned();

                    match color.as_str() {
                        "red" => Self::push(
                            &mut cubes,
                            Cube {
                                color: Color::Red,
                                quantity,
                            },
                        ),
                        "green" => Self::push(
                            &mut cubes,
                            Cube {
                                color: Color::Green,
                                quantity,
                            },
                        ),
                        "blue" => Self::push(
                            &mut cubes,
                            Cube {
                                color: Color::Blue,
                                quantity,
                            },
                        ),
                        _ => (),
                    };
                }

                for cube in &mut cubes {
                    if let Some(index) = initial_config.iter().position(|x| x.color == cube.color) {
                        if cube.quantity > initial_config[index].quantity {
                            valid = false;
                        }
                    }
                }

                if !valid {
                    break;
                }
            }

            if valid {
                Some(id)
            } else {
                None
            }
        });

        let sum = valid_games.sum::<u32>();

        println!("{sum}");
    }
}

impl Day2 {
    fn push(list: &mut Vec<Cube>, cube: Cube) {
        if let Some(index) = list.iter().position(|x| x.color == cube.color) {
            list[index].quantity += cube.quantity;
        } else {
            list.push(cube);
        }
    }
}
