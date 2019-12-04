use std::collections::HashMap;
use std::fs;

fn main() {
    part2()
}

fn part1() {
    let result = (136760..=595730)
        .filter(|i| {
            let mut digits = i.to_string().chars().collect::<Vec<_>>();
            digits.dedup();
            let has_double = digits.len() < 6;

            let always_increases =
                i.to_string().chars().try_fold(
                    '0',
                    |prev, c| {
                        if c >= prev {
                            Some(c)
                        } else {
                            None
                        }
                    },
                );

            has_double && always_increases.is_some()
        })
        .count();

    println!("{}", result);
}

fn part2() {
    let result = (136760..=595730)
        .filter(|i| {
            let mut digits = i.to_string().chars().collect::<Vec<_>>();
            digits.dedup();
            let has_duplicates = digits.len() < 6;
            let mut has_a_double = false;
            if has_duplicates {
                let mut counts = HashMap::new();
                for (i, c) in i.to_string().chars().enumerate() {
                    let entry = counts.entry(c).or_insert(vec![]);
                    entry.push(i);
                }

                has_a_double = counts.iter().any(|(_, vec)| match vec.len() {
                    2 => {
                        let mut nums = vec.iter();
                        let num_one = nums.next().unwrap();
                        let num_two = nums.next().unwrap();
                        (*num_one as i32 - *num_two as i32).abs() == 1
                    }
                    _ => false,
                });
            } else {
                return false;
            }

            let always_increases =
                i.to_string().chars().try_fold(
                    '0',
                    |prev, c| {
                        if c >= prev {
                            Some(c)
                        } else {
                            None
                        }
                    },
                );

            has_a_double && always_increases.is_some()
        })
        .count();

    println!("{}", result);
}
