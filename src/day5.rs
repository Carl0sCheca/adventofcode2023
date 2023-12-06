use crate::Run;

pub struct Day5;

enum ParserState {
    Init,
    Map,
    Numbers,
}

#[derive(Debug, Clone, Copy)]
struct MapRange {
    from: u64,
    to: u64,
    size: u64,
}

impl Run for Day5 {
    fn run() {
        let _input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let _input = include_str!("../inputs/day5input.txt");

        let seeds = _input
            .split('\n')
            .take(1)
            .map(|line| line["seeds:".len()..].to_owned())
            .map(|line| {
                line.split(' ')
                    .filter_map(|seed| seed.parse::<u64>().ok())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<_>>();
        let seeds = seeds[0].clone();

        let mut parser_state = ParserState::Init;

        let list = _input.split('\n').skip(1).enumerate().collect::<Vec<_>>();

        let mut vector = vec![];
        let mut results = vec![];
        for (i, line) in &list {
            match parser_state {
                ParserState::Init => parser_state = ParserState::Map,
                ParserState::Map => {
                    parser_state = ParserState::Numbers;
                }
                ParserState::Numbers => {
                    let numbers = line
                        .split(' ')
                        .filter_map(|seed| seed.parse::<u64>().ok())
                        .collect::<Vec<u64>>();

                    vector.push((numbers[0], numbers[1], numbers[2]));

                    if (*i + 1 < list.len() && list[*i + 1].1.is_empty()) || *i + 1 == list.len() {
                        parser_state = ParserState::Init;

                        let mut list_ranges = vec![];
                        for (from, to, size) in &vector {
                            list_ranges.push(MapRange {
                                from: *from,
                                to: *to,
                                size: *size,
                            });
                        }
                        results.append(&mut vec![list_ranges]);

                        vector.clear();
                    }
                }
            }
        }

        let final_result = seeds
            .iter()
            .map(|seed| {
                let mut check_value = *seed;

                for result in &results {
                    let index = result
                        .iter()
                        .filter(|r| check_value >= r.to && check_value < r.to + r.size)
                        .collect::<Vec<_>>();

                    if !index.is_empty() {
                        check_value = (check_value - index[0].to) + index[0].from;
                    }
                }

                check_value
            })
            .min()
            .unwrap();

        println!("{final_result}");
    }
}
