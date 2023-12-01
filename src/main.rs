mod day1;
mod day1_part2;

trait Run {
    fn run();
}

fn main() {
    let days = [day1::Day1::run, day1_part2::Day1Part2::run];

    days.iter().for_each(|f| f());
}
