use std::fs;
mod computer;
use computer::Computer;
use computer::State;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    part1();
    part2();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Type {
    Empty,
    Wall,
    Oxygen,
    Start,
}

impl From<i64> for Type {
    fn from(value: i64) -> Self {
        match value {
            1 => Type::Empty,
            0 => Type::Wall,
            2 => Type::Oxygen,
            _ => unimplemented!(),
        }
    }
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

type Map = HashMap<(isize, isize), (Type, usize)>;

fn part1() {
    let mut map: Map = HashMap::new();
    map.insert((0, 0), (Type::Start, 0));
    let mut queue: VecDeque<Node> = VecDeque::new();
    queue.push_back(Node {
        x: 0,
        y: 0,
        computer: Computer::new(input_to_registers()),
        step: 0,
    });

    while let Some(current) = queue.pop_front() {
        for direction in DIRECTIONS.iter() {
            if let Some(node) = iterate(&current.computer, direction, &current, &mut map) {
                queue.push_back(node)
            }
        }

    }
    visualize(&map);

    for ((x, y), (tile, step)) in map.iter() {
        if tile == &Type::Oxygen {
            println!("{:?}", (x, y));
            println!("oxygen on move: {}", step);
        }
    }
}

fn part2() {
    let mut map: Map = HashMap::new();
    map.insert((0, 0), (Type::Start, 0));
    let mut queue: VecDeque<Node> = VecDeque::new();
    queue.push_back(Node {
        x: 0,
        y: 0,
        computer: Computer::new(input_to_registers()),
        step: 0,
    });

    'bfs: while let Some(current) = queue.pop_front() {
        for direction in DIRECTIONS.iter() {
            if let Some(mut node) = iterate(&current.computer, direction, &current, &mut map) {
                if map.get(&(-20, 14)).is_some() {
                    queue.clear();
                    map.clear();
                    node.step = 0;
                    queue.push_back(node);
                    break 'bfs
                }
                queue.push_back(node)
            }
        }

    }

    while let Some(current) = queue.pop_front() {
        for direction in DIRECTIONS.iter() {
            if let Some(node) = iterate(&current.computer, direction, &current, &mut map) {
                queue.push_back(node)
            }
        }

    }

    let max = map.values().max_by(|(_, step1), (_, step2)| step1.cmp(&step2)).unwrap().1;
    println!("part2: {}", max);
}

fn iterate(computer: &Computer, direction: &Dir, current: &Node, map: &mut Map) -> Option<Node> {
    let new_coords = direction.change(current.x, current.y);
    let step = current.step + 1;
    if map.get(&new_coords).is_some() {
        return None
    }

    let a: Dir = *direction;
    let b: i64 = a.into();
    let mut computer = computer.clone();
    computer.input(b);
    computer.run();
    let output = computer.outputs.pop_front().expect("no tile type");
    let tile_type = Type::from(output);
    match tile_type {
        Type::Wall => {
            map.insert(new_coords, (Type::Wall, step));
            None
        }
        t => {
            if t == Type::Oxygen {
                // no idea what is going on this answer is -6 of the real answer???
                dbg!("found oxygen");
                dbg!(step);
            }
            map.insert(new_coords, (t, step));
            Some(Node {
                x: new_coords.0,
                y: new_coords.1,
                computer,
                step,
            })
        }
    }
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

    for ((x, y), (tile_type, _)) in map {
        canvas[(*y + min_y.abs()) as usize][(*x + min_x.abs()) as usize] = match tile_type {
            Type::Wall => '#',
            Type::Empty => '.',
            Type::Oxygen => '@',
            Type::Start => '0',
        };
    }

    for line in canvas {
        println!("{}", line.iter().collect::<String>());
    }
}
