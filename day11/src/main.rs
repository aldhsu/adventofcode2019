use itertools::Itertools;
use std::fs;
use std::io;
mod computer;
use computer::Computer;
use computer::State;
use std::collections::HashMap;

fn main() {
    // let result = part1();
    // println!("part1 {}", result);

    let result = part2();
    // println!("part2 {}", result);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    x: i32,
    y: i32,
    color: Color,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Color {
    Black,
    White,
}

impl From<i64> for Color {
    fn from(value: i64) -> Self {
        match value {
            0 => Color::Black,
            1 => Color::White,
            _ => unimplemented!(),
        }
    }
}

impl Into<i64> for Color {
    fn into(self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

struct Robot {
    position: (i32, i32),
    facing: Facing,
}

impl Robot {
    const FACING_ORDER : [Facing; 4] = [Facing::Up, Facing::Right, Facing::Down, Facing::Left];

    fn execute(&mut self, rotation: Rotation) {
        let current_position = Self::FACING_ORDER.iter().position(|f| f == &self.facing).unwrap();
        self.facing = match rotation {
            Rotation::Left => { 
                let next_position = current_position as isize - 1;
                match next_position {
                    -1 => Self::FACING_ORDER[3],
                    _ => Self::FACING_ORDER[next_position as usize],
                }
            }
            Rotation::Right => { 
                Self::FACING_ORDER[((current_position as isize + 1) % 4).abs() as usize]
            }
        };
        dbg!(self.facing);
        self.move_square()
    }

    fn move_square(&mut self) {
        match self.facing {
            Facing::Left => self.position.0 -= 1,
            Facing::Right => self.position.0 += 1,
            Facing::Up => self.position.1 += 1,
            Facing::Down => self.position.1 -= 1,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Facing {
    Left,
    Right,
    Up,
    Down,
}

enum Rotation {
    Left,
    Right,
}

impl From<i64> for Rotation {
    fn from(i: i64) -> Self {
        match i {
            0 => Rotation::Left,
            1 => Rotation::Right,
            _ => unimplemented!(),
        }
    }
}

fn part1() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let registers = input
        .split(",")
        .map(|x| match x.trim().parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                println!("could not parse {}", x);
                panic!("could not parse");
            }
        })
        .collect::<Vec<_>>();

    let mut map = HashMap::new();
    let mut robot = Robot { position: (0, 0), facing: Facing::Up, };
    let mut comp = Computer::new(registers);
    let mut moves = vec![];

    while comp.state != State::Halted {
        let current_tile = map.entry(robot.position).or_insert(Color::Black);
        comp.input(current_tile.clone() as i64);
        comp.run();
        let color = Color::from(comp.outputs.pop_front().expect("no color"));
        moves.push((robot.position, color.clone()));
        *current_tile = color;

        let direction = Rotation::from(comp.outputs.pop_front().expect("no direction"));
        robot.execute(direction);
    }

    dbg!(&moves[..10]);
    dbg!(&moves.len());
    map.values().count()
}

fn part2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let registers = input
        .split(",")
        .map(|x| match x.trim().parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                println!("could not parse {}", x);
                panic!("could not parse");
            }
        })
        .collect::<Vec<_>>();

    let mut map = HashMap::new();
    map.insert((0, 5), Color::White);
    let mut robot = Robot { position: (0, 5), facing: Facing::Up, };
    let mut comp = Computer::new(registers);
    let mut moves = vec![];

    while comp.state != State::Halted {
        let current_tile = map.entry(robot.position).or_insert(Color::Black);
        comp.input(current_tile.clone() as i64);
        comp.run();
        let color = Color::from(comp.outputs.pop_front().expect("no color"));
        moves.push((robot.position, color.clone()));
        *current_tile = color;

        let direction = Rotation::from(comp.outputs.pop_front().expect("no direction"));
        robot.execute(direction);
    }

    let mut canvas = [[Color::Black; 43]; 6];

    for ((x, y), color) in moves {
        canvas[y as usize][x as usize] = color;
    }

    for line in &canvas {
        println!("{}", line.iter().map(|color| {
            match color {
                Color::Black => ' ',
                Color::White => '@',
            }
        }).collect::<String>());
    }

    // let ((min_x,_), _) = moves.iter().min_by(|((x1, _), _), ((x2, _), _)| {
    //     x1.cmp(x2)
    // }).unwrap();
    // let ((max_x,_), _) = moves.iter().max_by(|((x1, _), _), ((x2, _), _)| {
    //     x1.cmp(x2)
    // }).unwrap();
    //
    // let ((_, min_y), _) = moves.iter().min_by(|((_, y1), _), ((_, y2), _)| {
    //     y1.cmp(y2)
    // }).unwrap();
    //
    // let ((_, max_y), _) = moves.iter().max_by(|((_, y1), _), ((_, y2), _)| {
    //     y1.cmp(y2)
    // }).unwrap();
    // dbg!((min_x, max_x, min_y, max_y));
}
