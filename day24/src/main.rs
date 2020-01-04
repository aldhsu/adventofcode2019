use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut map = input_to_map("input.txt");
    let mut seen = HashSet::new();

    while seen.insert(map.as_u32()) {
        map.next();
    }

    println!("part1 {}", map.as_u32());
}

fn part2() {
    let mut rec_map = RecMap::new(input_to_map("input.txt"));
    for _ in 0..200 {
        rec_map.next()
    }
    println!("part 2 {}", rec_map.count_bugs());
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point(usize, usize);

struct RecMap {
    maps: VecDeque<Map>,
}

impl Point {
    const COORDINATES: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    fn apply_coordinates(&self) -> Vec<Self> {
        Self::COORDINATES
            .iter()
            .map(|(x, y)| {
                Self(
                    (self.0 as isize + x) as usize,
                    (self.1 as isize + y) as usize,
                )
            })
            .collect()
    }

    fn apply_coords_unchecked(&self) -> Vec<(isize, isize)> {
        Self::COORDINATES
            .iter()
            .map(|(x, y)| {
                ((self.0 as isize + x), (self.1 as isize + y))
            })
            .collect()
    }
}

impl RecMap {
    fn new(map: Map) -> Self {
        let mut maps = VecDeque::new();
        maps.push_front(map);
        Self { maps }
    }

    fn next(&mut self) {
        self.maps.push_front(Map::new());
        self.maps.push_back(Map::new());

        self.maps = (0..self.maps.len())
            .map(|i| {
                let up = if let Some(index) = i.checked_sub(1) {
                    self.maps.get(index)
                } else {
                    None
                };
                let down = self.maps.get(i + 1);
                let current = self.maps.get(i).unwrap();

                let result = current.rec_next(up, down);
                result
            })
            .collect::<VecDeque<_>>();
    }

    fn count_bugs(&self) -> usize {
        for map in &self.maps {
            println!("{}", map);
            println!("\n");
        }
        let mut total = 0;

        for map in self.maps.iter() {
            total += map.board.iter().flatten().filter(|&&tile| tile).count();
        }

        total
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Map {
    board: [[bool; 5]; 5],
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = self
            .board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&tile| if tile { "#" } else { "." })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", string)
    }
}

impl Map {
    fn new() -> Self {
        Self {
            board: [[false; 5]; 5],
        }
    }

    fn insert(&mut self, point: Point, has_bugs: bool) {
        self.board[point.1][point.0] = has_bugs;
    }

    fn next(&mut self) {
        let mut neighbour_count = [[0; 5]; 5];

        for (y, row) in self.board.iter().enumerate() {
            for (x, has_bugs) in row.iter().enumerate() {
                if !has_bugs {
                    continue;
                }
                Point(x, y).apply_coordinates().iter().for_each(|&point| {
                    if let Some(mut val) = try_get_mut(&mut neighbour_count, point) {
                        *val += 1;
                    }
                });
            }
        }

        for y in 0..5 {
            for x in 0..5 {
                let current_pos = Point(x, y);
                let bug_count = neighbour_count[y][x];
                let has_bugs = self.board[y][x];
                if has_bugs {
                    if bug_count != 1 {
                        self.insert(current_pos, false);
                    }
                } else {
                    if bug_count == 1 || bug_count == 2 {
                        self.insert(current_pos, true);
                    }
                }
            }
        }
    }
    const CENTER: Point = Point(2, 2);

    fn rec_next(&self, up: Option<&Map>, down: Option<&Map>) -> Self {
        let mut new_map = self.clone().to_owned();

        for y in 0..5 {
            for x in 0..5 {
                let current_pos = Point(x, y);

                if current_pos == Self::CENTER {
                    continue;
                }

                let bug_count: usize = Point(x, y)
                    .apply_coords_unchecked()
                    .iter()
                    .map(|(x, y)| {
                        self.count_recursive(&current_pos, *x, *y, up, down)
                    })
                    .sum();

                if let Some(true) = try_get(&self.board, current_pos) {
                    if bug_count != 1 {
                        new_map.insert(current_pos, false);
                    }
                } else {
                    if bug_count == 1 || bug_count == 2 {
                        new_map.insert(current_pos, true);
                    }
                }
            }
        }

        new_map
    }

    fn count_recursive(&self, current_pos: &Point, x: isize, y: isize, up: Option<&Map>, down: Option<&Map>) -> usize {
        let point = Point(x as usize, y as usize);
        // check if middle
        if  point == Self::CENTER {
            // check all side it's on
            if let Some(down_map) = down {
                return match current_pos {
                    Point(1, 2) => {
                        down_map.board.iter().filter(|row| row[0]).count()
                    }
                    Point(3, 2) => {
                        down_map.board.iter().filter(|row| row[4]).count()
                    }
                    Point(2, 1) => {
                        down_map.board[0].iter().filter(|&&item| item).count()
                    }
                    Point(2, 3) => {
                        down_map.board[4].iter().filter(|&&item| item).count()
                    }
                    _ => {
                        println!("{:?}", current_pos);
                        unreachable!()
                    }
                };
            } else {
                return 0;
            }
        } else {
            // 1 if Some
            match try_get(&self.board, point) {
                Some(true) => 1,
                Some(false) => 0,
                None => {
                    // Check up if none
                    if let Some(map) = up {
                        let board = map.board;
                        match (x, y) {
                            (x, _) if x < 0 => {
                                board[2][1] as usize
                            }
                            (x, _) if x > 4 => {
                                board[2][3] as usize
                            }
                            (_, y) if y < 0 => {
                                board[1][2] as usize
                            }
                            (_, y) if y > 4 => {
                                board[3][2] as usize
                            }
                            _ => {
                                println!("x: {}, y: {}", x, y);
                                unreachable!()
                            }
                        }
                    } else {
                        0
                    }
                }
            }
        }

    }

    fn as_u32(&self) -> u32 {
        self.board
            .iter()
            .flatten()
            .enumerate()
            .fold(0, |mut num, (i, tile)| {
                if *tile {
                    num |= (1 << i);
                }
                num
            })
    }
}

fn try_get<T: Copy>(array: &[[T; 5]; 5], point: Point) -> Option<T> {
    Some(*array.get(point.1)?.get(point.0)?)
}

fn try_get_mut<T: Copy>(array: &mut [[T; 5]; 5], point: Point) -> Option<&mut T> {
    Some(&mut *array.get_mut(point.1)?.get_mut(point.0)?)
}

fn input_to_map(filename: &str) -> Map {
    let input = fs::read_to_string(filename).expect("could not read input");
    string_to_map(&input)
}

fn string_to_map(input: &str) -> Map {
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let has_bugs = if c == '#' { true } else { false };
            map.insert(Point(x, y), has_bugs);
        }
    }
    map
}

#[test]
fn part1_example1() {
    let mut map = input_to_map("part1_example1.txt");
    map.next();
    let one_min = string_to_map(
        "#..#.
####.
###.#
##.##
.##..",
    );
    assert_eq!(map, one_min);
    let two_min = string_to_map(
        "#####
....#
....#
...#.
#.###",
    );
    map.next();
    assert_eq!(map, two_min);
}

#[test]
fn it_can_be_converted_to_u32() {
    let mut map = input_to_map("part1_example1.txt");
    map.next();
    assert_eq!(map.as_u32(), 7200233);

    map.next();
    assert_eq!(map.as_u32(), 30687775);
}

#[test]
fn it_works_recursively() {
    let mut rec_map = RecMap::new(input_to_map("part1_example1.txt"));
    for i in 0..10 { rec_map.next() }
    assert_eq!(rec_map.count_bugs(), 99);
}
