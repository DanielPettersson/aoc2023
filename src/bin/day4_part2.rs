use std::collections::{HashMap, HashSet};
use aoc2023::day_input;

fn main() {
    let mut card_count_map: HashMap<usize, u64> = HashMap::new();
    for (id, num_wins) in day_input::lines().map(parse).enumerate() {

        let count = *card_count_map.entry(id).or_default() + 1;
        card_count_map.insert(id, count);

        for won_id in id + 1..id + 1 + num_wins {
            let count = *card_count_map.entry(won_id).or_default() + count;
            card_count_map.insert(won_id, count);
        }

    }
    let answer = card_count_map.values().sum::<u64>();
    println!("{answer}");
}

fn parse(line: &str) -> usize {
    let winning_nums = chunk_nums(&line[10..39]);
    let nums = chunk_nums(&line[42..]);
    winning_nums.intersection(&nums).count()
}

fn chunk_nums(line: &str) -> HashSet<u8> {
    line.chars()
        .collect::<Vec<char>>()
        .chunks(3).map(|c| {
        String::from_iter(c).trim().parse().unwrap()
    }).collect()
}