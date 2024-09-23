use std::{
    fs::{DirEntry, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

pub(super) fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub(super) fn should_ignore(entry: &DirEntry, ignore: &[String]) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| ignore.contains(&s.to_string()))
        .unwrap_or(false)
}

pub(super) fn count_lines_in_file(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}
