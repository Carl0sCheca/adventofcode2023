use crate::Run;

pub struct Day6;

impl Run for Day6 {
    fn run() {
        let _input = r"Time:      7  15   30
Distance:  9  40  200
";

        let _input = include_str!("../inputs/day6input.txt");

        let times = _input
            .split('\n')
            .take(1)
            .flat_map(|line| {
                line.split(' ')
                    .skip(1)
                    .filter_map(|p| p.parse().ok())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<_>>();

        let distances = _input
            .split('\n')
            .skip(1)
            .take(1)
            .flat_map(|line| {
                line.split(' ')
                    .skip(1)
                    .filter_map(|p| p.parse().ok())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<_>>();

        let final_result = (0..times.len())
            .into_iter()
            .map(|i| {
                (0..=times[i])
                    .into_iter()
                    .filter(|j| ((times[i] - *j) * *j) > distances[i])
                    .collect::<Vec<u64>>()
            })
            .fold(1, |acc, e| acc * e.len() as u64);

        println!("{final_result:?}");
    }
}
