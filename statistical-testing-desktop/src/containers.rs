/* Data containers used for captured data */

use serde::{Deserialize, Serialize};

pub struct CaptureStats {
    pub counter: u32,
    pub failures: u32,
    pub last: String,
}

/* Data structure to store a captured data frame*/
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CapturedData {
    pub data: String,
    pub crc: String,
}

/* Data structure to collect all stored captured data */
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CapturedDataCollection {
    pub items: Vec<CapturedData>,
}

/* Data structure to store the test results */
#[derive(Debug, Serialize)]
pub struct TestResults {
    pub monobit_frequency: f64,
    pub block_frequency: f64,
    pub runs: f64,
    pub longest_run: f64,
    pub binary_rank: f64,
    pub dft: f64,
    pub non_overlapping: Vec<f64>,
    pub overlapping: Vec<f64>,
    pub universal: Vec<f64>,
    pub linear_complexity: f64,
    pub serial: Vec<f64>,
    pub approximate_entropy: f64,
    pub cusum: Vec<f64>,
    pub random_excursions: Vec<f64>,
    pub random_excursions_variant: Vec<f64>
}

// Converts a string with length 32 consisting of '0' and '1' characters into the according u32 bitstring
pub fn string_to_bits(string: String) -> Option<u32> {
    // Check if string is a multiple of eight
    let len = string.len();
    if len != 32 {
        return None;
    }

    // Set up result variable
    let mut result: u32 = 0;

    // Populate array data
    for i in 0..len {
        // Extract next character from string
        let char = string.trim_ascii().chars().nth(i);

        // Check if character could be extracted
        match char {
            Some(char) => {
                // Check if character is a one => adjust result variable
                if char == '1' {
                    result = result | (1 << (31 - i));
                }
            }
            None => {
                // Could not extract character => return None (error value)
                return None;
            }
        }
    }

    // Return data in little endian
    Some(result.to_le())
}
