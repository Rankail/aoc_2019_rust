use std::fs;

fn solve1() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input file");

    let digits: Vec<u32> = content.chars().flat_map(|c| c.to_digit(10)).collect();

    let dim = (25, 6);

    let layers: Vec<Vec<u32>> = digits.chunks(dim.0 * dim.1).map(|s| s.into()).collect();

    let min_zeros = layers.iter().min_by_key(|layer: &&Vec<u32>| layer.iter().filter(|v| **v == 0).count())
        .expect("No layer found");

    let num_1 = min_zeros.iter().filter(|v| **v == 1).count();
    let num_2 = min_zeros.iter().filter(|v| **v == 2).count();

    println!("Answer 1: {}", num_1 * num_2);
}

fn solve2() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input file");

    let digits: Vec<u32> = content.chars().flat_map(|c| c.to_digit(10)).collect();

    let dim = (25, 6);

    let layers: Vec<Vec<u32>> = digits.chunks(dim.0 * dim.1).map(|s| s.into()).collect();

    let mut image = Vec::new();
    for i in 0..(dim.0 * dim.1) {
        for layer in &layers {
            if layer[i] != 2 {
                image.push(layer[i]);
                break;
            }
        }
    }

    println!("Answer 2:");
    for y in 0..dim.1 {
        for x in 0..dim.0 {
            match image[y * dim.0 + x] {
                0 => print!(" "),
                1 => print!("{}", char::from_u32(9608).unwrap_or('#')),
                _ => {}
            }
        }
        println!();
    }
}

fn main() {
    solve2();
}
