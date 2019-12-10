use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result1 = part1(&input);
    println!("part1: {}", result1);
}

#[derive(PartialEq, Eq, Debug)]
struct Asteroid {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Vector2d {
    x: i32,
    y: i32,
}

impl PartialEq for Vector2d {
    fn eq(&self, other: &Self) -> bool {
        self.x * other.y - other.x * self.y == 0
    }
}

impl Asteroid {
    fn vector(&self, other: &Self) -> Vector2d {
        Vector2d { x: self.x as i32 - other.x as i32, y: self.y as i32 - other.y as i32 }
    }
}

fn part1(input: &str) -> usize {
    let asteroids = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move | (x, c)| {
            if c == '#' { Some(Asteroid {x, y}) } else { None }

        })
    }).collect::<Vec<Asteroid>>();

    let angles = asteroids.iter().map(|origin| {
        let mut all_angles = asteroids.iter().filter_map(move |other| {
            if other == origin { None } else { Some(origin.vector(&other)) }
        }).collect::<Vec<_>>();
        all_angles.dedup();
        all_angles.len()
    });

    let max_angles = angles.max().unwrap();
    // dbg!(&max_angles);
    // asteroids.len() - max_angles
    max_angles
}

#[test]
fn vector2d_detects_same_if_in_line() {
    let a = Vector2d { x: 1, y: 1 };
    let b = Vector2d { x: 2, y: 2 };
    assert_eq!(a.eq(&b), true);
}

// fn vector2d_detects_same_if_in_line() {
//     let a = Vector2d { x: 1, y: 1 };
//     let b = Vector2d { x: 2, y: 2 };
//     assert_eq!(a == b, true);
// }

#[test]
fn asteroid_vector() {
    let a = Asteroid { x: 1, y: 5 };
    let b = Asteroid { x: 0, y: 3 };
    let vector = a.vector(&b);
    assert_eq!(vector.x, 1);
    assert_eq!(vector.y, 2);
}

#[test]
fn part1_example1() {
    let input = fs::read_to_string("example1.txt").unwrap();
    assert_eq!(part1(&input), 8);
}
