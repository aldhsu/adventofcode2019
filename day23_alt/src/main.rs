mod computer;
use computer::{Computer, input_to_registers};
use std::collections::HashSet;

fn main() {
    part1()
}

fn part1() {
    let registers = input_to_registers();
    let mut computers : Vec<Computer>= (0..50).map(|i| {
        let mut comp = Computer::new(registers.clone());
        comp.input(i);
        comp
    }).collect();
    let mut last_y = 0;
    let mut nat = [0, 0];

    loop {
        for i in 0..50 {
            let comp = &mut computers[i];
            comp.input(-1);
            comp.run();
            let mut values = vec![];

            while let Some(out) = comp.outputs.pop_front() {
                values.push(out);
            }

            values.chunks_exact(3).for_each(|chunk| {
                if let Some(255) = chunk.get(0) {
                    let y =  *chunk.get(2).unwrap();
                    nat = [*chunk.get(1).unwrap(), y];
                }

                if let Some(receiver) = computers.get_mut(*chunk.get(0).unwrap() as usize) {
                    receiver.input(*chunk.get(1).expect("couldn't send x"));
                    receiver.input(*chunk.get(2).expect("couldn't send y"));
                };
            })
        }

        if computers.iter().all(|comp| comp.inputs.is_empty() && comp.outputs.is_empty()) {
            let first_comp = computers.get_mut(0).unwrap();
            let y = nat[1];
            if y == last_y {
                println!("twice in a row {}", y);
                break
            }
            last_y = y;
            first_comp.input(nat[0]);
            first_comp.input(y);
        }
    }
}
