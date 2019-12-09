use itertools::Itertools;
use std::fs;
use std::io;
mod computer;
use computer::Computer;
use computer::State;

fn main() {
    let result = part2();
    println!("part1 {}", result);
}

fn part1() -> i64 {
    let input = fs::read_to_string("input.txt").unwrap();
    let registers = input
        .split(",")
        .map(|x| match x.trim().parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                println!("could not parse {}", x);
                panic!("could not parse");
            }
        })
        .collect::<Vec<_>>();

    let mut comp = Computer::new(registers);
    comp.input(1);
    comp.run();
    dbg!(&comp.outputs);
    comp.outputs.pop_front().expect("no output")
}

fn part2() -> i64 {
    let input = fs::read_to_string("input.txt").unwrap();
    let registers = input
        .split(",")
        .map(|x| match x.trim().parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                println!("could not parse {}", x);
                panic!("could not parse");
            }
        })
        .collect::<Vec<_>>();

    let mut comp = Computer::new(registers);
    comp.input(2);
    comp.run();
    dbg!(&comp.outputs);
    comp.outputs.pop_front().expect("no output")
}

