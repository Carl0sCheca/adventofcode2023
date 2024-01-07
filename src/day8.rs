use crate::Run;
use std::collections::HashMap;
pub struct Day8;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Movement {
    position: usize,
    movements: Vec<char>,
}

impl Run for Day8 {
    fn run() {
        let _input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let _input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let _input = include_str!("../inputs/day8input.txt");

        let mut movements = Movement {
            position: 0,
            movements: _input
                .lines()
                .take(1)
                .map(|line| line)
                .collect::<String>()
                .chars()
                .collect::<Vec<char>>(),
        };

        let mut nodes = HashMap::new();

        _input.lines().skip(2).for_each(|node| {
            let origin = node[0..3].to_owned();
            let left = node[7..10].to_owned();
            let right = node[12..15].to_owned();
            nodes.insert(origin, Node { left, right });
        });

        let mut count = 0usize;
        let mut origin = "AAA";

        while origin != "ZZZ" {
            let dir = movements.movements[movements.position];
            match dir {
                'R' => {
                    origin = &nodes[origin].right;
                }
                'L' => {
                    origin = &nodes[origin].left;
                }
                _ => {}
            }
            movements.position = (movements.position + 1) % movements.movements.len();
            count += 1;
        }

        println!("{count}");
    }
}
