use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};

use crate::args::{Args, Structure, Target};
use crate::report::{Analytics, Full, Tree};
use clap::Parser;
use serde::Serialize;

mod args;
mod counter;
mod report;

#[derive(Debug, Serialize)]
enum Report {
    Analytics(Analytics),
    Tree(Tree),
    Full(Full),
}

impl Report {
    fn from_args(path: &str, structure: Structure) -> io::Result<Self> {
        match structure {
            Structure::Analytics => {
                let analytics = Analytics::process(path);
                Ok(Report::Analytics(analytics))
            }
            Structure::Tree => {
                let tree = Tree::from_path(path)?;
                Ok(Report::Tree(tree))
            }
            Structure::Full => {
                let analytics = Analytics::process(path);
                let tree = Tree::from_path(path)?;
                Ok(Report::Full(Full::new(analytics, tree)))
            }
        }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Analytics(analytics) => write!(f, "{}", analytics)?,
            Self::Tree(tree) => write!(f, "{}", tree)?,
            Self::Full(full) => {
                write!(f, "{}", full.analytics)?;
                write!(f, "{}", full.tree)?;
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let Args {
        path,
        target,
        structure,
    } = Args::parse();

    let result = Report::from_args(&path, structure)?;

    match target {
        Target::Stdout => println!("{}", result),
        Target::Json => {
            let mut file = File::create("result.json")?;
            let maybe_json = serde_json::to_string_pretty(&result);
            match maybe_json {
                Ok(json) => {
                    File::write_all(&mut file, json.as_bytes())?;
                    println!("Successfully written result into result.json");
                }
                Err(err) => {
                    println!("Error while serializing to JSON: {}", err);
                }
            }
        }
    };

    Ok(())
}
