use std::fs;
use itertools::Itertools;

fn main() {
    part2()
}

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut registers = input
        .split(",")
        .map(|x| match x.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("could not parse {}", x);
                panic!("could not parse");
            }
        })
        .collect::<Vec<_>>();
    registers[1] = 12;
    registers[2] = 2;

    run_registers(&mut registers);
}

fn part2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let registers = input
        .split(",")
        .map(|x| match x.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("could not parse {}", x);
                panic!("could not parse");
            }
        })
        .collect::<Vec<_>>();

    for (noun, verb) in (0..100).cartesian_product(0..100) {
        let mut reg_clone = registers.clone(); 
        reg_clone[1] = noun;
        reg_clone[2] = verb;
        let result = run_registers(&mut reg_clone);
        if result == 19690720 {
            println!("noun: {}, verb: {}", noun, verb);
            break
        }
    }
}

fn run_registers(registers: &mut Vec<u32>) -> u32 {
    let mut position = 0;
    loop {
        match *registers.get(position).unwrap() {
            1 => {
                let num1 = get_register_value(registers, position + 1);
                let num2 = get_register_value(registers, position + 2);
                let d_index = *registers
                    .get(position + 3)
                    .expect("couldn't get deposit register");
                let deposit = registers
                    .get_mut(d_index as usize)
                    .expect("couldn't get deposit register");
                *deposit = num1 + num2;
                position += 4;
            }
            2 => {
                let num1 = get_register_value(registers, position + 1);
                let num2 = get_register_value(registers, position + 2);
                let d_index = *registers
                    .get(position + 3)
                    .expect("couldn't get deposit register");
                let deposit = registers
                    .get_mut(d_index as usize)
                    .expect("couldn't get deposit register value");
                *deposit = num1 * num2;
                position += 4;
            }
            99 => {
                break;
            }

            _ => unimplemented!(),
        }
    }

    registers[0]
}

fn get_register_value(registers: &Vec<u32>, index: usize) -> u32 {
    *registers
        .get(*registers.get(index).expect("couldn't reach register") as usize)
        .expect("couldn't reach register value")
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
