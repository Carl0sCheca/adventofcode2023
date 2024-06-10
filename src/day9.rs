pub struct Day9;

impl crate::Run for Day9 {
    fn run() {
        let _input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        // let _input = include_str!("../inputs/day9input.txt");

        let lines = _input
            .split('\n')
            .map(|line| {
                line.split_whitespace()
                    .map(|number| number.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut difference = vec![];
        for j in 0..lines.len() {
            difference.push(vec![]);
            // if difference[j].iter().sum() > 0 {

            // }

            difference[j].iter().for_each(|c| println!("{c} <-"));

            for i in 0..lines[j].len() - 1 {
                difference[0].push(lines[j][i + 1] - lines[j][i]);
            }

            println!("{difference:?}");
        }
    }
}
