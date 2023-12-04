use std::collections::HashSet;

use crate::Run;

pub struct Day4;

impl Run for Day4 {
    fn run() {
        //         let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let input = include_str!("../inputs/day4input.txt");

        let cards: i32 = input
            .split('\n')
            .filter_map(|line| {
                let colon = line.find(':')?;
                let _id = line["Card".len()..colon]
                    .to_owned()
                    .trim()
                    .parse::<u32>()
                    .unwrap();

                let mut result = [HashSet::new(), HashSet::new()];

                line[colon + 1..]
                    .to_owned()
                    .split('|')
                    .enumerate()
                    .for_each(|g| {
                        g.1.to_owned().split(' ').for_each(|v| {
                            let value = v.trim().parse::<u32>();
                            if let Ok(value) = value {
                                result[g.0].insert(value);
                            }
                        });
                    });

                let intersection: HashSet<_> = result[0].intersection(&result[1]).collect();

                if !intersection.is_empty() {
                    Some(2i32.pow((intersection.len() as i32 - 1).try_into().unwrap()))
                } else {
                    None
                }
            })
            .sum();

        println!("{cards}");
    }
}
