use crate::Run;

pub struct Day1 {}

impl Run for Day1 {
    fn run() {
        //         let input = r"1abc2
        // pqr3stu8vwx
        // a1b2c3d4e5f
        // treb7uchet";

        let input = include_str!("../inputs/day1input.txt");

        let split = input.split('\n').collect::<Vec<&str>>();
        let mut numbers = vec![];

        for i in split {
            let mut first_value = None;
            let mut last_value = None;

            for c in i.chars() {
                let num = c.to_digit(10);

                if let Some(num) = num {
                    if first_value.is_none() {
                        first_value = Some(num);
                    }
                    last_value = Some(num);
                }
            }

            if let (Some(first_value), Some(last_value)) = (first_value, last_value) {
                numbers.push(first_value * 10 + last_value);
            }
        }
        let sum: u32 = numbers.iter().sum();
        println!("{sum}");
    }
}
