use clap::{Parser, ValueEnum};
use std::fmt::Display;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub path: String,
    #[arg(short, long, default_value_t = Target::Stdout)]
    pub target: Target,
    #[arg(short, long, default_value_t = Structure::Analytics)]
    pub structure: Structure,
    #[arg(short, long)]
    pub ignore: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Target {
    Stdout,
    Json,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stdout => write!(f, "stdout"),
            Self::Json => write!(f, "json"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Structure {
    Analytics,
    Tree,
    Full,
}

impl Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Analytics => write!(f, "analytics"),
            Self::Tree => write!(f, "tree"),
            Self::Full => write!(f, "full"),
        }
    }
}
