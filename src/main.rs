// src/main.rs
/*
 * Main executable for TreasuryManager
 */

use clap::Parser;
use treasurymanager::{Result, run};

/// Command-line arguments parser
#[derive(Parser)]
#[command(version, about = "TreasuryManager - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Path to input file
    #[arg(short = 'i', long = "input")]
    input: Option<String>,
    
    /// Path to output file
    #[arg(short = 'o', long = "output")]
    output: Option<String>,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Cli::parse();
    
    // Run the application with parsed arguments
    run(args.verbose, args.input, args.output)
}