mod computer;
use computer::{input_to_registers, load_registers, Computer, State};
use std::fs;
use std::fs::OpenOptions;
use std::io::{stdin, Write};
use std::iter;
use itertools::Itertools;

fn main() {
    part1();
}

struct Game<'a> {
    comp: Computer,
    inputs: Vec<&'a str>,
}

enum Action {
    North,
    South,
    East,
    West,
    Take(String),
    DropItem(String),
    Inv,
    Unknown(String),
    Save,
    Load,
}

impl From<&str> for Action {
    fn from(i: &str) -> Self {
        use Action::*;
        match i {
            "n" => North,
            "s" => South,
            "e" => East,
            "w" => West,
            "i" => Inv,
            "save" => Save,
            "load" => Load,
            d if d.starts_with("d") => DropItem(d.split_at(1).1.to_string()),
            t if t.starts_with("t") => Action::Take(t.split_at(1).1.to_string()),
            a => Unknown(a.to_string()),
        }
    }
}

impl Into<Vec<i64>> for Action {
    fn into(self) -> Vec<i64> {
        use Action::*;

        match self {
            North => "north".to_string(),
            South => "south".to_string(),
            East => "east".to_string(),
            West => "west".to_string(),
            Take(value) => "take".to_string() + &value,
            DropItem(value) => "drop".to_string() + &value,
            Inv | Save | Load => "inv".to_string(),
            Unknown(value) => value,
        }
        .chars()
        .map(|c| (c as u8) as i64)
        .chain(iter::once(('\n' as u8) as i64))
        .collect::<Vec<i64>>()
    }
}

impl Game<'_> {
    fn run(&mut self) {
        while State::Halted != self.comp.state {
            self.comp.run();
            let output = self
                .comp
                .outputs
                .iter()
                .map(|&i| ((i as u8) as char).to_string())
                .collect::<String>();
            println!("{}", output);
            self.comp.outputs.clear();
            self.get_input().iter().for_each(|&i| self.comp.input(i));
        }
    }

    fn get_input(&mut self) -> Vec<i64> {
        println!("What do you do? (n)orth, (s)outh, (e)ast, (w)est, (t)ake, (d)rop, (i)nv");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        let action = Action::from(input.trim());

        if let Action::Save = action {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("save.txt")
                .expect("couldn't get file handle");
            file.write(
                self.comp
                    .registers
                    .iter()
                    .map(|reg| reg.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    .as_bytes(),
            )
            .ok();
        }

        if let Action::Load = action {
            let comp = Computer::new(load_registers("save.txt"));
            self.comp = comp;
        }
        action.into()
    }

    fn auto(&mut self) {
        fn input_cmd(action: Action, comp: &mut Computer) {
            let instructions: Vec<i64> = action.into();
            for instruction in instructions { comp.input(instruction) }
        }
        print!("{:?}", self.inputs );
        //drop all the items in the input list
        for i in 0..self.inputs.len() {
            let action = Action::DropItem(" ".to_owned() + self.inputs.get(i).unwrap());
            input_cmd(action, &mut self.comp);
            self.comp.run();
        }
        // move north
        input_cmd(Action::North, &mut self.comp);
        self.comp.run();
        input_cmd(Action::West, &mut self.comp);
        self.comp.run();
        input_cmd(Action::North, &mut self.comp);
        self.comp.run();
        input_cmd(Action::West, &mut self.comp);
        self.comp.run();
        self.comp.outputs.clear();

        input_cmd(Action::North, &mut self.comp);
        self.comp.run();

        let output = self
            .comp
            .outputs
            .iter()
            .map(|&i| ((i as u8) as char).to_string())
            .collect::<String>();
        print!("{}", output);
        self.comp.outputs.clear();
    }
}

// fn part1() {
//     let registers = input_to_registers();
//     let comp = Computer::new(registers);
//     let mut game = Game { comp, inputs: vec![] };
//     game.run();
// }
//
fn part1() {
    let items = [
        "jam",
        "loom",
        "mug",
        "spool of cat6",
        "prime number",
        "food ration",
        "fuel cell",
        "manifold",];
    for size in 1..9  { 
        for list in items.iter().combinations(size) {
            let registers = load_registers("save.txt");
            let comp = Computer::new(registers);
            let mut game = Game { comp, inputs: list.into_iter().map(|i| *i).collect::<Vec<&str>>() };
            game.auto();
        }
    }
}
