use clap::{Parser, Subcommand, ValueEnum};
use std::{fmt::Display, str::FromStr};

use crate::error::{Error, Result};

/// A todo app
#[derive(Parser, Debug)]
#[clap(author = "Daniel Bolivar", version)]
pub struct Args {
    /// The todo app commands
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new todo item
    Add {
        /// The name of the todo item
        name: String,
        /// The number of days to start the todo item
        #[clap(long)]
        days_to_start: Option<u32>,
        /// The number of days before the deadline
        #[clap(long)]
        days_to_complete: Option<u32>,
        /// The weight/importance of the todo item
        #[clap(value_enum, long)]
        weight: Option<Weight>,
        /// Optional description for the todo item
        #[clap(long)]
        description: Option<String>,
    },
    /// Remove a todo item
    Remove {
        /// The name of the todo item to remove
        name: String,
    },
    /// Edit an existing todo item
    Edit {
        /// The name of the todo item to edit
        name: String,
        /// New name for the todo item
        #[clap(long)]
        new_name: Option<String>,
        /// New description for the todo item
        #[clap(long)]
        description: Option<String>,
        /// New weight for the todo item
        #[clap(value_enum, long)]
        weight: Option<Weight>,
        /// New start date (days from now)
        #[clap(long)]
        days_to_start: Option<u32>,
        /// New deadline (days from now)
        #[clap(long)]
        days_to_complete: Option<u32>,
    },
    /// Mark a todo item as complete
    Complete {
        /// The name of the todo item to mark as complete
        name: String,
    },
    /// List todo items
    List {
        /// Filter by weight
        #[clap(value_enum, long)]
        weight: Option<Weight>,
        /// Show only completed items
        #[clap(long)]
        completed: bool,
        /// Sort by deadline
        #[clap(long)]
        sort_by_deadline: bool,
        /// Sort by weight
        #[clap(long)]
        sort_by_weight: bool,
    },
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Weight {
    Low,
    Medium,
    High,
}

impl Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Weight::Low => write!(f, "low"),
            Weight::Medium => write!(f, "medium"),
            Weight::High => write!(f, "high"),
        }
    }
}

impl FromStr for Weight {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Weight::Low),
            "medium" => Ok(Weight::Medium),
            "high" => Ok(Weight::High),
            _ => Err(Error::WeightParse(s.to_string())),
        }
    }
}
