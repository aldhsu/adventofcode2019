use std::cmp;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result : i64 = file.lines().map(|line| {
        let num = line.parse::<i64>().expect("unparseable");
        num / 3 - 2
    }).sum();

    println!("result1: {}", result);

    let result2 : i64 = file.lines().map(|line| {
        let num = line.parse::<i64>().expect("unparseable");
        calculate_fuel(num)
    }).sum();

    println!("result2: {}", result2);
}

fn calculate_fuel(num: i64) -> i64 {
        let mut total = 0;
        let mut mass = num;

        while mass > 0{
            let fuel = mass / 3 - 2;
            if fuel.is_negative() { break }
            total += fuel;
        }
        total
}

#[test]
fn it_calculates_correctly() {
    assert_eq!(calculate_fuel(1969), 966);
}

#[test]
fn it_calculates_correctly2() {
    assert_eq!(calculate_fuel(100756), 50346);
}

#[test]
fn it_calculates_correctly3() {
    assert_eq!(calculate_fuel(14), 2);
}

#[test]
fn it_calculates_correctly4() {
    assert_eq!(calculate_fuel(4), 0);
}
