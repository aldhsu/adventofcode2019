use std::collections::HashMap;
use std::fs;
use std::io;
use std::cmp;

fn main() -> Result<(), Error> {
    let reactants = produce_reactants("input.txt", "FUEL");
    println!("part1 : {}", reactants?.get("ORE").unwrap());

    println!("{}", part2("input.txt")?);
    Ok(())
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    NotFound,
    ParseIntError,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseIntError
    }
}

type Map = HashMap<String, Reaction>;

struct Reaction {
    units: i64,
    ingredients: Vec<(i64, String)>,
}

fn parse_input(filename: &str) -> Result<Map, Error> {
    let input = fs::read_to_string(filename)?;
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut halves = line.splitn(2, "=>").map(|s| s.trim());
        let ingredients = halves
            .next()
            .ok_or(Error::NotFound)?
            .split(", ")
            .map(|ingredient| parse_total_units(ingredient.trim()).unwrap())
            .collect::<Vec<(i64, String)>>();

        let (units, code) = parse_total_units(halves.next().ok_or(Error::NotFound)?)?;
        map.insert(code, Reaction { ingredients, units });
    }

    Ok(map)
}

fn parse_total_units(input: &str) -> Result<(i64, String), Error> {
    let mut iter = input.split(" ");
    let unit = iter.next().ok_or(Error::NotFound)?.parse()?;
    let code = iter.next().ok_or(Error::NotFound)?;
    Ok((unit, code.to_string()))
}

fn react(map: &Map, reactants: &mut Reactants, key: &str, times_do: i64) {
    let ing = map.get(key).expect("to have key");
    ing.ingredients.iter().for_each(|(needed, code)| {
        if code == "ORE" {
            let entry = reactants.entry("ORE".to_string()).or_insert(0);
            *entry += needed * times_do;
        } else {
            let total_available = reactants.entry(code.to_string()).or_insert(0);
            *total_available -= needed * times_do;
            let available = *reactants.get(code).unwrap();
            let makes = map.get(code).unwrap().units;

            if available > 0 { return }
            else {
                let do_times = available.abs() / makes + if available.abs() % makes > 0 { 1 } else { 0 };
                react(map, reactants, code, do_times);
            }
        }
    });
    let reactant = reactants.entry(key.to_string()).or_insert(0);
    *reactant += ing.units * times_do;
}

type Reactants = HashMap<String, i64>;

fn produce_reactants(input: &str, item: &str) -> Result<Reactants, Error> {
    let mut map = parse_input(input)?;
    let mut reactants: HashMap<String, i64> = HashMap::new();
    react(&map, &mut reactants, item, 1);
    Ok(reactants)
}

fn build_fuel(map: &Map, reactants: &mut Reactants, speed: i64) {
    react(&map, reactants, "FUEL", speed);
}

fn part2(input: &str) -> Result<i64, Error> {
    let mut map = parse_input(input)?;
    let mut reactants = Reactants::new();
    let target = 1000000000000;

    while *reactants.get("ORE").unwrap_or(&0) < target {
        let total_ore = *reactants.get("ORE").unwrap_or(&0);
        dbg!(total_ore);
        let speed = if total_ore == 0 {
            8840000
        } else {
            cmp::max(target / total_ore, 1)
        };
        build_fuel(&map, &mut reactants, speed)
    }
    while *reactants.get("ORE").unwrap_or(&0) < 900000000000 {
        build_fuel(&map, &mut reactants, 100000)
    }
    Ok(*reactants.get("FUEL").ok_or(Error::NotFound)?)
}

#[test]
fn it_reacts_simply() -> Result<(), Error> {
    let result = produce_reactants("part1_example1.txt", "A")?;
    assert_eq!(result.get("A"), Some(&10));
    Ok(())
}

// #[test]
#[test]
fn it_reacts_more_complex() -> Result<(), Error> {
    let result = produce_reactants("part1_example1.txt", "C")?;
    assert_eq!(result.get("ORE"), Some(&11));
    Ok(())
}

#[test]
fn part1_example5() -> Result<(), Error> {
    let result = produce_reactants("part1_example5.txt", "FUEL")?;
    assert_eq!(result.get("ORE"), Some(&2210736));
    Ok(())
}

#[test]
fn part2_example5() -> Result<(), Error> {
    let result = part2("part1_example5.txt")?;
    assert_eq!(result, 460664);
    Ok(())
}
