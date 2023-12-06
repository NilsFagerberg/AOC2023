use std::{io::Error, collections::HashMap, cmp::{min, max}};

fn day3_part1(file_path: &str) -> Result<usize, Error> {

    let file = std::fs::read_to_string(file_path)?;

    let mut sum = 0;

    let mut potential_parts: Vec<HashMap<usize, String>> = Vec::new();
    let mut symbols: Vec<HashMap<usize, String>> = Vec::new();

    // line e.g. "......832....@..........*.951*....984*111..801"
    for (i, line) in file.lines().enumerate() {
        let splits = line.split('.').filter(|s|!s.is_empty());

        let mut potential_parts_line: HashMap<usize, String> = HashMap::new();
        let mut symbols_line: HashMap<usize, String> = HashMap::new();
        
        // split e.g. "832" or "984*111"
        for split in splits {
            let mut found_digits = String::new();

            for c in split.chars() {
                if c.is_ascii_digit() {
                    found_digits.push(c);
                } else {
                    if !found_digits.is_empty() {
                        // println!("number found: {}", found_digits);
                        for found in line.match_indices(found_digits.as_str()) {
                            potential_parts_line.insert(found.0, found.1.to_string());
                        }
                        found_digits.clear();
                    }
                    for found in line.match_indices(c) {
                        // println!("Symbol found: {c}");
                        symbols_line.insert(found.0, found.1.to_string());
                    }
                }
            }
            if !found_digits.is_empty() {
                // println!("number found: {}", found_digits);
                for found in line.match_indices(found_digits.as_str()) {
                    potential_parts_line.insert(found.0, found.1.to_string());
                }
            }
        }

        potential_parts.push(potential_parts_line);
        symbols.push(symbols_line);
        /*
        for entry in potential_parts_line {
            println!("potential parts found for line {i}: {}, {}", entry.0, entry.1);
        }
        for entry in symbols_line {
            println!("symbols found for line {i}: {}, {}", entry.0, entry.1);
        }
        */
    } // Parsing done

    let lines: Vec<&str> = file.lines().collect();
    assert_eq!(potential_parts.len(), lines.len());

    for (i, parts_per_line) in potential_parts.iter().enumerate() {
        'next_part: for potential_part in parts_per_line {

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
                    continue 'next_part;
                }
            }
            // skip checking "below" last line
            if i < potential_parts.len() {
                let relevant_substr = &lines[i + 1][search_start..search_end];
                if relevant_substr.contains(|c: char|!c.is_ascii_digit() && c != '.') {
                    sum += part_number;
                    continue 'next_part;
                }
            }

            if search_start != 0 {
                let first = lines[i].chars().nth(search_start).expect("OOB!");
                if (!first.is_ascii_digit() && first != '.') {
                    sum += part_number;
                    continue 'next_part;
                }
            }
            if search_end != lines[i].len() {
                let last = lines[i].chars().nth(search_end).expect("OOB!");
                if (!last.is_ascii_digit() && last != '.') {
                    sum += part_number;
                }
            }
        }
    }

    Ok(sum)
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
    assert_eq!(res.unwrap(), 4361);
}

/*
#[test]
fn test_part2() {
    let res = day2_part2("day2/input/test.txt");

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 2286);
}
*/