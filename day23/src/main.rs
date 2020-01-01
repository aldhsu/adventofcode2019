mod computer;
use computer::{ Computer, input_to_registers };
use std::thread;
use std::collections::HashMap;
use std::sync::mpsc::TryRecvError;

fn main() {
    part1()
}

fn part1() {
    let registers = input_to_registers();
    let mut inputs = vec![];
    let mut outputs = vec![];
    let mut computers = vec![];

    (0..50).for_each(|i| {
        let (comp, input, output) = Computer::new(registers.clone());
        input.send(i).ok();
        inputs.push(input);
        outputs.push(output);
        computers.push(comp);
    });

    for mut computer in computers.drain(0..) {
        thread::spawn(move || {
            computer.run();
        });
    }

    let mut letterbox: HashMap<usize, Vec<i64>> = HashMap::new();

    let result = 'outer: loop {
        for (i, output) in outputs.iter().enumerate() {
            match output.try_recv() {
                Ok(out) => {
                    let entry = letterbox.entry(i).or_insert(vec![]);
                    if entry.len() == 3 {
                        if entry[0] == 255 { break 'outer entry[2] }
                        let input = &inputs[entry[0] as usize];
                        input.send(entry[1]).expect("couldn't send x");
                        input.send(entry[2]).expect("couldn't send y");
                    } else {
                        entry.push(out)
                    }
                }
                Err(TryRecvError::Empty) => {
                    continue
                }
                _ => panic!("destroyed?")
            }
        }
    };

    println!("{}", result)
}
