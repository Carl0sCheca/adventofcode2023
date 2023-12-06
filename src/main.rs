mod day1;
mod day1_part2;
mod day2;
mod day2_part2;
mod day3;
mod day3_part2;
mod day4;
mod day4_part2;
mod day5;
mod day5_part2;
mod day6;
mod day6_part2;

trait Run {
    fn run();
}

fn main() {
    let days = [
        day1::Day1::run,
        day1_part2::Day1Part2::run,
        day2::Day2::run,
        day2_part2::Day2Part2::run,
        day3::Day3::run,
        day3_part2::Day3Part2::run,
        day4::Day4::run,
        day4_part2::Day4Part2::run,
        day5::Day5::run,
        day5_part2::Day5Part2::run,
        day6::Day6::run,
        day6_part2::Day6Part2::run,
    ];

    days.iter().for_each(|f| f());
}
