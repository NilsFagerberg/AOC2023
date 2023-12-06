use std::{io::Error, collections::BTreeMap, cmp::{min, max}};

fn day3_part1(file_path: &str) -> Result<usize, Error> {

    let file = std::fs::read_to_string(file_path)?;

    let mut sum = 0;

    let mut potential_parts: Vec<BTreeMap<usize, String>> = Vec::new();
    let mut symbols: Vec<BTreeMap<usize, String>> = Vec::new();

    // line e.g. "......832....@..........*.951*....984*111..801"
    for (i, line) in file.lines().enumerate() {

        let mut potential_parts_line: BTreeMap<usize, String> = BTreeMap::new();
        let mut symbols_line: BTreeMap<usize, String> = BTreeMap::new();

        let mut found_digits = String::new();
        let mut digit_ix = 0;

        for (j, c) in line.chars().enumerate() {
            if (c.is_ascii_digit()) {
                if (found_digits.is_empty()) {
                    digit_ix = j;
                }
                found_digits.push(c);
                continue;
            }
            if (!found_digits.is_empty()) {
                potential_parts_line.insert(digit_ix, found_digits.clone());
                digit_ix = 0;
                found_digits.clear();
            }
            if (c == '.') {
                continue;
            }
            symbols_line.insert(j, c.to_string());

        }
        if (!found_digits.is_empty()) {
            potential_parts_line.insert(digit_ix, found_digits.clone());
            digit_ix = 0;
            found_digits.clear();
        }
        
        potential_parts.push(potential_parts_line);
        symbols.push(symbols_line);

    } // Parsing done

    let lines: Vec<&str> = file.lines().collect();
    assert_eq!(potential_parts.len(), lines.len());

    for (i, parts_per_line) in potential_parts.iter().enumerate() {

        for potential_part in parts_per_line {
            let temp = potential_part.1.len() + potential_part.0 + 1;
            let part_number = potential_part.1.parse::<usize>().expect("NaN!");

            let search_start = match potential_part.0 {
                0 => 0,
                x => x - 1
            };
            let search_end = min(lines[i].len(), temp);

            // skip checking "above" first line
            if i > 0 {
                let relevant_substr = &lines[i - 1][search_start..search_end];
                if relevant_substr.contains(|c: char|!c.is_ascii_digit() && c != '.') {
                    sum += part_number;
                    continue;
                }
            }
            // skip checking "below" last line
            if i < potential_parts.len() - 1 {
                let relevant_substr = &lines[i + 1][search_start..search_end];
                if relevant_substr.contains(|c: char|!c.is_ascii_digit() && c != '.') {
                    sum += part_number;
                    continue;
                }
            }

            let relevant_substr = &lines[i][search_start..search_end];
            if (relevant_substr.starts_with(|c: char|!c.is_ascii_digit() && c != '.')) {
                sum += part_number;
                continue;
            }

            if (relevant_substr.ends_with(|c: char|!c.is_ascii_digit() && c != '.')) {
                sum += part_number;
                continue;
            }
        }
    }

    Ok(sum)
}

fn day3_part2(file_path: &str) -> Result<usize, Error> {

    let file = std::fs::read_to_string(file_path)?;

    let mut sum = 0;

    let mut potential_parts: Vec<BTreeMap<usize, String>> = Vec::new();
    let mut symbols: Vec<BTreeMap<usize, String>> = Vec::new();

    // line e.g. "......832....@..........*.951*....984*111..801"
    for (i, line) in file.lines().enumerate() {

        let mut potential_parts_line: BTreeMap<usize, String> = BTreeMap::new();
        let mut symbols_line: BTreeMap<usize, String> = BTreeMap::new();

        let mut found_digits = String::new();
        let mut digit_ix = 0;

        for (j, c) in line.chars().enumerate() {
            if (c.is_ascii_digit()) {
                if (found_digits.is_empty()) {
                    digit_ix = j;
                }
                found_digits.push(c);
                continue;
            }
            if (!found_digits.is_empty()) {
                potential_parts_line.insert(digit_ix, found_digits.clone());
                digit_ix = 0;
                found_digits.clear();
            }
            if (c == '.') {
                continue;
            }
            if (c == '*') {
                symbols_line.insert(j, c.to_string());
            }

        }
        if (!found_digits.is_empty()) {
            potential_parts_line.insert(digit_ix, found_digits.clone());
            digit_ix = 0;
            found_digits.clear();
        }
        
        potential_parts.push(potential_parts_line);
        symbols.push(symbols_line);

    } // Parsing done

    let lines: Vec<&str> = file.lines().collect();

    for (i, gears_per_line) in symbols.iter().enumerate() {
        for gear in gears_per_line {
            
        }
    } 
    Ok(2)
}

fn main() {
    // Part 1
    let res_part1 = day3_part1("day3/input/actual.txt").expect("An error occurred!");
    println!("part 1 sum: {res_part1}");

    // Part 2
    //let res_part2 = day2_part2("day2/input/actual.txt").expect("An error occurred!");
    // println!("part 2 sum: {res_part2}");
}

#[test]
fn test_part1() {

    let res = day3_part1("day3/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 4361);
}


#[test]
fn test_part2() {
    let res = day3_part2("day3/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 467835);
}