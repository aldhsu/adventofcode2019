mod computer;
use computer::{input_to_registers, Computer};
use std::collections::VecDeque;
use std::fmt::write;
use std::iter;

fn main() {
    // part1();
    part2();
}

#[derive(Clone)]
enum Instruction {
    And(Register, Register),
    Or(Register, Register),
    Not(Register, Register),
    Walk,
    Run,
}

impl Into<String> for Instruction {
    fn into(self) -> String {
        use Instruction::*;
        match self {
            And(a, b) => vec!["AND".to_string(), a.into(), b.into()],
            Or(a, b) => vec!["OR".to_string(), a.into(), b.into()],
            Not(a, b) => vec!["NOT".to_string(), a.into(), b.into()],
            Walk => vec!["WALK\n".to_string()],
            Run => vec!["RUN\n".to_string()],
        }
        .join(" ")
    }
}

#[derive(Clone)]
enum Register {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Temp,
    Jump,
}

impl Into<String> for Register {
    fn into(self) -> String {
        use Register::*;
        match self {
            One => "A",
            Two => "B",
            Three => "C",
            Four => "D",
            Five => "E",
            Six => "F",
            Seven => "G",
            Eight => "H",
            Nine => "I",
            Temp => "T",
            Jump => "J",
        }
        .to_string()
    }
}

struct SpringProgram {
    comp: Computer,
    instructions: Vec<Instruction>,
}
impl SpringProgram {
    fn run(&mut self) {
        self.instructions
            .iter()
            .cloned()
            .map(|instruction| instruction.into())
            .collect::<Vec<String>>()
            .join("\n")
            .chars()
            .for_each(|c| {
                self.comp.input((c as u8) as i64);
            });

        self.comp.run();
        println!(
            "{}",
            self.comp
                .outputs
                .iter()
                .map(|c| (*c as u8) as char)
                .collect::<String>()
        )
    }

    fn outputs(&self) -> &VecDeque<i64> {
        &self.comp.outputs
    }
}

fn part1() {
    use Instruction::*;
    use Register::*;
    let registers = input_to_registers();
    let mut comp = Computer::new(registers);

    let mut prog = SpringProgram {
        comp,
        instructions: vec![
            Not(Three, Jump),
            Or(Four, Temp),
            And(Temp, Jump),
            Not(One, Temp),
            Or(Temp, Jump),
            Walk,
        ],
    };

    prog.run();
    dbg!(&prog.outputs());
}

fn part2() {
    use Instruction::*;
    use Register::*;
    let registers = input_to_registers();
    let mut comp = Computer::new(registers);

    let mut prog = SpringProgram {
        comp,
        instructions: vec![
            //Jump make sure you can jump again
            Not(Three, Jump),
            And(Four, Jump),
            And(Eight, Jump),
            //Jump over the middle if there is a hole
            Not(Two, Temp),
            And(Four, Temp),
            Or(Temp, Jump),
            //Jump if are right next to a hole
            Not(One, Temp),
            Or(Temp, Jump),
            Run,
        ],
    };

    prog.run();
    dbg!(prog.outputs());
}
