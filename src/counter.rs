use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn count_lines_in_file(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}
