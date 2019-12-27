use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::iter;

fn main() {
    // println!("part1: {}", part1("input.txt"));
    println!("part2: {}", part2("input2.txt"));
}

fn part1(input: &str) -> usize {
    let mazes = input_to_mazes(input);
    solve(mazes.first().unwrap())
}

fn solve(maze: &Maze) -> usize {
    let map = maze.key_map();
    let reachable_keys_len = maze.keys.len();

    let mut queue: BinaryHeap<Path> = BinaryHeap::new();
    queue.push(Path::new(maze.starting_position));
    let mut results: Vec<usize> = vec![];
    let mut visited: HashSet<(Point, Vec<char>)> = HashSet::new();

    while let Some(path) = queue.pop() {
        dbg!("{}", path.keys.len());
        if path.keys.len() == reachable_keys_len {
            results.push(path.steps);
            continue;
        }

        let mut keys = path.keys.iter().cloned().collect::<Vec<_>>();
        keys.sort();
        if visited.insert((path.current_position, keys)) {
            let options = map.get(&path.current_position).unwrap();
            for opt in options.iter().filter(|opt| {
                opt.doors.iter().all(|door| path.keys.contains(door))
                    && opt.keys.iter().all(|k| !path.keys.contains(k))
            }) {
                let mut new_path = path.clone();
                new_path.keys.append(&mut opt.keys.clone());
                new_path.steps += opt.steps;
                new_path.current_position = opt.current_position;
                queue.push(new_path);
            }
        }
    }

    *results
        .iter()
        .min_by(|a, b| a.cmp(&b))
        .expect("couldn't find a min")
}

fn part2(input: &str) -> usize {
    let mut mazes = input_to_mazes(input);
    mazes.iter_mut().for_each(|m| m.filter_unreachable());

    mazes
        .iter_mut()
        .map(|maze| {
            solve(maze)
        })
        .sum()
}

type Map = HashMap<(usize, usize), Tile>;

#[derive(Debug, Clone)]
struct Maze {
    map: Map,
    keys: Vec<Tile>,
    doors: Vec<Tile>,
    starting_position: (usize, usize),
}

#[derive(Debug, Clone, PartialEq, Ord, Eq)]
struct Path {
    current_position: Point,
    doors: Vec<char>,
    keys: Vec<char>,
    steps: usize,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        Some(self.steps.cmp(&other.steps).reverse())
    }
}

type Point = (usize, usize);

impl Path {
    fn new(start: Point) -> Self {
        Self {
            current_position: start,
            doors: vec![],
            keys: vec![],
            steps: 0,
        }
    }

    fn apply_move(&mut self, movement: (isize, isize)) -> Point {
        self.current_position.0 = (self.current_position.0 as isize + movement.0) as usize;
        self.current_position.1 = (self.current_position.1 as isize + movement.1) as usize;
        self.steps += 1;
        self.current_position
    }
}

type KeyMap = HashMap<Point, Vec<Path>>;

impl Maze {
    #[rustfmt::skip]
    const COORDINATES: [(isize, isize); 4] = [
                (0, 1),
        (-1, 0),        (1, 0),
                (0, -1)
    ];

    //{
    //  key: Vec<Path { endpoint: (usize, usize), doors: Vec<char>, steps: usize }>
    //
    //}
    //
    //1. Include start
    //2. Iterate over keys
    //3. Iterate over keys where key is not in same pos
    //4. Collect Vec
    //
    //Using key_map
    //1. Start from beginning position
    //2. Find where you can go with your keys (get key, filter out paths for doors you don't have
    //   keys to, filter out keys you already have)
    //3. BFS over the results
    //4. Choose from minimums
    //
    //
    fn filter_unreachable(&mut self) {
        let (kept_keys, removed_keys) = self.keys.iter().partition(|k| {
            if let Tile::Key { x, y, .. } = k {
                if self.bfs(self.starting_position, (*x, *y)).is_some() {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        });
        self.keys = kept_keys;
        removed_keys.iter().for_each(|k| {
            if let Tile::Key { x, y, .. } = k {
                self.map.insert((*x, *y), Tile::Empty);
            };
        });

        let key_chars = self
            .keys
            .iter()
            .map(|k| {
                if let Tile::Key { name, .. } = k {
                    name
                } else {
                    panic!("should only be tiles")
                }
            })
            .collect::<HashSet<_>>();

        let (kept_doors, removed_doors) = self.doors.iter().partition(|d| {
            if let Tile::Door { name, x, y } = d {
                if self.bfs(self.starting_position, (*x, *y)).is_some()
                    && key_chars
                        .get(&name.to_lowercase().next().unwrap())
                        .is_some()
                {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        });

        self.doors = kept_doors;
        removed_doors.iter().for_each(|k| {
            if let Tile::Door { x, y, .. } = k {
                self.map.insert((*x, *y), Tile::Empty);
            };
        });
    }

    fn key_map(&self) -> KeyMap {
        let positions = self
            .keys
            .iter()
            .map(|k| {
                if let Tile::Key { x, y, .. } = k {
                    (*x, *y)
                } else {
                    panic!("should not have nonkey values when building key map")
                }
            })
            .chain(iter::once(self.starting_position))
            .collect::<Vec<_>>();

        let mut map: KeyMap = HashMap::new();

        for position in &positions {
            let mut paths: Vec<Path> = Vec::with_capacity(positions.len());
            match self.map.get(position).unwrap() {
                Tile::Key { .. } | Tile::Entrance => {
                    for other in &positions {
                        if position == other {
                            continue;
                        }
                        paths.push(self.bfs(*position, *other).expect("should never try to get a position it can't reach"))
                    }
                    map.insert(*position, paths);
                }
                _ => {
                    dbg!("{}", self.map.get(position));
                    panic!("should not have none key values when builidng key map with bfs")
                }
            }
        }

        dbg!("map: {}", &map);
        dbg!("keys: {}", &self.keys);
        map
    }

    fn bfs(&self, start: Point, end: Point) -> Option<Path> {
        let mut seen: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Path> = VecDeque::new();
        let mut true_path: Option<Path> = None;

        queue.push_back(Path::new(start));

        while let Some(mut path) = queue.pop_front() {
            if path.current_position == end {
                match self.map.get(&path.current_position).unwrap() {
                    Tile::Key { name, .. } => {
                        path.keys.push(*name);
                    }
                    _ => {}
                }
                true_path = Some(path);
                break;
            }

            for coordinate in Self::COORDINATES.iter() {
                let mut new_path = path.clone();
                let new_pos = new_path.apply_move(*coordinate);

                if seen.insert(new_pos) {
                    match self
                        .map
                        .get(&new_pos)
                        .expect("couldn't get new position from map")
                    {
                        Tile::Wall => {}
                        Tile::Entrance | Tile::Empty | Tile::Key { .. } => {
                            queue.push_back(new_path);
                        }
                        Tile::Door { name, .. } => {
                            new_path.doors.push(name.to_lowercase().next().unwrap());
                            queue.push_back(new_path);
                        }
                    }
                }
            }
        }

        true_path
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Entrance,
    Wall,
    Empty,
    Key { name: char, x: usize, y: usize },
    Door { name: char, x: usize, y: usize },
}

impl Tile {
    fn from_char(c: char, x: usize, y: usize) -> Self {
        match c {
            '@' => Tile::Entrance,
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            door if door.is_uppercase() => Tile::Door { name: door, x, y },
            key if key.is_lowercase() => Tile::Key { name: key, x, y },
            _ => unimplemented!(),
        }
    }
}

fn input_to_mazes(input: &str) -> Vec<Maze> {
    let text = fs::read_to_string(input).expect("couldn't get input");
    let mut map: Map = HashMap::new();
    let mut starting_positions = vec![];
    let mut keys = vec![];
    let mut doors = vec![];

    for (y, line) in text.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = Tile::from_char(c, x, y);
            map.insert((x, y), tile.clone());
            match tile {
                Tile::Entrance => starting_positions.push((x, y)),
                key @ Tile::Key { .. } => keys.push(key),
                door @ Tile::Door { .. } => doors.push(door),
                _ => {}
            }
        }
    }

    starting_positions
        .iter()
        .map(|&starting_position| Maze {
            starting_position,
            map: map.clone(),
            keys: keys.clone(),
            doors: doors.clone(),
        })
        .collect()
}

#[test]
fn part1_example1() {
    assert_eq!(part1("example1.txt"), 86);
}

#[test]
fn part1_example4() {
    assert_eq!(part1("example4.txt"), 81);
}
