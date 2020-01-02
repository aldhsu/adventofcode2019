#![feature(is_sorted)]
use std::collections::VecDeque;
use std::fs;
use modinverse::modinverse;

fn main() {
    // println!("part1 {}", part1());
    part2();
}

fn part1() -> i32 {
    let instructions = input_to_instructions("input.txt");
    let deck = (0..10007).collect::<Vec<i32>>();
    instructions.iter().fold(deck, |cards, instruction| {
        instruction.apply(cards)
    }).iter().enumerate().find_map(|(i, num)| {
        if *num == 2019 {
            Some(i as i32)
        } else {
            None
        }
    }).expect("couldn't find card")
}

fn part2() {
}

enum Instruction {
    NewStack,
    Increment(usize),
    Cut(isize),
}

impl Instruction {
    fn apply(&self, mut deck: Vec<i32>) -> Vec<i32> {
        use Instruction::*;

        match self {
            NewStack => deck.into_iter().rev().collect(),
            Increment(n) => {
                let mut iter = deck.iter();
                let mut result = vec![0; deck.len()];

                for count in 0..deck.len() {
                    let replace_at = (count * n) % deck.len();
                    result[replace_at] = *iter.next().unwrap();
                }

                result
            }
            Cut(n) => {
                if n.is_positive() {
                    let mut back = deck.split_off(*n as usize);
                    back.append(&mut deck);
                    back
                } else {
                    let index = deck.len() as isize + n;

                    let mut back = deck.split_off(index as usize);
                    back.append(&mut deck);
                    back
                }
            }
        }
    }
}

fn input_to_instructions(filename: &str) -> Vec<Instruction> {
    use Instruction::*;
    let input = fs::read_to_string(filename).expect("couldn't read file");
    input
        .lines()
        .map(|line| {
            if line.starts_with("deal with increment") {
                let number = parse_num(line);
                return Increment(number as usize);
            }

            if line.starts_with("cut") {
                let number = parse_num(line);
                return Cut(number as isize);
            }

            if line.starts_with("deal into new stack") {
                return NewStack;
            }

            panic!("fell through")
        })
        .collect()
}

fn parse_num(input: &str) -> i32 {
    input
        .chars()
        .filter(|c| c.is_digit(10) || *c == '-')
        .collect::<String>()
        .parse()
        .expect("couldn't turn into number")
}

#[test]
fn instruction_cut_works() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(
        Instruction::Cut(3).apply(vec),
        vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]
    );
}

#[test]
fn instruction_new_deck_works() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(
        Instruction::NewStack.apply(vec),
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    );
}

#[test]
fn instruction_increment_works() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(
        Instruction::Increment(3).apply(vec),
        vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
    );
}
