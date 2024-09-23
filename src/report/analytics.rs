use serde::Serialize;
use std::{fmt::Display, fs, io, path::Path};

use super::utils::{count_lines_in_file, is_hidden, should_ignore};

#[derive(Debug, Default, Serialize)]
pub struct Analytics {
    less_than_100: usize,
    between_100_and_500: usize,
    between_501_and_1000: usize,
    between_1001_and_2000: usize,
    between_2001_and_3000: usize,
    between_3001_and_4000: usize,
    between_4001_and_5000: usize,
    between_5001_and_6000: usize,
    between_6001_and_7000: usize,
    between_7001_and_8000: usize,
    more_than_8000: usize,
}

impl Analytics {
    pub fn from_path(path: &str, ignore: &[String]) -> io::Result<Self> {
        let mut result = Self::default();
        result.visit_dirs(Path::new(path), ignore)?;
        Ok(result)
    }

    fn visit_dirs(&mut self, dir: &Path, ignore: &[String]) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;

                if is_hidden(&entry) {
                    continue;
                }

                if should_ignore(&entry, ignore) {
                    continue;
                }

                let path = entry.path();
                if path.is_dir() {
                    self.visit_dirs(&path, ignore)?;
                } else if path.is_file() {
                    match count_lines_in_file(&path) {
                        Ok(lines_count) => match lines_count {
                            0..=99 => self.less_than_100 += 1,
                            100..=500 => self.between_100_and_500 += 1,
                            501..=1000 => self.between_501_and_1000 += 1,
                            1001..=2000 => self.between_1001_and_2000 += 1,
                            2001..=3000 => self.between_2001_and_3000 += 1,
                            3001..=4000 => self.between_3001_and_4000 += 1,
                            4001..=5000 => self.between_4001_and_5000 += 1,
                            5001..=6000 => self.between_5001_and_6000 += 1,
                            6001..=7000 => self.between_6001_and_7000 += 1,
                            7001..=8000 => self.between_7001_and_8000 += 1,
                            _ => self.more_than_8000 += 1,
                        },
                        Err(err) => println!("Error readint {}: {}", path.display(), err),
                    }
                }
            }
        }
        Ok(())
    }
}

impl Display for Analytics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "                Analytics                 ")?;
        writeln!(f, "Number of lines:          Number of files:")?;
        writeln!(f, "< 100                     {}", self.less_than_100)?;
        writeln!(f, "100 ~ 500                 {}", self.between_100_and_500)?;
        writeln!(f, "501 ~ 1000                {}", self.between_501_and_1000)?;
        writeln!(
            f,
            "1001 ~ 2000               {}",
            self.between_1001_and_2000
        )?;

        writeln!(
            f,
            "2001 ~ 3000               {}",
            self.between_2001_and_3000
        )?;

        writeln!(
            f,
            "3001 ~ 4000               {}",
            self.between_3001_and_4000
        )?;
        writeln!(
            f,
            "4001 ~ 5000               {}",
            self.between_4001_and_5000
        )?;
        writeln!(
            f,
            "5001 ~ 6000               {}",
            self.between_5001_and_6000
        )?;
        writeln!(
            f,
            "6001 ~ 7000               {}",
            self.between_6001_and_7000
        )?;
        writeln!(
            f,
            "7001 ~ 8000               {}",
            self.between_7001_and_8000
        )?;
        writeln!(f, "> 8000                    {}", self.more_than_8000)?;
        Ok(())
    }
}
