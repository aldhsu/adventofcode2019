use std::fs;
use std::io;
mod computer;
use computer::Computer;
use computer::State;
use std::collections::HashMap;
use std::fmt;
use std::cmp::Ordering;

fn main() {
    // let result = part1();
    // println!("part1 {}", result);

    part2();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    x: i32,
    y: i32,
    tile_type: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Type {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Type {
    fn from(value: i64) -> Self {
        match value {
            0 => Type::Empty,
            1 => Type::Wall,
            2 => Type::Block,
            3 => Type::Paddle,
            4 => Type::Ball,
            _ => unimplemented!(),
        }
    }
}

fn input_to_registers() -> Vec<i64> {
    let input = fs::read_to_string("input.txt").unwrap();
    input
        .split(",")
        .map(|x| match x.trim().parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                println!("could not parse {}", x);
                panic!("could not parse");
            }
        })
        .collect::<Vec<_>>()
}

struct Game {
    map: HashMap<(i64, i64), Type>,
    ball_position: (i64, i64),
    paddle_position: (i64, i64),
    score: i64,
}

impl Game {
    fn new(comp: &mut Computer) -> Self {
        let mut game = Self {
            map: HashMap::new(),
            ball_position: (0, 0),
            paddle_position: (0, 0),
            score: 0,
        };
        game.update(comp);
        game
    }

    fn insert(&mut self, x: i64, y: i64, item: Type) {
        match item {
            Type::Ball => {
                self.map.remove(&self.ball_position);
                self.ball_position = (x, y);
            },
            Type::Paddle => {
                self.map.remove(&self.ball_position);
                self.paddle_position = (x, y);
            },
            _ => {}
        }
        self.map.insert((x, y), item);
    }

    fn update(&mut self, comp: &mut Computer) {
        comp.run();

        while comp.outputs.len() > 0 {
            let x = comp.outputs.pop_front().unwrap();
            let y = comp.outputs.pop_front().unwrap();
            let z = comp.outputs.pop_front().unwrap();
            if x == -1 && y == 0 {
                self.score = z;
            } else {
                self.insert(x, y, Type::from(z));
            }
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut canvas = [[' '; 42]; 26];

        for ((x, y), tile_type) in &self.map {
            canvas[*y as usize][*x as usize] = match tile_type {
                Type::Empty => ' ',
                Type::Ball => '@',
                Type::Wall => '#',
                Type::Block => '$',
                Type::Paddle => '-',
            }
        }

        let result = canvas
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}\n{}", self.score, result)
    }
}

fn part1() -> usize {
    let mut registers = input_to_registers();
    let mut comp = Computer::new(registers);
    let game = Game::new(&mut comp);
    game
        .map
        .values()
        .filter(|&&tile| tile == Type::Block)
        .count()
}

fn part2() -> usize {
    let mut registers = input_to_registers();
    registers[0] = 2;
    let mut comp = Computer::new(registers);
    let mut game = Game::new(&mut comp);

    while comp.state != State::Halted {
        // let one_second = std::time::Duration::from_millis(10);
        // std::thread::sleep(one_second);
        // print!("{}[2J", 27 as char);
        // println!("{}", game);

        match comp.state {
            State::Waiting => {
                let (ball_x, _) = game.ball_position;
                let (paddle_x, _) = game.paddle_position;
                let input = match ball_x.cmp(&paddle_x) {
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                };
                comp.input(input);
                game.update(&mut comp);
            }
            _ => break,
        }
    }
    println!("{}", game.score);
    0
}
