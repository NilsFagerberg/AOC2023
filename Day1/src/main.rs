use std::collections::HashMap;
use std::io::Error;

fn day1_part1(file_path: &str) -> Result<i32, Error> {
    let file = std::fs::read_to_string(file_path)?;
    let mut sum = 0;
    for line in file.lines() {
        let first = line.find(|c: char | c.is_ascii_digit()).expect("no digit found by find");
        let last = line.rfind(|c: char | c.is_ascii_digit()).expect("no digit found by rfind");

        let mut num = line[first..first+1].to_owned();
        let last_str = &line[last..last+1];
        num.push_str(last_str);

        sum += num.parse::<i32>().expect("NaN!");
    }

    Ok(sum)
}

struct IxAndDigit {
    ix: usize,
    digit: i32
}

fn day1_part2(file_path: &str) -> Result<i32, Error> {

    let file = std::fs::read_to_string(file_path)?;
    let str_matches: HashMap<&str, i32> = HashMap::from([
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]);

    let mut sum = 0;

    for line in file.lines() {
        let mut first = IxAndDigit {
            ix: line.len(),
            digit: 0
        };

        let mut last = IxAndDigit {
            ix: 0,
            digit: 0
        };

        // Map first ASCII digit (if any) to first
        match line.find(|c: char | c.is_ascii_digit()) {
            Some(x) => first = IxAndDigit{ix: x, digit: line[x..x+1].parse::<i32>().expect("NaN!")},
            None => ()
        }

        // Map last ASCII digit (if any) to last
        match line.rfind(|c: char | c.is_ascii_digit()) {
            Some(x) => last = IxAndDigit{ix: x, digit: line[x..x+1].parse::<i32>().expect("NaN!")},
            None => ()
        }

        for key in str_matches.keys() {
            match line.find(key) {
                None => (),
                Some(x) => {
                    if (x < first.ix) {
                        first = IxAndDigit {ix: x, digit: str_matches[key]};
                    }
                }
            }
            match line.rfind(key) {
                None => (),
                Some(x) =>{
                    if (x > last.ix) {
                        last = IxAndDigit {ix: x, digit: str_matches[key]};
                    }
                }
            }
        }
        sum += 10*first.digit;
        sum += last.digit;
    }

    Ok(sum)
}

fn main () {
    // Part 1
    let res_part1 = day1_part1("Day1/input/actual.txt").expect("An error occurred!");
    println!("part 1 sum: {res_part1}");

    // Part 2
    let res_part2 = day1_part2("Day1/input/actual.txt").expect("An error occurred!");
    println!("part 2 sum: {res_part2}")
}

#[test]
fn test_part1() {

    let res = day1_part1("Day1/input/test_part1.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 142);
}

#[test]
fn test_part2() {

    let res = day1_part2("Day1/input/test_part2.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 281);
}