use std::io::Error;

fn day_part1(file_path: &str) -> Result<i32, Error> {
    let file = std::fs::read_to_string(file_path)?;
    let mut res = 0;

    for line in file.lines() {
    }

    Ok(res)
}

fn day_part2(file_path: &str) -> Result<i32, Error> {

    let file = std::fs::read_to_string(file_path)?;
    let mut res = 0;

    for line in file.lines() {
    }

    Ok(res)
}

fn main () {
    // Part 1
    let res_part1 = day_part1("day/input/actual.txt").expect("An error occurred!");
    println!("part 1 sum: {res_part1}");

    // Part 2
    let res_part2 = day_part2("day/input/actual.txt").expect("An error occurred!");
    println!("part 2 sum: {res_part2}")
}

#[test]
fn test_part1() {

    let res = day_part1("day/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 142);
}

#[test]
fn test_part2() {

    let res = day_part2("day/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 281);
}