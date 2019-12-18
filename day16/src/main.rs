use ndarray::arr1;
use std::fs;

struct Pattern {
    position: usize,
}

impl Pattern {
    const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

    fn new(position: usize) -> Self {
        Self { position }
    }

    fn to_vec(&self, len: usize) -> Vec<isize> {
        Self::BASE_PATTERN
            .iter()
            .fold(vec![], |mut acc, item| {
                for _ in 0..=self.position {
                    acc.push(*item);
                }
                acc
            })
            .into_iter()
            .cycle()
            .skip(1)
            .take(len)
            .collect::<Vec<_>>()
    }
}

fn main() {
    part2();
}

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = iterate_phases(&create_input(&input));
    println!(
        "part1: {:?}",
        result[..8]
            .iter()
            .map(|d| d.to_string())
            .collect::<String>()
    );
}

fn part2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = iterate_phases(&create_input(&input));
    println!(
        "part1: {:?}",
        result[..8]
            .iter()
            .map(|d| d.to_string())
            .collect::<String>()
    );
}

fn create_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect::<Vec<isize>>()
}

fn next_phase(input: &Vec<isize>, patterns: &Vec<Vec<isize>>) -> Vec<isize> {
    let output = arr1(input);
    (0..input.len())
        .map(|i| {
            let coef = arr1(&patterns[i]);
            let result = output.dot(&coef);
            (result % 10).abs()
        })
        .collect()
}

fn iterate_phases(input: &Vec<isize>) -> Vec<isize> {
    let patterns = (0..input.len())
        .map(|i| Pattern::new(i).to_vec(input.len()))
        .collect::<Vec<_>>();
    (0..100).fold(input.clone(), |previous, i| {
        println!("iteration {}", i);
        next_phase(&previous, &patterns)
    })
}

#[test]
fn it_creates_patterns_correctly0() {
    let pattern = Pattern::new(0);
    assert_eq!(pattern.to_vec(4), vec![1, 0, -1, 0]);
}

#[test]
fn it_creates_patterns_correctly1() {
    let pattern = Pattern::new(1);
    assert_eq!(pattern.to_vec(8), vec![0, 1, 1, 0, 0, -1, -1, 0]);
}

#[test]
fn it_creates_patterns_correctly2() {
    let pattern = Pattern::new(2);
    assert_eq!(
        pattern.to_vec(12),
        vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0]
    );
}

#[test]
fn it_creates_patterns_correctly10000() {
    let pattern = Pattern::new(10000);
    assert_eq!(
        pattern.to_vec(10000),
        vec![0; 10000]
    );
}

#[test]
fn it_creates_patterns_correctly650() {
    let pattern = Pattern::new(650);
    assert_eq!(
        pattern.to_vec(650),
        vec![0; 650]
    );
}

#[test]
fn it_creates_one_phase_correctly1() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let patterns = (0..input.len())
        .map(|i| Pattern::new(i).to_vec(input.len()))
        .collect::<Vec<_>>();
    assert_eq!(next_phase(&input, &patterns), vec![4, 8, 2, 2, 6, 1, 5, 8],)
}

#[test]
fn it_creates_the_correct_phases0() {
    let input = create_input("80871224585914546619083218645595");
    let expected: Vec<isize> = vec![2, 4, 1, 7, 6, 1, 7, 6];
    assert_eq!(iterate_phases(&input)[..8], expected[..8]);
}

#[test]
fn ndarray_works() {
    let a = arr1(&[1, 2, 3, 4]);
    let b = arr1(&[1, 1, 1, 1]);
    assert_eq!(a.dot(&b), 10);
}
