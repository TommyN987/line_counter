use serde::Serialize;
use std::{
    fmt::Display,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
};

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
    pub fn from_path(path: &str) -> io::Result<Self> {
        let root = path.to_string();
        let (directories, files) = Self::get_directory_contents(Path::new(path))?;

        let result = Self {
            root,
            directories,
            files,
        };

        Ok(result)
    }

    fn get_directory_contents(path: &Path) -> io::Result<(Option<Vec<Directory>>, Option<Files>)> {
        let mut directories = vec![];
        let mut files = vec![];

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                let sub_directory = Self::get_directory(&entry_path)?;
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

    fn get_directory(path: &Path) -> io::Result<Directory> {
        let root = path.to_string_lossy().to_string();
        let (sub_directories, files) = Self::get_directory_contents(path)?;
        Ok(Directory {
            root,
            sub_directories: sub_directories.map(Box::new),
            files,
        })
    }

    fn get_file_report(path: &Path) -> io::Result<FileReport> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let lines_count = reader.lines().count();

        Ok(FileReport {
            path: path.to_string_lossy().to_string(),
            lines_count,
        })
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.root)?;
        if let Some(directories) = &self.directories {
            for directory in directories {
                write!(f, "{}", directory)?;
            }
        }
        if let Some(files) = &self.files {
            write!(f, "{}", files)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct Directory {
    root: String,
    sub_directories: Option<Box<Vec<Directory>>>,
    files: Option<Files>,
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    {}\n", self.root)?;

        if let Some(directories) = &self.sub_directories {
            for directory in directories.iter() {
                write!(f, "    {}\n", directory)?;
            }
        }

        if let Some(files) = &self.files {
            write!(f, "{}\n", files)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct Files(Vec<FileReport>);

impl Display for Files {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for file_report in &self.0 {
            write!(f, "    {}: {}\n", file_report.path, file_report.lines_count)?;
        }
        Ok(())
    }
}
