use std::collections::{HashMap, HashSet};

use crate::Run;

pub struct Day4Part2;

impl Run for Day4Part2 {
    fn run() {
        let _input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let _input = include_str!("../inputs/day4input.txt");

        let mut cards = HashMap::new();
        let mut duplicated_cards = HashMap::new();

        let lines = _input.split('\n').collect::<Vec<_>>();

        lines.iter().for_each(|line| {
            let colon = line.find(':');
            if let Some(colon) = colon {
                let id = line["Card".len()..colon]
                    .to_owned()
                    .trim()
                    .parse::<u32>()
                    .unwrap();

                cards.insert(id, 1);

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
                    (id..=id + intersection.len() as u32).for_each(|i| {
                        if i != id {
                            *duplicated_cards.entry(i).or_insert(0) += 1;
                        }
                    });

                    // copies
                    if duplicated_cards.contains_key(&id) {
                        for _ in 0..duplicated_cards[&id] {
                            (id..=id + intersection.len() as u32).for_each(|i| {
                                if i != id {
                                    *duplicated_cards.entry(i).or_insert(0) += 1;
                                }
                            });
                        }
                    }
                }
            }
        });

        for i in cards.values() {
            *duplicated_cards.entry((*i) as u32 + 1).or_insert(0) += 1;
        }

        let sum: i32 = duplicated_cards.values().sum();
        println!("{sum}");
    }
}
