use std::fs;
mod computer;
use computer::Computer;
use computer::State;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let result = part1();
    // println!("part1 {}", result);
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct Tile {
//     x: i32,
//     y: i32,
//     tile_type: Type,
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Type {
    Empty,
    Wall,
    Oxygen,
}

impl From<i64> for Type {
    fn from(value: i64) -> Self {
        match value {
            0 => Type::Empty,
            1 => Type::Wall,
            2 => Type::Oxygen,
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

struct Node {
    x: isize,
    y: isize,
    computer: Computer,
    step: usize,
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl From<i64> for Dir {
    fn from(i: i64) -> Self {
        match i {
            1 => Dir::North,
            2 => Dir::South,
            3 => Dir::West,
            4 => Dir::East,
            _ => unimplemented!(),
        }
    }
}

impl From<Dir> for i64 {
    fn from(i: Dir) -> Self {
        match i {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

impl Dir {
    fn change(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Dir::North => (x, y + 1),
            Dir::East => (x + 1, y),
            Dir::South => (x, y - 1),
            Dir::West => (x - 1, y),
        }
    }
}

const DIRECTIONS: [Dir; 4] = [Dir::East, Dir::South, Dir::West, Dir::North];

type Map = HashMap<(isize, isize), Type>;

fn part1() {
    let mut map: Map = HashMap::new();
    map.insert((0, 0), Type::Empty);
    let mut queue: VecDeque<Node> = VecDeque::new();
    queue.push_back(Node {
        x: 0,
        y: 0,
        computer: Computer::new(&input_to_registers()),
        step: 0,
    });

    let mut count = 0;
    // while !queue.is_empty() {
        while count < 10 {
        count += 1;
        let current = queue.pop_front().unwrap();

        for direction in DIRECTIONS.iter() {
            let new_coords = direction.change(current.x, current.y);
            if map.get(&new_coords).is_some() {
                continue;
            }

            let mut comp = current.computer.clone();
            let a: Dir = *direction;
            let b: i64 = a.into();
            comp.input(b);
            comp.run();
            // dbg!(new_registers == current.registers);
            let output = comp.outputs.pop_front().expect("no tile type");
            let tile_type = Type::from(output);
            match tile_type {
                Type::Wall => {
                    map.insert(new_coords, Type::Wall);
                }
                t => {
                    if t == Type::Oxygen {
                        dbg!("found oxygen");
                    }
                    map.insert(new_coords, t);
                    queue.push_back(Node {
                        x: new_coords.0,
                        y: new_coords.1,
                        computer: comp.clone(),
                        step: count,
                    });
                }
            }
        }

        visualize(&map);
        // dbg!(queue.iter().map(|node| (node.x, node.y)).collect::<Vec<_>>());
    }

    dbg!(map);
}

fn visualize(map: &Map) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;
    for (x, y) in map.keys() {
        if *x > max_x {
            max_x = *x
        }
        if *x < min_x {
            min_x = *x
        }
        if *y > max_y {
            max_y = *y
        }
        if *y < min_y {
            min_y = *y
        }
    }

    let mut canvas = vec![vec![' '; (max_x + min_x.abs() + 10) as usize]; (max_y + min_y.abs() + 10) as usize];

    for ((x, y), tile_type) in map {
        canvas[(*y + min_y.abs()) as usize][(*x + min_x.abs()) as usize] = match tile_type {
            Type::Wall => '#',
            Type::Empty => '.',
            Type::Oxygen => '@',
        };
    }

    for line in canvas {
        println!("{}", line.iter().collect::<String>());
    }
}
