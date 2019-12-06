use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type Map = HashMap<String, HashSet<String>>;
type CountMap = HashMap<String, usize>;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    part2(&input, "B2L", "9XB");
}

fn part1(input: &str) -> usize {
    let count_map = make_count_map(&input);

    let result: usize = count_map.values().sum();
    println!("{:?}", count_map);
    println!("{}", result);
    result
}

fn make_map(input: &str) -> Map {
    let mut map: Map = HashMap::new();

    input.lines().for_each(|line| {
        let mut iter = line.trim().split(")");
        let anchor = iter.next().unwrap();
        let orbiter = iter.next().unwrap();

        let orbiters = map.entry(anchor.to_string()).or_insert(HashSet::new());
        orbiters.insert(orbiter.to_string());
    });

    map
}

fn make_count_map(input: &str) -> CountMap {
    let map = make_map(input);
    let mut count_map: CountMap = HashMap::new();

    for (key, _) in &map {
        count_indirects(&key, &map, &mut count_map);
    }

    count_map
}

fn count_indirects(id: &str, map: &Map, count_map: &mut CountMap) -> usize {
    if let Some(count) = count_map.get(id) {
        return *count;
    } else {
        if let Some(orbiters) = map.get(id) {
            let count = orbiters.iter().fold(0, |acc, orbiter| {
                acc + count_indirects(orbiter, map, count_map)
            });
            let total = count + orbiters.len();
            count_map.insert(id.to_string(), total);
            total
        } else {
            0
        }
    }
}

fn part2(input: &str, start: &str, end: &str) -> usize {
    let map = make_map(input);
    let mut transfer_map: CountMap = HashMap::new();

    transfer(start, &map, &mut transfer_map, 0);
    *transfer_map.get(end).expect("couldn't get end planet")
}

fn transfer(id: &str, map: &Map, count_map: &mut CountMap, step: usize) {
    count_map.entry(id.to_string()).or_insert(step);
    if let Some(planets) = map.get(id) {
        for id in planets {
            transfer(id, map, count_map, step + 1);
        }
    }

    if let Some((orbit_id, _)) = map
        .iter()
        .find(|(key, planets)| count_map.get(*key).is_none() && planets.get(id).is_some())
    {
        transfer(orbit_id, map, count_map, step + 1);
    };
}

#[test]
fn part1_example1() {
    let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
    assert_eq!(part1(input), 42);
}

#[test]
fn part2_example1() {
    let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
    let result = part2(input, "I", "K");
    assert_eq!(result, 4);
}
