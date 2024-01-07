use crate::Run;
use std::collections::HashMap;
pub struct Day8Part2;

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

impl Run for Day8Part2 {
    fn run() {
        let _input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

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

        let mut origins = nodes
            .keys()
            .filter_map(|p| {
                if p.ends_with("A") {
                    Some(p.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        let mut ends = vec![];

        let mut count = 1usize;

        loop {
            let dir = movements.movements[movements.position];
            let mut delete = vec![];
            for (i, origin) in &mut origins.iter_mut().enumerate() {
                match dir {
                    'R' => {
                        *origin = nodes[origin].right.to_owned();
                    }
                    'L' => {
                        *origin = nodes[origin].left.to_owned();
                    }
                    _ => {}
                }

                if origin.ends_with("Z") {
                    ends.push(count);
                    delete.push(i);
                }
            }

            for del in delete {
                origins.remove(del);
            }

            movements.position = (movements.position + 1) % movements.movements.len();
            count += 1;

            if origins.len() == 0 {
                break;
            }
        }

        let result = ends.iter().fold(1, |acc, &e| num::integer::lcm(e, acc));

        println!("{result:?}");
    }
}
