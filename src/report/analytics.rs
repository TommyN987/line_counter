use std::fmt::Display;

use serde::Serialize;
use walkdir::WalkDir;

use crate::counter::count_lines_in_file;

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
    pub fn process(path: &str) -> Self {
        let mut result = Analytics::default();

        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .for_each(|entry| {
                let file_path = entry.path();
                if file_path.is_file() {
                    match count_lines_in_file(file_path.to_string_lossy().as_ref()) {
                        Ok(lines_count) => match lines_count {
                            0..=99 => result.less_than_100 += 1,
                            100..=500 => result.between_100_and_500 += 1,
                            501..=1000 => result.between_1001_and_2000 += 1,
                            1001..=2000 => result.between_1001_and_2000 += 1,
                            2001..=3000 => result.between_1001_and_2000 += 1,
                            3001..=4000 => result.between_1001_and_2000 += 1,
                            4001..=5000 => result.between_1001_and_2000 += 1,
                            5001..=6000 => result.between_1001_and_2000 += 1,
                            6001..=7000 => result.between_1001_and_2000 += 1,
                            7001..=8000 => result.between_1001_and_2000 += 1,
                            _ => result.more_than_8000 += 1,
                        },
                        Err(err) => {
                            eprintln!("Error reading {}: {}", file_path.display(), err);
                        }
                    }
                }
            });

        result
    }
}

impl Display for Analytics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "                Analytics                 \n")?;
        write!(f, "Number of lines:          Number of files:\n")?;
        write!(f, "< 100                     {}\n", self.less_than_100)?;
        write!(
            f,
            "100 ~ 500                 {}\n",
            self.between_100_and_500
        )?;
        write!(
            f,
            "501 ~ 1000                {}\n",
            self.between_501_and_1000
        )?;
        write!(
            f,
            "1001 ~ 2000               {}\n",
            self.between_1001_and_2000
        )?;

        write!(
            f,
            "2001 ~ 3000               {}\n",
            self.between_2001_and_3000
        )?;

        write!(
            f,
            "3001 ~ 4000               {}\n",
            self.between_3001_and_4000
        )?;
        write!(
            f,
            "4001 ~ 5000               {}\n",
            self.between_4001_and_5000
        )?;
        write!(
            f,
            "5001 ~ 6000               {}\n",
            self.between_5001_and_6000
        )?;
        write!(
            f,
            "6001 ~ 7000               {}\n",
            self.between_6001_and_7000
        )?;
        write!(
            f,
            "7001 ~ 8000               {}\n",
            self.between_7001_and_8000
        )?;
        write!(f, "> 8000                    {}\n", self.more_than_8000)?;
        Ok(())
    }
}
