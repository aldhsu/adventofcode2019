use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let mut moons = [
        Moon {
            x: 17,
            y: 5,
            z: 1,
            velocity: Velocity::new(),
        },
        Moon {
            x: -2,
            y: -8,
            z: 8,
            velocity: Velocity::new(),
        },
        Moon {
            x: 7,
            y: -6,
            z: 14,
            velocity: Velocity::new(),
        },
        Moon {
            x: 1,
            y: -10,
            z: 4,
            velocity: Velocity::new(),
        },
    ];

    // let result1 = part1(&mut moons, 1000);
    // println!("part1: {}", result1)

    let result2 = part2(&mut moons);
    // it_can_find_repeat()
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    velocity: Velocity,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

impl Velocity {
    fn new() -> Self {
        Velocity { x: 0, y: 0, z: 0 }
    }

    fn kin(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Moon {
    fn apply_gravity(&mut self, other: &Self) {
        self.velocity.x += Self::gravity_change(self.x, other.x);
        self.velocity.y += Self::gravity_change(self.y, other.y);
        self.velocity.z += Self::gravity_change(self.z, other.z);
    }

    fn apply_velocity(&mut self) {
        self.x += self.velocity.x;
        self.y += self.velocity.y;
        self.z += self.velocity.z;
    }

    fn gravity_change(a: i32, b: i32) -> i32 {
        match a.cmp(&b) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1,
        }
    }

    fn pot(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn kin(&self) -> i32 {
        self.velocity.kin()
    }

    fn energy(&self) -> i32 {
        self.pot() * self.kin()
    }
}

fn simulate(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }
            let other = moons[j].clone();
            let moon = moons.get_mut(i).unwrap();
            moon.apply_gravity(&other);
        }
    }

    moons.iter_mut().for_each(|moon| moon.apply_velocity());
}

fn part1(moons: &mut [Moon], steps: usize) -> i32 {
    for _ in 0..steps {
        simulate(moons);
    }
    moons.iter().map(|moon| moon.energy()).sum()
}

#[derive(Debug)]
struct Universe {
    x: Vec<i32>,
    y: Vec<i32>,
    z: Vec<i32>,
    vel_x: Vec<i32>,
    vel_y: Vec<i32>,
    vel_z: Vec<i32>,
}

fn part2(moons: &mut [Moon]) {
    let past_states = Universe {
        x: moons.iter().map(|moon| moon.x).collect::<Vec<_>>(),
        y: moons.iter().map(|moon| moon.y).collect::<Vec<_>>(),
        z: moons.iter().map(|moon| moon.z).collect::<Vec<_>>(),
        vel_x: vec![0, 0, 0, 0],
        vel_y: vec![0, 0, 0, 0],
        vel_z: vec![0, 0, 0, 0],
    };
    let mut repeats = Universe {
        x: vec![],
        y: vec![],
        z: vec![],
        vel_x: vec![],
        vel_y: vec![],
        vel_z: vec![],
    };

    simulate(moons);

    let mut count: usize = 1;
    while (repeats.x.len() < 2) || (repeats.y.len() < 2) || (repeats.z.len() < 2)
    // for i in 0..100000000
    {
        println!("{}", count);
        // if moons.iter().all(|moon| moon.kin() == 0)  {
        if past_states.x == moons.iter().map(|moon| moon.x).collect::<Vec<_>>() && moons.iter().all(|moon| moon.velocity.x == 0) {
            repeats.x.push(count as i32);
        }
        if past_states.y == moons.iter().map(|moon| moon.y).collect::<Vec<_>>() && moons.iter().all(|moon| moon.velocity.y == 0)  {
            repeats.y.push(count as i32);
        }
        if past_states.z == moons.iter().map(|moon| moon.z).collect::<Vec<_>>() && moons.iter().all(|moon| moon.velocity.z == 0)  {
            repeats.z.push(count as i32);
        }
        // if past_states.vel_x == moons.iter().map(|moon| moon.velocity.x).collect::<Vec<_>>() {
        //     repeats.vel_x.push(count as i32);
        // }
        // if past_states.vel_y == moons.iter().map(|moon| moon.velocity.y).collect::<Vec<_>>() {
        //     repeats.vel_y.push(count as i32);
        // }
        // if past_states.vel_z == moons.iter().map(|moon| moon.velocity.z).collect::<Vec<_>>() {
        //     repeats.vel_z.push(count as i32);
        // }
        // }
        // for (i, moon) in moons.iter().enumerate() {
        //     let initial = past_states[i];
        //     if initial == *moon {
        //         repeats[i][0] = Some(count);
        //         repeats[i][1] = Some(count);
        //         repeats[i][2] = Some(count);
        //     }
            // if initial.x == moon.x {
            //     repeats[i][0] = Some(count);
            // };
            // if initial.y == moon.y {
            //     repeats[i][1] = Some(count);
            // };
            // if initial.z == moon.z {
            //     repeats[i][2] = Some(count);
            // };
        // }
        count += 1;
        simulate(moons);
    }

    println!("{:?}", repeats);
}

#[test]
fn it_responds_to_gravity() {
    let mut a = Moon {
        x: 0,
        y: 0,
        z: 0,
        velocity: Velocity::new(),
    };
    let b = Moon {
        x: 0,
        y: 0,
        z: 1,
        velocity: Velocity::new(),
    };
    a.apply_gravity(&b);
    assert_eq!(a.velocity.z, 1);

    a.apply_gravity(&b);
    assert_eq!(a.velocity.z, 2);
}

#[test]
fn it_calculates_gravity_correctly() {
    let mut moons = [
        Moon {
            x: -1,
            y: 0,
            z: 2,
            velocity: Velocity::new(),
        },
        Moon {
            x: 2,
            y: -10,
            z: -7,
            velocity: Velocity::new(),
        },
        Moon {
            x: 4,
            y: -8,
            z: 8,
            velocity: Velocity::new(),
        },
        Moon {
            x: 3,
            y: 5,
            z: -1,
            velocity: Velocity::new(),
        },
    ];

    part1(&mut moons, 1);
    assert_eq!(
        moons[0],
        Moon {
            x: 2,
            y: -1,
            z: 1,
            velocity: Velocity { x: 3, y: -1, z: -1 }
        }
    );

    let energy = part1(&mut moons, 9);
    assert_eq!(
        moons[0],
        Moon {
            x: 2,
            y: 1,
            z: -3,
            velocity: Velocity { x: -3, y: -2, z: 1 }
        }
    );

    assert_eq!(energy, 179);
}

// #[test]
fn it_can_find_repeat() {
    let mut moons = [
        Moon {
            x: -1,
            y: 0,
            z: 2,
            velocity: Velocity::new(),
        },
        Moon {
            x: 2,
            y: -10,
            z: -7,
            velocity: Velocity::new(),
        },
        Moon {
            x: 4,
            y: -8,
            z: 8,
            velocity: Velocity::new(),
        },
        Moon {
            x: 3,
            y: 5,
            z: -1,
            velocity: Velocity::new(),
        },
    ];

    part2(&mut moons);
    // assert_eq!(1, 2);
}
