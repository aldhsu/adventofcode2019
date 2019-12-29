mod computer;
use computer::input_to_registers;
use computer::Computer;

fn main() {
    // println!("part1: {:?}", part1());
    println!("part2: {:?}", part2());
}

fn part1() -> usize {
    let registers = input_to_registers();
    let mut comp = Computer::new(registers);
    let mut canvas: [[usize; 50]; 50] = [[0; 50]; 50];
    for x in 0..50 {
        for y in 0..50 {
            canvas[y as usize][x as usize] = attracted(&comp, x, y) as usize;
        }
    }

    for row in canvas.iter() {
        println!("{}", row.iter().map(|c| c.to_string()).collect::<String>());
    }
    canvas.iter().map(|row| row.iter().sum::<usize>()).sum::<usize>()
}

fn part2() -> i64 {
    let registers = input_to_registers();
    let mut comp = Computer::new(registers);
    let (mut x, mut y) = (0, 0);
    loop {
        if !attracted(&comp, x, y + 99) {
            x+= 1;
            continue
        }

        if !attracted(&comp, x + 99, y) {
            y+= 1;
            continue
        }

        return 10000 * x + y
    }
}
 
fn attracted(comp: &Computer, x: i64, y: i64) -> bool {
    let mut new_comp = comp.clone();
    new_comp.input(x);
    new_comp.input(y);
    new_comp.run();
    match new_comp.outputs.pop_front().unwrap() {
        0 => false,
        1 => true,
        _ => unimplemented!(),
    }
}
