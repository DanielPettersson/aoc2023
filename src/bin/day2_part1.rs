use aoc2023::day_input;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let answer: u32 = day_input::lines().map(parse_game).filter(|g| g.is_possible()).map(|g| g.id).sum();
    println!("{answer}");
}

fn parse_game(line: &str) -> Game {
    Game {
        id: 1,
        draws: vec![],
    }
}

struct Game {
    id: u32,
    draws: Vec<Draw>
}

impl Game {
    fn is_possible(&self) -> bool {
        self.draws.iter().all(|d| d.is_possible())
    }
}

struct Draw {
    red: u32,
    green: u32,
    blue: u32
}

impl Draw {
    fn is_possible(&self) -> bool {
        self.red < MAX_RED && self.green < MAX_GREEN && self.blue < MAX_BLUE
    }
}