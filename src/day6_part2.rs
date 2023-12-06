use crate::Run;

pub struct Day6Part2;

impl Run for Day6Part2 {
    fn run() {
        let _input = r"Time:      7  15   30
Distance:  9  40  200
";

        let _input = include_str!("../inputs/day6input.txt");

        let time = _input
            .split('\n')
            .take(1)
            .flat_map(|line| line.split(' ').skip(1).collect::<Vec<_>>())
            .filter(|line| !line.is_empty())
            .fold(String::new(), |acc, e| format!("{acc}{e}"))
            .parse::<u64>()
            .unwrap();

        let distance = _input
            .split('\n')
            .skip(1)
            .take(1)
            .flat_map(|line| line.split(' ').skip(1).collect::<Vec<_>>())
            .filter(|line| !line.is_empty())
            .fold(String::new(), |acc, e| format!("{acc}{e}"))
            .parse::<u64>()
            .unwrap();

        let final_result = (0..=time)
            .into_iter()
            .filter(|j| ((time - *j) * *j) > distance)
            .collect::<Vec<u64>>()
            .len() as u64;

        println!("{final_result:?}");
    }
}
