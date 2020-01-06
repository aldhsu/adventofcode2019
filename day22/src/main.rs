use std::fs;

fn main() {
    println!("part1 {}", part1());
    println!("part2 {}", part2());
}

fn mod_pow_by_squaring(mut base: i128, mut exponent: i128, modulus: i128) -> i128 {
    if modulus == 1 { return 0 }

    let mut result = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base).rem_euclid(modulus)
        }
        exponent >>= 1;
        base = (base * base).rem_euclid(modulus)
    }
    result
}

fn geometric_sum(starting_pos: i128, a: i128, b: i128, length: i128, shuffles: i128) -> i128 {
    let a_pow_shuffles = mod_pow_by_squaring(a, shuffles, length);
    let p = (b * (1 - a_pow_shuffles)).rem_euclid(length);
    let q = (1 - a).rem_euclid(length);

    // invert Q so you can divide
    let q_inv = mod_pow_by_squaring(q, length - 2, length);
    let p_on_q = p * q_inv;
    (a_pow_shuffles * starting_pos + p_on_q).rem_euclid(length)

}

fn inv_geometric_sum(starting_pos: i128, a: i128, b: i128, length: i128, shuffles: i128) -> i128 {
    let a_pow_shuffles = mod_pow_by_squaring(a, shuffles, length);
    let p = (b * (1 - a_pow_shuffles)).rem_euclid(length);
    let q = (1 - a).rem_euclid(length);

    let q_inv = mod_pow_by_squaring(q, length - 2, length);
    let p_on_q = p * q_inv;

    let a_inv = mod_pow_by_squaring(a_pow_shuffles, length - 2, length);
    ((starting_pos - p_on_q) * a_inv).rem_euclid(length)
}

fn part1() -> i32 {
    let instructions = input_to_instructions("input.txt");
    let length = 10007;
    let lgc = instructions.iter().fold((1, 0), |lgc, instruction| {
        instruction.compose_lgc(lgc.0, lgc.1, length)
    });
    geometric_sum(2019, lgc.0, lgc.1, length, 1) as i32
}

fn part2() -> i128 {
    let instructions = input_to_instructions("input.txt");
    let length = 119315717514047 ;
    let shuffles = 101741582076661;
    let lgc = instructions.iter().fold((1, 0), |lgc, instruction| {
        instruction.compose_lgc(lgc.0, lgc.1, length)
    });
    let card = 2020;
    inv_geometric_sum(card, lgc.0, lgc.1, length, shuffles) as i128
}

enum Instruction {
    NewStack,
    Increment(usize),
    Cut(isize),
}

impl Instruction {
    fn apply(&self, mut deck: Vec<i32>) -> Vec<i32> {
        use Instruction::*;

        match self {
            NewStack => deck.into_iter().rev().collect(),
            Increment(n) => {
                let mut iter = deck.iter();
                let mut result = vec![0; deck.len()];

                for count in 0..deck.len() {
                    let replace_at = (count * n) % deck.len();
                    result[replace_at] = *iter.next().unwrap();
                }

                result
            }
            Cut(n) => {
                if n.is_positive() {
                    let mut back = deck.split_off(*n as usize);
                    back.append(&mut deck);
                    back
                } else {
                    let index = deck.len() as isize + n;

                    let mut back = deck.split_off(index as usize);
                    back.append(&mut deck);
                    back
                }
            }
        }
    }

    fn compose_lgc(&self, a: i128, b: i128, length: i128) -> (i128, i128) {
        use Instruction::*;

        let (c, d) = match self {
            NewStack => (-1, -1),
            Cut(n) => (1, -1 * *n as i128),
            Increment(n) => (*n as i128, 0),
        };

        ((a * c).rem_euclid(length), (b * c + d).rem_euclid(length))
    }
}

fn input_to_instructions(filename: &str) -> Vec<Instruction> {
    use Instruction::*;
    let input = fs::read_to_string(filename).expect("couldn't read file");
    input
        .lines()
        .map(|line| {
            if line.starts_with("deal with increment") {
                let number = parse_num(line);
                return Increment(number as usize);
            }

            if line.starts_with("cut") {
                let number = parse_num(line);
                return Cut(number as isize);
            }

            if line.starts_with("deal into new stack") {
                return NewStack;
            }

            panic!("fell through")
        })
        .collect()
}

fn parse_num(input: &str) -> i32 {
    input
        .chars()
        .filter(|c| c.is_digit(10) || *c == '-')
        .collect::<String>()
        .parse()
        .expect("couldn't turn into number")
}

#[test]
fn instruction_cut_works() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(
        Instruction::Cut(3).apply(vec),
        vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]
    );
}

#[test]
fn instruction_new_deck_works() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(
        Instruction::NewStack.apply(vec),
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    );
}

#[test]
fn instruction_increment_works() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(
        Instruction::Increment(3).apply(vec),
        vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
    );
}

#[test]
fn part1_works() {
    assert_eq!(part1(), 6061);
}
