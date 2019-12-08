use std::collections::VecDeque;

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


pub enum State {
    Operating,
    Halted,
    Waiting,
}

pub struct Computer {
    position: usize,
    inputs: VecDeque<i64>,
    pub outputs: VecDeque<i64>,
    pub registers: Vec<i64>,
    pub state: State,
}

impl Computer {
    pub fn new(registers: Vec<i64>) -> Self {
        Self {
            registers,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            position: 0,
            state: State::Operating,
        }
    }

    pub fn run(&mut self) {
        loop {
            let machine_code = self.registers.get(self.position).unwrap().to_string();

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
            let mut pos = self.position + 1..;
            match opcode {
                Opcode::Add => {
                    let num1 = self.get_register_value(pos.next(), opmodes.next());
                    let num2 = self.get_register_value(pos.next(), opmodes.next());
                    let deposit = self.get_mut_register_value(pos.next());
                    *deposit = num1 + num2;
                    self.position = pos.next().unwrap();
                }
                Opcode::Mult => {
                    let num1 = self.get_register_value(pos.next(), opmodes.next());
                    let num2 = self.get_register_value(pos.next(), opmodes.next());
                    let deposit = self.get_mut_register_value(pos.next());
                    *deposit = num1 * num2;
                    self.position = pos.next().unwrap();
                }
                Opcode::Input => {
                    if let Some(input) = self.inputs.pop_front() {
                        let deposit = self.get_mut_register_value(pos.next());
                        *deposit = input;
                        self.position = pos.next().unwrap();
                    } else {
                        self.state = State::Waiting;
                        break
                    }
                }
                Opcode::Output => {
                    let deposit = self.get_register_value(pos.next(), opmodes.next());
                    self.outputs.push_back(deposit);
                    self.position = pos.next().unwrap();
                }
                Opcode::JumpIfTrue => {
                    let num1 = self.get_register_value(pos.next(), opmodes.next());
                    let num2 = self.get_register_value(pos.next(), opmodes.next());
                    if num1 != 0 {
                        self.position = num2 as usize;
                    } else {
                        self.position = pos.next().unwrap()
                    }
                }
                Opcode::JumpIfFalse => {
                    let num1 = self.get_register_value(pos.next(), opmodes.next());
                    let num2 = self.get_register_value(pos.next(), opmodes.next());
                    if num1 == 0 {
                        self.position = num2 as usize;
                    } else {
                        self.position = pos.next().unwrap()
                    }
                }
                Opcode::LessThan => {
                    let num1 = self.get_register_value(pos.next(), opmodes.next());
                    let num2 = self.get_register_value(pos.next(), opmodes.next());
                    let deposit = self.get_mut_register_value(pos.next());
                    *deposit = if num1 < num2 { 1 } else { 0 };
                    self.position = pos.next().unwrap()
                }
                Opcode::EqualTo => {
                    let num1 = self.get_register_value(pos.next(), opmodes.next());
                    let num2 = self.get_register_value(pos.next(), opmodes.next());
                    let deposit = self.get_mut_register_value(pos.next());
                    *deposit = if num1 == num2 { 1 } else { 0 };
                    self.position = pos.next().unwrap()
                }
                Opcode::Halt => {
                    self.state = State::Halted;
                    break;
                }
            }
        }
    }
    fn get_register_value(&self, index: Option<usize>, mode: Option<char>) -> i64 {
        match Mode::from(mode.unwrap_or('0').to_digit(10).unwrap()) {
            Mode::Position => *self
                .registers
                .get(
                    *self
                        .registers
                        .get(index.unwrap())
                        .expect("couldn't reach register positionally")
                        as usize,
                )
                .expect("couldn't reach register value"),
            Mode::Immediate => *self
                .registers
                .get(index.unwrap())
                .expect("couldn't reach register immediately"),
        }
    }

    fn get_mut_register_value(&mut self, index: Option<usize>) -> &mut i64 {
        let d_index = *self
            .registers
            .get(index.unwrap())
            .expect("couldn't get deposit register position");
        self.registers
            .get_mut(d_index as usize)
            .expect("couldn't get deposit register position value")
    }

    pub fn input(&mut self, input: i64) {
        self.inputs.push_back(input);
    }
}

#[test]
fn it_works() {
    let mut comp = Computer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    comp.run();
    assert_eq!(comp.registers, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
}

#[test]
fn it_works2() {
    let mut comp = Computer::new(vec![1, 0, 0, 0, 99]);
    comp.run();
    assert_eq!(comp.registers, vec![2, 0, 0, 0, 99]);
}

#[test]
fn it_works_with_modes() {
    let mut comp = Computer::new(vec![1002, 4, 3, 4, 33]);
    comp.run();
    assert_eq!(comp.registers, vec![1002, 4, 3, 4, 99]);
}

#[test]
fn it_works_with_negatives() {
    let mut comp = Computer::new(vec![1101, 100, -1, 4, 0]);
    comp.run();
    assert_eq!(comp.registers, vec![1101, 100, -1, 4, 99]);
}

