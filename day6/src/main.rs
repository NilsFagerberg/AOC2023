use std::io::Error;

fn day6_part1(file_path: &str) -> Result<i32, Error> {
    let file = std::fs::read_to_string(file_path)?;
    let mut res = 1;

    let mut lines = file.lines();
    let mut times = lines.next().expect("No times line in input!").split_ascii_whitespace();
    let mut distances = lines.next().expect("No distances line in input!").split_ascii_whitespace();

    times.next();
    distances.next();

    for (i, time) in times.enumerate() {
        let mut winning_ways = 0;
        let time_nbr = time.parse::<i32>().expect("NaN!");
        let distance_nbr = distances.next().unwrap().parse::<i32>().expect("NaN!");
        for j in 1..time_nbr {
            if ((time_nbr - j) * j > distance_nbr) {
                winning_ways += 1;
            }
        }
        res *= winning_ways;
    }


    Ok(res)
}

fn day6_part2(file_path: &str) -> Result<i32, Error> {

    let file = std::fs::read_to_string(file_path)?;
    let mut res = 0;

    let mut lines = file.lines();
    let mut times = lines.next().expect("No times line in input!").split_ascii_whitespace();
    let mut distances = lines.next().expect("No distances line in input!").split_ascii_whitespace();

    times.next();
    distances.next();

    let mut time = "".to_owned();
    let mut distance = "".to_owned();
    for substr in times {
        time.push_str(substr);
        distance.push_str(distances.next().expect("End of distance line!"));
    }

    let time_nbr = time.parse::<i64>().expect("NaN!");
    println!("{distance}");
    let distance_nbr = distance.parse::<i64>().expect("NaN!");

    for j in 1..time_nbr {
        if ((time_nbr - j) * j > distance_nbr) {
            res += 1;
        }
    }

    Ok(res)
}

fn main () {
    // Part 1
    let res_part1 = day6_part1("day6/input/actual.txt").expect("An error occurred!");
    println!("part 1 sum: {res_part1}");

    // Part 2
    let res_part2 = day6_part2("day6/input/actual.txt").expect("An error occurred!");
    println!("part 2 sum: {res_part2}")
}

#[test]
fn test_part1() {

    let res = day6_part1("day6/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 288);
}

#[test]
fn test_part2() {

    let res = day6_part2("day6/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 71503);
}