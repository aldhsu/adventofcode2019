use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result1 = part1(&input);
    println!("part1: asteroid {:?}, count: {}", result1.0, result1.1);

    let result2 = part2(&input, 199, &result1.0);
    println!("part2: {}", result2);
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Asteroid {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Vector2d {
    x: i32,
    y: i32,
    angle: f64,
    distance: usize,
}

impl Vector2d {
    fn new(x: i32, y: i32) -> Self {
        let mut radians = -(y as f64).atan2(x as f64) + std::f64::consts::PI / 2.0;
        radians = if radians >= 0.0 {
            radians
        } else {
            radians + 2.0 * std::f64::consts::PI
        };
        radians = radians % (2.0 * std::f64::consts::PI);
        let angle = radians * (180.0 / std::f64::consts::PI);
        Self {
            x,
            y,
            angle: angle,
            distance: (x.abs() + y.abs()) as usize,
        }
    }

    fn is_obscured(&self, other: &Self) -> bool {
        let on_line = self.x * other.y - other.x * self.y == 0;
        let same_direction = (self.x.is_negative() == other.x.is_negative())
            && (self.y.is_negative() == other.y.is_negative());
        on_line && same_direction
    }
}

impl PartialOrd for Vector2d {
    fn partial_cmp(&self, other: &Vector2d) -> Option<Ordering> {
        Some(match self.angle.partial_cmp(&other.angle)? {
            Ordering::Equal => self.distance.cmp(&other.distance),
            order => order,
        })
    }
}

impl Asteroid {
    fn vector(&self, other: &Self) -> Vector2d {
        let result = Vector2d::new(
            other.x as i32 - self.x as i32,
            self.y as i32 - other.y as i32,
        );
        if other.x == 7 {

            dbg!(&result);
            dbg!(&self.x);
            dbg!(&other.x);
            dbg!(self.x as i32 - other.x as i32);
        }
        result
    }
}

fn part1(input: &str) -> (Asteroid, usize) {
    let asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Asteroid { x, y })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<Asteroid>>();

    let angles = asteroids.iter().enumerate().map(|(i, origin)| {
        let all_angles = asteroids
            .iter()
            .filter_map(move |other| {
                if other == origin {
                    None
                } else {
                    Some(origin.vector(&other))
                }
            })
            .collect::<Vec<_>>();

        let mut uniq: Vec<Vector2d> = Vec::with_capacity(all_angles.len());

        for angle in all_angles {
            let exists = uniq
                .iter()
                .any(|other| other != &angle && angle.is_obscured(&other));

            if !exists {
                uniq.push(angle);
            }
        }

        (origin, uniq.len())
    });

    let max_angles = angles
        .max_by(|(a, a_ngles), (b, b_ngles)| a_ngles.cmp(b_ngles))
        .unwrap();
    // asteroids.len() - max_angles
    let (asteroid, count) = max_angles;
    (*asteroid, count)
}

fn part2(input: &str, nth: usize, origin: &Asteroid) -> usize {
    let asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Asteroid { x, y })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<Asteroid>>();

    let mut all_angles = asteroids
        .iter()
        .filter_map(move |other| {
            if other == origin {
                None
            } else {
                Some((other, origin.vector(&other)))
            }
        })
        .collect::<Vec<_>>();
    all_angles.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("{:?}", all_angles);

    let mut destroyed = all_angles.iter().take(1).collect::<Vec<_>>();
    let mut previous = destroyed.first().unwrap().clone();

    for angle in all_angles.iter().cycle() {
        if destroyed.contains(&angle) {
            continue;
        }
        if previous.1.is_obscured(&angle.1) {
            continue;
        }
        destroyed.push(angle);
        previous = angle;
        if destroyed.len() > nth {
            break;
        }
    }

    dbg!(&destroyed);
    let two_hundredth = destroyed[nth].0;
    two_hundredth.x * 100 + two_hundredth.y
}

#[test]
fn vector2d_detects_same_if_in_line() {
    let a = Vector2d::new(1, 1);
    let b = Vector2d::new(2, 2);
    assert_eq!(a.is_obscured(&b), true);
}

#[test]
fn asteroid_vector() {
    let a = Asteroid { x: 1, y: 5 };
    let b = Asteroid { x: 0, y: 3 };
    let vector = a.vector(&b);
    assert_eq!(vector.x, 1);
    assert_eq!(vector.y, 2);
}

#[test]
fn vector_angle() {
    let a = Vector2d::new(1, 1);
    assert_eq!(a.angle, 45.0);
}

#[test]
fn vectors_in_the_same_angle_can_be_sorted() {
    let a = Vector2d::new(1, 2);
    let b = Vector2d::new(2, 4);
    assert_eq!(a.is_obscured(&b), true);
    let mut v = vec![&b, &a];
    v.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    assert_eq!(v, vec![&a, &b]);
}

#[test]
fn vectors_not_in_the_same_angle_can_be_sorted() {
    let a = Vector2d::new(1, 0);
    let b = Vector2d::new(0, 1);
    let c = Vector2d::new(-1, 0);
    let d = Vector2d::new(-1, -2);
    let e = Vector2d::new(0, -1);
    let mut v = vec![&d, &c, &b, &a, &e];
    v.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    assert_eq!(v, vec![&b, &a, &e, &d, &c]);
}

#[test]
fn part1_example5() {
    let input = fs::read_to_string("example5.txt").unwrap();
    assert_eq!(part1(&input), (Asteroid { x: 11, y: 13 }, 210));
}

#[test]
fn part2_example1() {
    let input = fs::read_to_string("example2_1.txt").unwrap();
    dbg!(part1(&input));
    assert_eq!(part2(&input, 0, &Asteroid { x: 8, y: 3 }), 801);
    assert_eq!(part2(&input, 1, &Asteroid { x: 8, y: 3 }), 900);
}
