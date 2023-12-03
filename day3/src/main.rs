use std::io::Error;

fn day3_part1(file_path: &str) -> Result<usize, Error> {

    let file = std::fs::read_to_string(file_path)?;

    let mut schematic = Vec::new();

    for line in file.lines() {

    }

    Ok(2)
}

fn main() {
    // Part 1
    let res_part1 = day3_part1("day3/input/actual.txt").expect("An error occurred!");
    println!("part 1 sum: {res_part1}");

    // Part 2
    //let res_part2 = day2_part2("day2/input/actual.txt").expect("An error occurred!");
    //println!("part 2 sum: {res_part2}");
}

#[test]
fn test_part1() {

    let res = day3_part1("day3/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 8);
}

/*
#[test]
fn test_part2() {
    let res = day2_part2("day2/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 2286);
}
*/