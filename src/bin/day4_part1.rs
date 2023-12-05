use std::collections::HashSet;
use aoc2023::day_input;

fn main() {
    let cards: Vec<Card> = day_input::lines().map(Card::parse).collect();
    let answer: u64 = cards.iter().map(|c| c.points()).sum();
    println!("{answer}");
}

#[derive(Debug)]
struct Card {
    winning_nums: HashSet<u8>,
    nums: HashSet<u8>,
}

impl Card {
    fn parse(line: &str) -> Card {
        let winning_nums = chunk_nums(&line[10..39]);
        let nums = chunk_nums(&line[42..]);
        Card {
            winning_nums,
            nums,
        }
    }

    fn points(&self) -> u64 {
        let wins = self.winning_nums.intersection(&self.nums).count() as u32;
        if wins > 0 {
            2u64.pow(wins - 1)
        } else {
            0
        }
    }
}

fn chunk_nums(line: &str) -> HashSet<u8> {
    line.chars()
        .collect::<Vec<char>>()
        .chunks(3).map(|c| {
        String::from_iter(c).trim().parse().unwrap()
    }).collect()
}