/*
 * Copyright 2021-2024 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(about, author, version)]
pub(crate) struct Cli {
    /// Configuration filename
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config_filename: PathBuf,
}

pub(crate) fn parse() -> Cli {
    Cli::parse()
}
