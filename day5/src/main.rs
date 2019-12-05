use itertools::Itertools;
use std::fs;
use std::io;

fn main() {
    part1()
}

enum Opcode {
    Add,
    Mult,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    EqualTo,
    Halt,
}

impl From<u8> for Opcode {
    fn from(i: u8) -> Self {
        match i {
            1 => Opcode::Add,
            2 => Opcode::Mult,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::EqualTo,
            99 => Opcode::Halt,
            _ => panic!("unknown opcode"),
        }
    }
}

enum Mode {
    Position,
    Immediate,
}

impl From<u32> for Mode {
    fn from(i: u32) -> Mode {
        match i {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => unimplemented!(),
        }
    }
}

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut registers = input
        .split(",")
        .map(|x| match x.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("could not parse {}", x);
                panic!("could not parse");
            }
        })
        .collect::<Vec<_>>();

    run_registers(&mut registers);
}

fn run_registers(registers: &mut Vec<i32>) {
    let mut position = 0;
    loop {
        let machine_code = registers.get(position).unwrap().to_string();

        let mut opmodes = machine_code.chars().rev().skip(2);
        let opcode = Opcode::from(
            machine_code
                .chars()
                .rev()
                .take(2)
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .map(|c| c.to_string())
                .collect::<String>()
                .parse::<u8>()
                .expect("couldn't parse opcode"),
        );
        match opcode {
            Opcode::Add => {
                let num1 = get_register_value(registers, position + 1, opmodes.next());
                let num2 = get_register_value(registers, position + 2, opmodes.next());
                let deposit = get_mut_register_value(registers, position + 3, opmodes.next());
                *deposit = num1 + num2;
                position += 4;
            }
            Opcode::Mult => {
                let num1 = get_register_value(registers, position + 1, opmodes.next());
                let num2 = get_register_value(registers, position + 2, opmodes.next());
                let deposit = get_mut_register_value(registers, position + 3, opmodes.next());
                *deposit = num1 * num2;
                position += 4;
            }
            Opcode::Input => {
                let deposit = get_mut_register_value(registers, position + 1, opmodes.next());
                let mut input = String::new();
                println!("Please enter some input: ");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Did not enter a correct string");
                let input_number = input
                    .trim()
                    .parse::<i32>()
                    .expect("Could not parse input must be i32");
                *deposit = input_number;
                position += 2;
            }
            Opcode::Output => {
                let deposit = get_register_value(registers, position + 1, opmodes.next());
                println!("Output: {}", deposit);
                position += 2;
            }
            Opcode::JumpIfTrue => {
                let num1 = get_register_value(registers, position + 1, opmodes.next());
                let num2 = get_register_value(registers, position + 2, opmodes.next());
                if num1 != 0 {
                    position = num2 as usize;
                } else {
                    position += 3
                }
            }
            Opcode::JumpIfFalse => {
                let num1 = get_register_value(registers, position + 1, opmodes.next());
                let num2 = get_register_value(registers, position + 2, opmodes.next());
                if num1 == 0 {
                    position = num2 as usize;
                } else {
                    position += 3
                }
            }
            Opcode::LessThan => {
                let num1 = get_register_value(registers, position + 1, opmodes.next());
                let num2 = get_register_value(registers, position + 2, opmodes.next());
                let deposit = get_mut_register_value(registers, position + 3, opmodes.next());
                *deposit = if num1 < num2 { 1 } else { 0 };
                position += 4
            }
            Opcode::EqualTo => {
                let num1 = get_register_value(registers, position + 1, opmodes.next());
                let num2 = get_register_value(registers, position + 2, opmodes.next());
                let deposit = get_mut_register_value(registers, position + 3, opmodes.next());
                *deposit = if num1 == num2 { 1 } else { 0 };
                position += 4
            }
            Opcode::Halt => {
                dbg!(registers);
                break;
            }
        }
    }
}

fn get_register_value(registers: &Vec<i32>, index: usize, mode: Option<char>) -> i32 {
    match Mode::from(mode.unwrap_or('0').to_digit(10).unwrap()) {
        Mode::Position => *registers
            .get(
                *registers
                    .get(index)
                    .expect("couldn't reach register positionally") as usize,
            )
            .expect("couldn't reach register value"),
        Mode::Immediate => *registers
            .get(index)
            .expect("couldn't reach register immediately"),
    }
}

fn get_mut_register_value(registers: &mut Vec<i32>, index: usize, _: Option<char>) -> &mut i32 {
    let d_index = *registers
        .get(index)
        .expect("couldn't get deposit register position");
    registers
        .get_mut(d_index as usize)
        .expect("couldn't get deposit register position value")
}

#[test]
fn it_works() {
    let mut registers = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    run_registers(&mut registers);
    assert_eq!(
        registers,
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
}

#[test]
fn it_works2() {
    let mut registers = vec![1, 0, 0, 0, 99];
    run_registers(&mut registers);
    assert_eq!(registers, vec![2, 0, 0, 0, 99]);
}

#[test]
fn it_works_with_modes() {
    let mut registers = vec![1002, 4, 3, 4, 33];
    run_registers(&mut registers);
    assert_eq!(registers, vec![1002, 4, 3, 4, 99]);
}
#[test]
fn it_works_with_negatives() {
    let mut registers = vec![1101, 100, -1, 4, 0];
    run_registers(&mut registers);
    assert_eq!(registers, vec![1101, 100, -1, 4, 99]);
}
