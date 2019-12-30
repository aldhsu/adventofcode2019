use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    // println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let maze = input_to_maze("input.txt");
    maze.find_shortest_path()
}

fn part2() -> usize {
    let maze = input_to_maze2("input.txt");
    maze.find_shortest_path2()
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point(usize, usize);

const COORDINATES: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

impl Point {
    fn apply_coordinates(&self) -> Vec<Self> {
        COORDINATES
            .iter()
            .map(|(x, y)| {
                Self(
                    (self.0 as isize + x) as usize,
                    (self.1 as isize + y) as usize,
                )
            })
            .collect()
    }
}
type Map = HashMap<Point, Tile>;
struct Maze {
    map: Map,
    start: Point,
    end: Point,
}

impl Maze {
    fn find_shortest_path(&self) -> usize {
        let mut seen = HashSet::new();
        seen.insert(self.start);
        let mut queue = VecDeque::new();
        queue.push_back((0, self.start));

        while let Some((steps, point)) = queue.pop_front() {
            if point == self.end {
                return steps
            }
            let new_points = point.apply_coordinates();

            for new_point in new_points.iter() {
                match self.map.get(new_point) {
                    Some(Tile::Empty) | Some(Tile::Teleport(..)) => {
                        if seen.insert(*new_point) {
                            queue.push_back((steps + 1, *new_point))
                        }
                    },
                    _ => {}
                }
            }

            if let Some(Tile::Teleport(tp)) = self.map.get(&point) {
                if seen.insert(*tp) {
                    queue.push_back((steps + 1, *tp))
                }
            }

        }
        panic!("Couldn't find a path");
    }

    fn find_shortest_path2(&self) -> usize {
        let mut seen = HashSet::new();
        seen.insert((self.start, 0));
        let mut queue = VecDeque::new();
        queue.push_back((0, self.start, 0));

        while let Some((steps, point, level)) = queue.pop_front() {
            if point == self.end && level == 0 {
                return steps
            }
            let new_points = point.apply_coordinates();

            for new_point in new_points.iter() {
                match self.map.get(new_point) {
                    Some(Tile::Empty) | Some(Tile::RecTp(..)) => {
                        if seen.insert((*new_point, level)) {
                            queue.push_back((steps + 1, *new_point, level))
                        }
                    },
                    _ => {}
                }
            }

            if let Some(Tile::RecTp(tp, inc_level)) = self.map.get(&point) {
                let new_level = level + inc_level;
                if  new_level < 0 { continue }

                if seen.insert((*tp, new_level)) {
                    queue.push_back((steps + 1, *tp, new_level))
                }
            }

        }

        panic!("Couldn't find a path");
    }
}

enum Tile {
    Wall,
    Empty,
    Teleport(Point),
    RecTp(Point, isize),
}

fn input_to_maze(filename: &str) -> Maze {
    // 1. parse into IR
    let input = fs::read_to_string(filename).expect("couldn't read file");

    let intermediate_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut labels: HashMap<String, Vec<Point>> = HashMap::new();

    let mut map: HashMap<Point, Tile> = HashMap::new();
    for (y, line) in intermediate_map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                '#' => {
                    map.insert(Point(x, y), Tile::Wall);
                }
                '.' => {
                    let current_position = Point(x, y);
                    current_position
                        .apply_coordinates()
                        .iter()
                        .enumerate()
                        .for_each(
                            |(direction, point)| match intermediate_map[point.1][point.0] {
                                first_char if first_char.is_alphabetic() => {
                                    let second_coord = point.apply_coordinates()[direction];
                                    let second_char =
                                        intermediate_map[second_coord.1][second_coord.0];
                                    let mut label = vec![first_char, second_char];
                                    label.sort();
                                    let entry =
                                        labels.entry(label.iter().collect()).or_insert(vec![]);
                                    entry.push(current_position);
                                }
                                _ => {}
                            },
                        );
                    map.insert(Point(x, y), Tile::Empty);
                }
                _ => {}
            }
        }
    }

    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    for (name, points) in labels {
        if name == "AA" {
            start = Some(points[0]);
            continue;
        }

        if name == "ZZ" {
            end = Some(points[0]);
            continue;
        }

        assert!(points.len() == 2);

        let mut iter = points.iter();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();

        map.insert(*first, Tile::Teleport(*second));
        map.insert(*second, Tile::Teleport(*first));
    }

    Maze {
        map,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn input_to_maze2(filename: &str) -> Maze {
    // 1. parse into IR
    let input = fs::read_to_string(filename).expect("couldn't read file");

    let intermediate_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut labels: HashMap<String, Vec<Point>> = HashMap::new();

    let mut map: HashMap<Point, Tile> = HashMap::new();
    for (y, line) in intermediate_map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                '#' => {
                    map.insert(Point(x, y), Tile::Wall);
                }
                '.' => {
                    let current_position = Point(x, y);
                    current_position
                        .apply_coordinates()
                        .iter()
                        .enumerate()
                        .for_each(
                            |(direction, point)| match intermediate_map[point.1][point.0] {
                                first_char if first_char.is_alphabetic() => {
                                    let second_coord = point.apply_coordinates()[direction];
                                    let second_char =
                                        intermediate_map[second_coord.1][second_coord.0];
                                    let mut label = vec![first_char, second_char];
                                    label.sort();
                                    let entry =
                                        labels.entry(label.iter().collect()).or_insert(vec![]);
                                    entry.push(current_position);
                                }
                                _ => {}
                            },
                        );
                    map.insert(Point(x, y), Tile::Empty);
                }
                _ => {}
            }
        }
    }

    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    let center :(isize, isize) = (input.lines().next().unwrap().chars().count() as isize / 2, input.lines().count() as isize / 2);
    for (name, mut points) in labels {
        if name == "AA" {
            start = Some(points[0]);
            continue;
        }

        if name == "ZZ" {
            end = Some(points[0]);
            continue;
        }

        assert!(points.len() == 2);

        points.sort_by(|a, b| {
            let a_dist = (a.1 as isize - center.1).abs().pow(2) + (a.0 as isize - center.0).abs().pow(2);
            let b_dist = (b.1 as isize - center.1).abs().pow(2) + (b.0 as isize - center.0).abs().pow(2);
            a_dist.cmp(&b_dist)
        });
        let mut iter = points.iter();
        let down = iter.next().unwrap();
        let up = iter.next().unwrap();

        map.insert(*up, Tile::RecTp(*down, -1));
        map.insert(*down, Tile::RecTp(*up, 1));
    }

    Maze {
        map,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

#[test]
fn part2_example2() {
    let maze = input_to_maze2("part2_example2.txt");
    assert_eq!(maze.find_shortest_path2(), 396)
}
