mod computer;

use computer::State;
use computer::Computer;

#[derive(Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Scaff,
    RobotUp,
    RobotDown,
    RobotRight,
    RobotLeft,
    RobotVisited,
    RobotDied,
}

impl From<char> for Tile {
    fn from(i: char) -> Tile {
        match i {
            '.' => Tile::Empty,
            '#' => Tile::Scaff,
            '^' => Tile::RobotUp,
            'v' => Tile::RobotDown,
            '<' => Tile::RobotLeft,
            '>' => Tile::RobotRight,
            '?' => Tile::RobotVisited,
            'x' => Tile::RobotDied,
            _ => unimplemented!(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Facing {
    Left,
    Right,
    Up,
    Down,
}

enum Rotation {
    Left,
    Right,
}


impl From<&str> for Rotation {
    fn from(i: &str) -> Self {
        match i {
            "L" => Rotation::Left,
            "R" => Rotation::Right,
            _ => unimplemented!(),
        }
    }
}

fn execute_order(facing: Facing, mut position: (usize, usize), order: &str) -> (Facing, Vec<(usize, usize)>){
    const FACING_ORDER : [Facing; 4] = [Facing::Up, Facing::Right, Facing::Down, Facing::Left];

    let rotation = Rotation::from(&order.chars().filter(|c| c.is_alphabetic()).collect::<String>()[..]);

    let current_position = FACING_ORDER.iter().position(|f| *f == facing).unwrap();
    let new_facing = match rotation {
        Rotation::Left => {
            let next_position = current_position as isize - 1;
            match next_position {
                -1 => FACING_ORDER[3],
                _ => FACING_ORDER[next_position as usize],
            }
        }
        Rotation::Right => {
            FACING_ORDER[((current_position as isize + 1) % 4).abs() as usize]
        }
    };
    println!("{:?} {:?} {}", &facing, &new_facing, order);

    fn move_robot(new_facing: Facing, position: (usize, usize)) -> (usize, usize) {
        println!("{:?}", position);
        match new_facing {
            Facing::Left => (position.0.checked_sub(1).expect("broke facing left"), position.1),
            Facing::Right => (position.0 + 1, position.1),
            Facing::Up => (position.0, position.1.checked_sub(1).expect("broke facing up")),
            Facing::Down => (position.0, position.1 + 1),
        }
    }

    let number_of_moves : i8 = order.chars().filter(|c| c.is_numeric()).collect::<String>().parse::<i8>().unwrap();
    let mut movements = vec![];
    for _ in 0..number_of_moves {
        let new_position = move_robot(new_facing, position);
        position = new_position;
        movements.push(new_position);
    }
    (new_facing, movements)
}

type Map = Vec<Vec<Tile>>;

fn main() {
    // let input = computer::input_to_registers();
    // let mut comp = Computer::new(input);
    // comp.run();
    //
    // let result = comp.outputs.iter().map(|code| *code as u8 as char).collect::<String>();
    // println!("{}", result);
    // let width = result.lines().nth(0).unwrap().chars().count();
    // let height = result.lines().count();
    // let mut map = vec![vec![Tile::Empty; width]; height];
    // result.lines().enumerate().for_each(|(y, line)| {
    //     line.chars().enumerate().for_each(|(x, c)| {
    //         map[y][x] = Tile::from(c);
    //     })
    // });
    //
    // let mut intersections = vec![];
    //
    // for (y, row) in map.iter().enumerate() {
    //     for (x, _) in row.iter().enumerate() {
    //         if detect_intersection(&map, x, y) {
    //             intersections.push((x, y));
    //         }
    //     }
    // }
    //
    // let result : usize = intersections.iter().map(|(x, y)| x * y).sum();
    // println!("part1: {}", result);
    part2()
}

fn part2_working() {
    //L10L8R8L8R6
    //L10L8R8L8R6
    //R6R8R8
    //R6R6
    //L8L10R6R8R8
    //R6
    //L8L10R6R8R8
    //R6
    //R6
    //L8L10R6R8R8
    //L10L8R8L8R6
    //
    let input = computer::input_to_registers();
    let mut comp = Computer::new(input);
    comp.run();

    let result = comp.outputs.iter().map(|code| *code as u8 as char).collect::<String>();
    let width = result.lines().nth(0).unwrap().chars().count();
    let height = result.lines().count();
    let mut map = vec![vec![Tile::Empty; width]; height];
    result.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map[y][x] = Tile::from(c);
        })
    });

    let mut move_map = map.clone();
    let directions = "L10,L8,R8,L8,R6,L10,L8,R8,L8,R6,R6,R8,R8,R6,R6,L8,L10,R6,R8,R8,R6,R6,L8,L10,R6,R8,R8,R6,R6,L8,L10,R6,R8,R8,L10,L8,R8,L8,R6";
    // let directions = "
    //     L10L8R8L8R6
    //     L10L8R8L8R6
    //     R6R8R8
    //     R6R6L8L10
    //     R6R8R8
    //     R6R6L8L10
    //     R6R8R8
    //     R6R6L8L10
    //     R6R8R8
    //     L,1,0,L,8,R,8,L,8,R,6";
    let mut start = (24, 0);
    let mut facing = Facing::Up;
    let mut positions = vec![];

    directions.split(",").for_each(|order| {
        let (new_facing, mut outputs) = execute_order(facing, start, order);
        facing = new_facing;
        start = *outputs.last().unwrap();
        positions.append(&mut outputs);
    });

    for (x, y) in positions.iter() {
        move_map[*y][*x] = Tile::RobotVisited;
    }

    for row in move_map.iter() {
        let result = row.iter().map(|tile| {
            match tile {
                Tile::Empty => '.',
                Tile::Scaff => '#',
                Tile::RobotVisited => '?',
                _ => 'A',
            }
        }).collect::<String>();
        println!("{}", result);
    }
    ()
}

fn part2() {
    // let directions = "
    //     L10L8R8L8R6 A
    //     L10L8R8L8R6 A
    //     R6R8R8 B
    //     R6R6L8L10 C
    //     R6R8R8 B
    //     R6R6L8L10 C
    //     R6R8R8 B
    //     R6R6L8L10 C
    //     R6R8R8 B
    //     L10L8R8L8R6" A;
    //
    //
    fn convert_to_input(input: &str) -> String {
        let mut out = input.chars().map(|c| c.to_string()).collect::<Vec<String>>().join(",");
        out.push('\n');
        out
    }
    let mut input = computer::input_to_registers();
    input[0] = 2;
    let mut comp = Computer::new(input);
    comp.run();

    let movement_routine = convert_to_input("AABCBCBCBA");
    let mut routine_a = vec!["L","10","L","8","R","8","L","8","R","6"].into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join(",");
    routine_a.push('\n');
    let routine_b = convert_to_input("R6R8R8");
    let mut routine_c = vec!["R","6","R","6","L","8","L","10"].into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join(",");
    routine_c.push('\n');

    for c in movement_routine.chars() {
        comp.input((c as u8) as i64)
    }
    for c in routine_a.chars() {
        comp.input((c as u8) as i64)
    }
    for c in routine_b.chars() {
        comp.input((c as u8) as i64)
    }
    for c in routine_c.chars() {
        comp.input((c as u8) as i64)
    }

    comp.input(('n' as u8) as i64);
    comp.input(('\n' as u8) as i64);
    comp.run();
    dbg!(comp.outputs);
}

#[rustfmt::skip]
const COORDINATES : [(i8, i8); 9]= [
    (-1, 1), (0, 1), (1, 1),
    (-1, 0), (0, 0),  (1, 0),
    (-1, -1),(0, -1), (1, -1)
];

const INTERSECTION_TILES : [Option<Tile>; 9]= [
    Some(Tile::Empty), Some(Tile::Scaff), Some(Tile::Empty),
    Some(Tile::Scaff), Some(Tile::Scaff), Some(Tile::Scaff),
    Some(Tile::Empty), Some(Tile::Scaff), Some(Tile::Empty),
];

fn detect_intersection(map: &Map, x: usize, y: usize) -> bool {
    COORDINATES.iter().map(|(x_offset, y_offset)| {
        safe_get(map, x, y, *x_offset, *y_offset)
    }).zip(INTERSECTION_TILES.iter()).all(|(actual, expected)| actual == expected.as_ref())
}

fn safe_get(map: &Map, x: usize, y: usize, x_offset: i8, y_offset: i8) -> Option<&Tile> {
    map.get(safe_add(y, y_offset)?)?.get(safe_add(x, x_offset)?)
}

fn safe_add(a: usize, b: i8) -> Option<usize>{
    if b.is_negative() {
        a.checked_sub(b.abs() as usize)
    } else {
        a.checked_add(b as usize)
    }
}
