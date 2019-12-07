use itertools::Itertools;
use std::fs;
use std::io;

fn main() {
    part1();
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

    run_amplifier(&registers)
}

fn run_amplifier(registers: &Vec<i64>) -> i64 {
    let mut max = 0;
    let mut max_seq = vec![];

    for phases in (0..=4).permutations(5) {
        let mut last_result = 0;
        for phase in &phases {
            let mut copy = registers.clone();
            last_result = run_registers(&mut copy, Some(vec![last_result, *phase]));
        }

        if last_result > max {
            max = last_result;
            max_seq = phases;
            println!("new max: {}", max);
            println!("new max_seq: {:?}", max_seq);
        }
    };

    println!("Max is {}", max);
    max
}

fn run_registers(registers: &mut Vec<i64>, inputs: Option<Vec<i64>>) -> i64 {
    let mut output = vec![];
    let mut input_vec = match inputs {
        Some(vec) => vec,
        None => vec![],
    };
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
        let mut pos = position + 1..;
        match opcode {
            Opcode::Add => {
                let num1 = get_register_value(registers, pos.next(), opmodes.next());
                let num2 = get_register_value(registers, pos.next(), opmodes.next());
                let deposit = get_mut_register_value(registers, pos.next());
                *deposit = num1 + num2;
                position = pos.next().unwrap();
            }
            Opcode::Mult => {
                let num1 = get_register_value(registers, pos.next(), opmodes.next());
                let num2 = get_register_value(registers, pos.next(), opmodes.next());
                let deposit = get_mut_register_value(registers, pos.next());
                *deposit = num1 * num2;
                position = pos.next().unwrap();
            }
            Opcode::Input => {
                let deposit = get_mut_register_value(registers, pos.next());
                if let Some(input) = input_vec.pop() {
                    *deposit = input;
                    position = pos.next().unwrap();
                } else {
                    panic!("No inputs given");
                }
            }
            Opcode::Output => {
                let deposit = get_register_value(registers, pos.next(), opmodes.next());
                output.push(deposit);
                position = pos.next().unwrap();
            }
            Opcode::JumpIfTrue => {
                let num1 = get_register_value(registers, pos.next(), opmodes.next());
                let num2 = get_register_value(registers, pos.next(), opmodes.next());
                if num1 != 0 {
                    position = num2 as usize;
                } else {
                    position = pos.next().unwrap()
                }
            }
            Opcode::JumpIfFalse => {
                let num1 = get_register_value(registers, pos.next(), opmodes.next());
                let num2 = get_register_value(registers, pos.next(), opmodes.next());
                if num1 == 0 {
                    position = num2 as usize;
                } else {
                    position = pos.next().unwrap()
                }
            }
            Opcode::LessThan => {
                let num1 = get_register_value(registers, pos.next(), opmodes.next());
                let num2 = get_register_value(registers, pos.next(), opmodes.next());
                let deposit = get_mut_register_value(registers, pos.next());
                *deposit = if num1 < num2 { 1 } else { 0 };
                position = pos.next().unwrap()
            }
            Opcode::EqualTo => {
                let num1 = get_register_value(registers, pos.next(), opmodes.next());
                let num2 = get_register_value(registers, pos.next(), opmodes.next());
                let deposit = get_mut_register_value(registers, pos.next());
                *deposit = if num1 == num2 { 1 } else { 0 };
                position = pos.next().unwrap()
            }
            Opcode::Halt => {
                break;
            }
        }
    }

    if output.len() > 1 { panic!("output count was more than one. {:?}", output) }
    *output.last().unwrap_or(&0)
}

fn get_register_value(registers: &Vec<i64>, index: Option<usize>, mode: Option<char>) -> i64 {
    match Mode::from(mode.unwrap_or('0').to_digit(10).unwrap()) {
        Mode::Position => *registers
            .get(
                *registers
                    .get(index.unwrap())
                    .expect("couldn't reach register positionally") as usize,
            )
            .expect("couldn't reach register value"),
        Mode::Immediate => *registers
            .get(index.unwrap())
            .expect("couldn't reach register immediately"),
    }
}

fn get_mut_register_value(registers: &mut Vec<i64>, index: Option<usize>) -> &mut i64 {
    let d_index = *registers
        .get(index.unwrap())
        .expect("couldn't get deposit register position");
    registers
        .get_mut(d_index as usize)
        .expect("couldn't get deposit register position value")
}

#[test]
fn it_works() {
    let mut registers = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    run_registers(&mut registers, None);
    assert_eq!(
        registers,
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
}

#[test]
fn it_works2() {
    let mut registers = vec![1, 0, 0, 0, 99];
    run_registers(&mut registers, None);
    assert_eq!(registers, vec![2, 0, 0, 0, 99]);
}

#[test]
fn it_works_with_modes() {
    let mut registers = vec![1002, 4, 3, 4, 33];
    run_registers(&mut registers, None);
    assert_eq!(registers, vec![1002, 4, 3, 4, 99]);
}

#[test]
fn it_works_with_negatives() {
    let mut registers = vec![1101, 100, -1, 4, 0];
    run_registers(&mut registers, None);
    assert_eq!(registers, vec![1101, 100, -1, 4, 99]);
}

#[test]
fn it_works_out_the_max_thrust_signal() {
    let mut registers = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
    assert_eq!(run_amplifier(&registers), 43210);
}

#[test]
fn it_works_out_the_max_thrust_signal_example2() {
    let mut registers = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
    assert_eq!(run_amplifier(&registers), 54321);
}

#[test]
fn it_works_out_the_max_thrust_signal_example3() {
    let mut registers = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
    assert_eq!(run_amplifier(&registers), 65210);
}
