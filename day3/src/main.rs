use std::collections::HashMap;
use std::fs;

#[derive(Hash, Clone, PartialEq, Eq, Copy, Debug)]
struct Tile {
    x: i16,
    y: i16,
}

impl Tile {
    pub fn manhattan_distance(&self) -> i16 {
        self.x.abs() + self.y.abs()
    }
}

type Map = HashMap<Tile, [Option<usize>; 2]>;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = part2(input);
    println!("result1: {:?}", result);
}

fn part1(input: String) -> i16 {
    let map = create_map(input);
    let result = map
        .iter()
        .filter(|(_, steps)| match steps {
            [Some(_), Some(_)] => true,
            _ => false,
        })
        .min_by(|(a, _), (b, _)| a.manhattan_distance().cmp(&b.manhattan_distance()));

    println!("result1: {:?}", result);
    result.unwrap().0.manhattan_distance()
}

fn part2(input: String) -> usize {
    let map = create_map(input);
    let result = map
        .iter()
        .filter(|(_, steps)| match steps {
            [Some(_), Some(_)] => true,
            _ => false,
        })
        .min_by(|(_, stepsa), (_, stepsb)| match (stepsa, stepsb) {
            ([Some(a1), Some(a2)], [Some(b1), Some(b2)]) => (a1 + a2).cmp(&(b1 + b2)),
            _ => panic!("something went wrong with the filtering"),
        });

    if let [Some(a), Some(b)] = result.unwrap().1 {
        let val = a + b;
        println!("result1: {:?}, a: {}, b: {}", val, a, b);
        val
    } else {
        panic!("no result");
    }
}

fn create_map(input: String) -> Map {
    let mut lines = input.lines();
    let mut map: Map = HashMap::new();

    lines
        .next()
        .unwrap()
        .split(",")
        .fold((Tile { x: 0, y: 0 }, 0), |starting_pos, movement| {
            do_move(starting_pos, movement, &mut map, 0)
        });

    lines
        .next()
        .unwrap()
        .split(",")
        .fold((Tile { x: 0, y: 0 }, 0), |starting_pos, movement| {
            do_move(starting_pos, movement, &mut map, 1)
        });

    map
}

fn do_order(
    starting_position: (Tile, usize),
    number: u16,
    map: &mut Map,
    f: &dyn Fn(&mut Tile),
    iteration: usize,
) -> (Tile, usize) {
    let (mut current_position, mut step_no) = starting_position;
    for _ in 0..number {
        step_no += 1;
        let mut new_position = current_position.clone();
        f(&mut new_position);
        let entry = map.entry(new_position).or_insert([None, None]);
        entry[iteration] = Some(step_no);
        current_position = new_position;
    }

    (current_position, step_no)
}

fn do_move(
    starting_position: (Tile, usize),
    movement: &str,
    mut map: &mut Map,
    iteration: usize,
) -> (Tile, usize) {
    let mut iter = movement.chars();
    let direction = iter.next().unwrap();
    let number = iter.collect::<String>().parse::<u16>().unwrap();

    match direction {
        'U' => do_order(
            starting_position,
            number,
            &mut map,
            &|tile: &mut Tile| tile.y += 1,
            iteration,
        ),
        'R' => do_order(
            starting_position,
            number,
            &mut map,
            &|tile: &mut Tile| tile.x += 1,
            iteration,
        ),
        'D' => do_order(
            starting_position,
            number,
            &mut map,
            &|tile: &mut Tile| tile.y -= 1,
            iteration,
        ),
        'L' => do_order(
            starting_position,
            number,
            &mut map,
            &|tile: &mut Tile| tile.x -= 1,
            iteration,
        ),
        _ => unimplemented!(),
    }
}

#[test]
fn part1_example1() {
    let input = "R8,U5,L5,D3
U7,R6,D4,L4
"
    .to_string();
    assert_eq!(part1(input), 6);
}

#[test]
fn part2_example1() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
"
    .to_string();
    assert_eq!(part2(input), 610);
}

#[test]
fn part2_example2() {
    let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        .to_string();
    assert_eq!(part2(input), 410);
}
