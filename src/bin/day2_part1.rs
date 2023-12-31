use aoc2023::day_input;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let answer: u32 = day_input::lines().map(parse_game).filter(|g| g.is_possible()).map(|g| g.id).sum();
    println!("{answer}");
}

fn parse_game(line: &str) -> Game {
    let split_game: Vec<&str> = line.split(':').collect();
    let id: u32  = split_game.first().unwrap()[5..].parse().unwrap();

    let split_draws: Vec<&str> = split_game.last().unwrap().split(';').collect();
    let mut draws: Vec<Draw> = Vec::with_capacity(split_draws.len());

    for draw_str in split_draws {
        let mut draw = Draw::default();
        for draw_color_str in draw_str.split(", ") {
            let split_draw_color_str: Vec<&str> = draw_color_str.trim().split(' ').collect();
            let number: u32 = split_draw_color_str.first().unwrap().parse().unwrap();

            match *split_draw_color_str.last().unwrap() {
                "red" => draw.red = number,
                "green" => draw.green = number,
                "blue" => draw.blue = number,
                col => panic!("unexpected color: {col}")
            };
        }
        draws.push(draw);
    }

    Game {
        id,
        draws,
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

#[derive(Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32
}

impl Draw {
    fn is_possible(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }
}