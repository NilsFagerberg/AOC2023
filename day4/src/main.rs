use std::{io::Error, collections::HashSet};

fn day4_part1(file_path: &str) -> Result<i32, Error> {
    let file = std::fs::read_to_string(file_path)?;
    let mut res = 0;

    for line in file.lines() {
        let mut my_numbers: HashSet<usize> = HashSet::new();
        let mut winning_numbers: HashSet<usize> = HashSet::new();

        let mut matches = 0;

        let info = line.split_once(": ").expect("Invalid input! No card number found").1;
        let (my_numbers_list, winning_numbers_list) = info.split_once(" | ").expect("Invalid input! No delimeter found");

        for my_number_str in my_numbers_list.split_ascii_whitespace() {
            my_numbers.insert(my_number_str.parse::<usize>().expect("NaN!"));
        }

        for winning_number_str in winning_numbers_list.split_ascii_whitespace() {
            winning_numbers.insert(winning_number_str.parse::<usize>().expect("NaN!"));
        }

        for number in my_numbers {
            if winning_numbers.contains(&number) {
                matches += 1;
            }
        }

        if matches > 0 {
            res += 1 << (matches - 1);
        }
    }

    Ok(res)
}

fn day4_part2(file_path: &str) -> Result<i32, Error> {

    let file = std::fs::read_to_string(file_path)?;
    let mut res = 0;
    let num_cards = file.lines().count();

    let mut wins_array = vec![1; num_cards];

    for (i, line) in file.lines().enumerate() {
        let mut my_numbers: HashSet<usize> = HashSet::new();
        let mut winning_numbers: HashSet<usize> = HashSet::new();

        let mut matches = 0;

        let info = line.split_once(": ").expect("Invalid input! No card number found").1;
        let (my_numbers_list, winning_numbers_list) = info.split_once(" | ").expect("Invalid input! No delimeter found");

        for my_number_str in my_numbers_list.split_ascii_whitespace() {
            my_numbers.insert(my_number_str.parse::<usize>().expect("NaN!"));
        }

        for winning_number_str in winning_numbers_list.split_ascii_whitespace() {
            winning_numbers.insert(winning_number_str.parse::<usize>().expect("NaN!"));
        }

        for number in my_numbers {
            if winning_numbers.contains(&number) {
                matches += 1;
            }
        }
        for x in 0..matches {
            let ix = x + i;
            if (ix < num_cards) {
                wins_array[ix + 1] += wins_array[i];
            }
        }
    }

    res = wins_array.iter().sum();

    Ok(res)
}

fn main () {
    // Part 1
    let res_part1 = day4_part1("day4/input/actual.txt").expect("An error occurred!");
    println!("part 1 sum: {res_part1}");

    // Part 2
    let res_part2 = day4_part2("day4/input/actual.txt").expect("An error occurred!");
    println!("part 2 sum: {res_part2}")
}

#[test]
fn test_part1() {

    let res = day4_part1("day4/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 13);
}

#[test]
fn test_part2() {

    let res = day4_part2("day4/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 30);
}