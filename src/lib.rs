// src/lib.rs
/*
 * Core library for TreasuryManager
 */

use log::{info, error, debug};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

/// Custom result type for the library, wrapping a boxed error
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Process result structure, containing success, message, and optional data
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    /// Whether the process was successful
    pub success: bool,
    /// Message describing the outcome of the process
    pub message: String,
    /// Optional data returned from the process
    pub data: Option<serde_json::Value>,
}

/// TreasuryManager processor, handling data processing and statistics
#[derive(Debug)]
pub struct TreasuryManagerProcessor {
    /// Whether to enable verbose logging
    verbose: bool,
    /// Count of processed items
    processed_count: usize,
}

impl TreasuryManagerProcessor {
    /// Create a new processor instance with the specified verbosity level
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            processed_count: 0,
        }
    }

    /// Process a given string of data
    ///
    /// # Arguments
    ///
    /// * `data` - The string of data to process
    ///
    /// # Returns
    ///
    /// A `Result` containing the process result
    pub fn process(&mut self, data: &str) -> Result<ProcessResult> {
        if self.verbose {
            debug!("Processing data of length: {}", data.len());
        }

        // Simulate processing
        self.processed_count += 1;
        
        let result = ProcessResult {
            success: true,
            message: format!("Successfully processed item #{}", self.processed_count),
            data: Some(serde_json::json!({
                "length": data.len(),
                "processed_at": chrono::Utc::now().to_rfc3339(),
                "item_number": self.processed_count
            })),
        };

        Ok(result)
    }

    /// Get statistics about the processed items
    ///
    /// # Returns
    ///
    /// A JSON value containing the statistics
    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "processed_count": self.processed_count,
            "verbose": self.verbose
        })
    }
}

/// Main processing function
pub fn run(verbose: bool, input: Option<String>, output: Option<String>) -> Result<()> {
    // Initialize logging based on verbosity
    if verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }
    
    info!("Starting TreasuryManager processing");
    
    let mut processor = TreasuryManagerProcessor::new(verbose);
    
    // Read input
    let input_data = match input {
        Some(path) => {
            info!("Reading input from file: {}", path);
            fs::read_to_string(path)
        }
        None => {
            info!("No input file specified");
            Ok(String::new())
        }
    }?;

    // Process input data
    let results = process_input_data(&mut processor, input_data);

    // Output results
    output_results(output, results);

    Ok(())
}

/// Process a string of input data
///
/// # Arguments
///
/// * `processor` - The processor instance
/// * `input_data` - The string of input data
///
/// # Returns
///
/// A vector of process results
fn process_input_data(processor: &mut TreasuryManagerProcessor, input_data: String) -> Vec<ProcessResult> {
    let mut results = Vec::new();

    for line in input_data.lines() {
        let result = processor.process(line)?;
        results.push(result);
    }

    results
}

/// Output process results to a file
///
/// # Arguments
///
/// * `output` - The output file path
/// * `results` - The vector of process results
fn output_results(output: Option<String>, results: Vec<ProcessResult>) {
    if let Some(path) = output {
        info!("Outputting results to file: {}", path);
        let json = serde_json::to_string_pretty(&results)?;
        fs::write(path, json).expect("Failed to write output file");
    } else {
        info!("No output file specified");
    }
}