use crate::containers::*;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use nistrs::prelude::*;

/* Read the bits stored in a file */
fn read_data(path: String) -> Vec<u8> {
    /* Vector to store the extracted bytes */
    let mut bytes: Vec<u8> = Vec::new();

    /* Read lines */
    let file_string: Result<String, std::io::Error> = read_to_string(path);
    match file_string {
        Ok(file_string) => {
            /* Split into lines */
            let lines: Vec<String> = file_string.lines().map(String::from).collect();

            /* Iterate over lines */
            for line in lines.into_iter() {
                /* Read data from text */
                let mut dws: Vec<u32> = Vec::new();

                /* Get individual bit strings (double words) */
                let bitstring = string_to_bits(line);
                match bitstring {
                    Some(bitstring) => dws.push(bitstring),
                    None => {
                        println!("Could not read captured data part");
                        std::process::exit(255);
                    }
                }

                /* Split double words into bytes */
                for dw in dws {
                    /* Double word to bytes (little endian) */
                    let dw_split: [u8; 4] = u32::to_le_bytes(dw.to_le());

                    /* Push bytes into vector */
                    for byte in dw_split {
                        bytes.push(byte);
                    }
                }
            }

            //println!("Read {} bits", bytes.len() * 8);
        }
        Err(err) => {
            println!("Could not read file: {}", err)
        }
    }
    bytes
}

pub fn evaluate(path: String, out_path: String) {
    /* 1. Read data */
    let data = read_data(path);

    /* 2. Check len */
    if data.len() > 0 {
        /* 3. Convert data to  nistrs bits */
        let data = BitsData::from_binary(data);

        /* 4. Perform statistical tests */
        /* 4.1. Monobit frequency test */
        let monobit_frequency = frequency_test(&data).unwrap().1;

        /* 4.2. Frequency within a block */
        // No clear m recommendation
        let block_frequency = block_frequency_test(&data, 16_384).unwrap().1;
        //let block_frequency = block_frequency_test(&data, 4_194_304).unwrap().1;

        /* 4.3. Runs test */
        let runs = runs_test(&data).unwrap().1;

        /* 4.4. Longest run of ones */
        let longest_run = longest_run_of_ones_test(&data).unwrap().1;

        /* 4.5. Binary matrix rank */
        let binary_rank = rank_test(&data).unwrap().1;

        /* 4.6. DFT test */
        let dft = fft_test(&data).unwrap().1;

        /* 4.7. Non-overlapping template matching-test */
        let mut non_overlapping: Vec<f64> = Vec::new();
        // Recommendation: m = 9 or 10
        let result = non_overlapping_template_test(&data, 9).unwrap();
        for r in result {
            non_overlapping.push(r.1);
        }

        /* 4.8. Overlapping template matching test */
        // Recommendation: m = 9 or 10
        let mut overlapping: Vec<f64> = Vec::new();
        let result = overlapping_template_test(&data, 9).unwrap();
        for r in result {
            overlapping.push(r.1);
        }

        /* 4.9. Universal statistical test */        
        let mut universal: Vec<f64> = Vec::new();
        let result = universal_test(&data).unwrap();
        for r in result {
            universal.push(r.1);
        }

        /* 4.10. Linear complexity test */
        /* m increased to 5000 */
        // Recommendation: 500 <= m <= 5000
        let linear_complexity = linear_complexity_test(&data, 5000).unwrap().1;

        /* 4.11. Serial test */
        let mut serial: Vec<f64> = Vec::new();
        // Recommendation: m < floor(log2(n)) - 2
        let result = serial_test(&data,  8).unwrap();
        for r in result {
            serial.push(r.1);
        }

        /* 4.12. Approximate entropy test */
        // Recommendation: m < floor(log2(n)) - 5
        let approximate_entropy = approximate_entropy_test(&data, 8).unwrap().1;

        /* 4.13. Cumulative sums test */
        let mut cusum: Vec<f64> = Vec::new();
        let result = cumulative_sums_test(&data).unwrap();
        for r in result {
            cusum.push(r.1);
        }

        /* 4.14. Random excursions test */
        let mut random_excursions: Vec<f64> = Vec::new();
        let result = random_excursions_test(&data);
        match result {
            Ok(_) => {
                for r in result.unwrap() {
                    random_excursions.push(r.1);
                }
            }
            Err(err) => {
                // Random excursions failed
                random_excursions = vec![0.0, 0.0, 0.0, 0.0,
                                            0.0, 0.0, 0.0, 0.0];
                println!("{}", err);
            }
        }

        /* 4.15. Random excursions variant test */
        let mut random_excursions_variant: Vec<f64> = Vec::new();
        let result = random_excursions_variant_test(&data);
        match result {
            Ok(_) => {
                for r in result.unwrap() {
                    random_excursions_variant.push(r.1);
                }
            }
            Err(err) => {
                random_excursions_variant = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                                0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
                println!("{}", err);
            }
        }

        /* Create data structure */
        let test_results = TestResults{
            monobit_frequency,
            block_frequency,
            runs,
            longest_run,
            binary_rank,
            dft,
            non_overlapping,
            overlapping,
            universal,
            linear_complexity,
            serial,
            approximate_entropy,
            cusum,
            random_excursions,
            random_excursions_variant
        };

        // Serialize to JSON
        let serialized = serde_json::to_string_pretty(&test_results);
        match serialized {
            Ok(serialized) => {
                let file = File::create(out_path);
                match file {
                    Ok(mut file) => {
                        let result = file.write_all(serialized.as_bytes());
                        match result {
                            Ok(_) => (),
                            Err(err) => {
                                println!("Could not write data: {}", err);
                            }
                        }
                    },
                    Err(err) => {
                        println!("Can ont create file to store results: {}", err);
                    }
                }
            },
            Err(err) => {
                println!("Can not serialize data: {}", err);
            }
        }
    }

}
