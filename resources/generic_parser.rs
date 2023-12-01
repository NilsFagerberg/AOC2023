fn parse_data<T: FromStr>(s: &str, separator: char) -> Option<T> {
    let splits = s.split(separator);
    for split in splits {
        match T::from_str(&split) {
            Ok(x) => return Some(x),
            _ => continue,
        };
    }
    return None
}

fn main () {
    let file = File::open(file_path)?;
    let buffread = BufReader::new(file);
    for line in buffread.lines() {
        let l = line?;
        let val = parse_data::<i32>(&l, ' ');
        if val.is_some() {
            println!("found int: {}", val.unwrap());
        }
    }
}