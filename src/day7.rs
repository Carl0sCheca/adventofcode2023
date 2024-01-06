use crate::Run;
pub struct Day7;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A = 0,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Strength {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Game {
    strength: Strength,
    cards: Vec<Card>,
    bid: u32,
}

impl Run for Day7 {
    fn run() {
        let _input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let _input = r"2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

        let _input = include_str!("../inputs/day7input.txt");

        let parsed_games = _input.split('\n').filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some((
                    line.chars()
                        .take(5)
                        .into_iter()
                        .filter_map(|c| match c {
                            'A' => Some(Card::A),
                            'K' => Some(Card::K),
                            'Q' => Some(Card::Q),
                            'J' => Some(Card::J),
                            'T' => Some(Card::T),
                            '9' => Some(Card::Nine),
                            '8' => Some(Card::Eight),
                            '7' => Some(Card::Seven),
                            '6' => Some(Card::Six),
                            '5' => Some(Card::Five),
                            '4' => Some(Card::Four),
                            '3' => Some(Card::Three),
                            '2' => Some(Card::Two),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                    line[5..].trim().parse::<u32>().unwrap(),
                ))
            }
        });

        let mut result = parsed_games
            .filter_map(|read| {
                let mut result = vec![0u32; 13];

                for card in &read.0 {
                    result[*card as usize] += 1;
                }

                let max_value = *result.iter().max().unwrap();

                return match max_value {
                    5 => {
                        // five
                        Some(Game {
                            strength: Strength::Five,
                            bid: read.1,
                            cards: read.0,
                        })
                    }
                    4 => {
                        // four
                        Some(Game {
                            strength: Strength::Four,
                            bid: read.1,
                            cards: read.0,
                        })
                    }
                    3 => {
                        result.retain(|&x| x != max_value);
                        let next_max_value = *result.iter().max().unwrap_or(&0);

                        if next_max_value == 2 {
                            // fullhouse
                            Some(Game {
                                strength: Strength::FullHouse,
                                bid: read.1,
                                cards: read.0,
                            })
                        } else {
                            // three of a kind
                            Some(Game {
                                strength: Strength::Three,
                                bid: read.1,
                                cards: read.0,
                            })
                        }
                    }
                    2 => {
                        let positions: Vec<usize> = result
                            .iter()
                            .enumerate()
                            .filter(|&(_, &num)| num == max_value)
                            .map(|(index, _)| index)
                            .collect();

                        if positions.len() == 2 {
                            // two pair
                            Some(Game {
                                strength: Strength::TwoPair,
                                bid: read.1,
                                cards: read.0,
                            })
                        } else {
                            // one pair
                            Some(Game {
                                strength: Strength::OnePair,
                                bid: read.1,
                                cards: read.0,
                            })
                        }
                    }
                    1 => Some(Game {
                        strength: Strength::HighCard,
                        bid: read.1,
                        cards: read.0,
                    }),
                    _ => None,
                };
            })
            .collect::<Vec<_>>();

        result.sort_unstable();
        result.reverse();

        let final_value = result
            .iter()
            .enumerate()
            .fold(0, |acc, (i, game)| (game.bid * (i as u32 + 1)) + acc);
        println!("{final_value}");
    }
}
