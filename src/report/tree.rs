use serde::Serialize;
use std::{fmt::Display, fs, io, path::Path};

use super::utils::{count_lines_in_file, is_hidden, should_ignore};

#[derive(Debug, Serialize)]
struct FileReport {
    path: String,
    lines_count: usize,
}

#[derive(Debug, Serialize)]
pub struct Tree {
    root: String,
    directories: Option<Vec<Directory>>,
    files: Option<Files>,
}

impl Tree {
    pub fn from_path(path: &str, ignore: &[String]) -> io::Result<Self> {
        let root = path.to_string();
        let (directories, files) = Self::get_directory_contents(Path::new(path), ignore)?;

        let result = Self {
            root,
            directories,
            files,
        };

        Ok(result)
    }

    fn get_directory_contents(
        path: &Path,
        ignore: &[String],
    ) -> io::Result<(Option<Vec<Directory>>, Option<Files>)> {
        let mut directories = vec![];
        let mut files = vec![];

        for entry in fs::read_dir(path)? {
            let entry = entry?;

            if is_hidden(&entry) {
                continue;
            }

            if should_ignore(&entry, ignore) {
                continue;
            }

            let entry_path = entry.path();

            if entry_path.is_dir() {
                let sub_directory = Self::get_directory(&entry_path, ignore)?;
                directories.push(sub_directory);
            } else if entry_path.is_file() {
                let file_report = Self::get_file_report(&entry_path)?;
                files.push(file_report);
            }
        }

        Ok((
            if directories.is_empty() {
                None
            } else {
                Some(directories)
            },
            if files.is_empty() {
                None
            } else {
                Some(Files(files))
            },
        ))
    }

    fn get_directory(path: &Path, ignore: &[String]) -> io::Result<Directory> {
        let root = path.to_string_lossy().to_string();
        let (sub_directories, files) = Self::get_directory_contents(path, ignore)?;
        Ok(Directory {
            root,
            sub_directories,
            files,
        })
    }

    fn get_file_report(path: &Path) -> io::Result<FileReport> {
        let lines_count = count_lines_in_file(path)?;

        Ok(FileReport {
            path: path.to_string_lossy().to_string(),
            lines_count,
        })
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.root)?;
        if let Some(directories) = &self.directories {
            for directory in directories {
                write!(f, "{}", directory)?;
            }
        }
        if let Some(files) = &self.files {
            writeln!(f, "{}", files)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct Directory {
    root: String,
    sub_directories: Option<Vec<Directory>>,
    files: Option<Files>,
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    {}", self.root)?;

        if let Some(directories) = &self.sub_directories {
            for directory in directories.iter() {
                writeln!(f, "    {}", directory)?;
            }
        }

        if let Some(files) = &self.files {
            writeln!(f, "{}", files)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct Files(Vec<FileReport>);

impl Display for Files {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for file_report in &self.0 {
            writeln!(f, "    {}: {}", file_report.path, file_report.lines_count)?;
        }
        Ok(())
    }
}
