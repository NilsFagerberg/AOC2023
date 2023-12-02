use std::{io::Error, collections::HashMap};

fn day2_part1(file_path: &str) -> Result<usize, Error> {

    let file = std::fs::read_to_string(file_path)?;

    let total_cubes_per_color = HashMap::from([
        ("red", 12), ("green", 13), ("blue", 14)]);

    let mut sum = 0;

    'game: for (i, line) in file.lines().enumerate() {
        let ix = line.find(": ").expect("Malformed input!");
        let substr = &line[ix+2..];
        
        // One "game round"
        for split in substr.split(';') {
            // Each color
            for subsplit in split.split(',') {
                let mut iter = subsplit.split_ascii_whitespace();
                let number_str = iter.next().expect("Malformed input, no number found");
                let number = number_str.parse::<i32>().expect("NaN!");
                let color = iter.next().expect("Malformed input, no color found!");

                assert!(total_cubes_per_color.contains_key(color), "Rogue color {} detected!", color);

                // Impossible number of cubes
                if number > total_cubes_per_color[color] {
                    continue 'game;
                }
            }
        }
        sum += i + 1;
    }
    Ok(sum)
}

fn day2_part2(file_path: &str) -> Result<i32, Error> {
    let file = std::fs::read_to_string(file_path)?;

    let mut sum = 0;

    for (i, line) in file.lines().enumerate() {
        let ix = line.find(": ").expect("Malformed input!");
        let substr = &line[ix+2..];

        let mut cubes_seen = HashMap::from([
            ("red", 0), ("green", 0), ("blue", 0)]);    
        
        // One "game round"
        for split in substr.split(';') {
            // Each color
            for subsplit in split.split(',') {
                let mut iter = subsplit.split_ascii_whitespace();
                let number_str = iter.next().expect("Malformed input, no number found");
                let number = number_str.parse::<i32>().expect("NaN!");
                let color = iter.next().expect("Malformed input, no color found!");

                assert!(cubes_seen.contains_key(color), "Rogue color {} detected!", color);

                if number > cubes_seen[color] {
                    cubes_seen.insert(color, number);
                }
            }
        }
        let pow = cubes_seen.values().fold(1, | x, y | x * y);
        sum += pow;
    }
    Ok(sum)
}

fn main() {
    // Part 1
    let res_part1 = day2_part1("day2/input/actual.txt").expect("An error occurred!");
    println!("part 1 sum: {res_part1}");

    // Part 2
    let res_part2 = day2_part2("day2/input/actual.txt").expect("An error occurred!");
    println!("part 2 sum: {res_part2}");
}

#[test]
fn test_part1() {

    let res = day2_part1("day2/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 8);
}

#[test]
fn test_part2() {
    let res = day2_part2("day2/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 2286);
}