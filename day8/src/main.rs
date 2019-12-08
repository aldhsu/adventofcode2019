use std::fs;

fn main() {
    part2();
}

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let layers = split_into_layers(input, 25, 6);

    println!("layers: {}", layers.len());
    let zero_layer = layers
        .iter()
        .max_by(|&a, &b| {
            let a_count = a.iter().flatten().filter(|&&c| c != '0').count();
            let b_count = b.iter().flatten().filter(|&&c| c != '0').count();
            a_count.cmp(&b_count)
        })
        .expect("Couldn't find a max");

    let ones = zero_layer.iter().flatten().filter(|&&c| c == '1').count();
    let twos = zero_layer.iter().flatten().filter(|&&c| c == '2').count();

    println!("Part1: {}", ones * twos);
}

fn part2() {
    let width = 25;
    let height = 6;
    let input = fs::read_to_string("input.txt").unwrap();
    let layers = split_into_layers(input, width, height);

    let mut canvas = vec![vec![2; width]; height];
    for layer in layers {
        for (y, row) in layer.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match canvas[y][x] {
                    2 => canvas[y][x] = c.to_digit(10).unwrap(),
                    _ => continue,
                }
            }
        }
    }

    for line in canvas.iter() {
        let line = line.iter().map(|d| match d {
            1 => '@',
            _ => ' ',
        }).collect::<String>();
        println!("{:?}", line);
    }
}

fn split_into_layers<'a>(input: String, width: usize, height: usize) -> Vec<Vec<Vec<char>>> {
    input
        .chars()
        .collect::<Vec<_>>()
        .chunks(width)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
        .chunks(height)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>()
}

#[test]
fn it_works() {
    let input = "123456789012".to_string();
    assert_eq!(
        split_into_layers(input, 3, 2),
        [
            [['1', '2', '3'], ['4', '5', '6']],
            [['7', '8', '9'], ['0', '1', '2']],
        ],
    );
}
