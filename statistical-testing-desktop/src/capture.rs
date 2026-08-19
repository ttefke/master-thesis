use crate::containers::*;
use crc::Crc;
use indicatif::ProgressBar;
use serialport::{SerialPort, TTYPort};
use std::{fs, io::Read, io::Write, time::Duration, vec};

const FAILURE: [u8; 1] = [0x00];
const SUCCESS: [u8; 1] = [0xFF];

pub fn capture(device: String, baudrate: u32, path: String, dw: u32) {
    /* Construct data structure */
    let mut collection = CapturedDataCollection { items: Vec::new() };

    /* Set up CRC generator */
    const CRC: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);

    /* Create progressbar */
    let pb = ProgressBar::new(dw as u64);

    /* Connect to device via serial tty port */
    let port = serialport::new(&device, baudrate).open_native();

    /* Capture status */
    let mut stats = CaptureStats {
        counter: 0,
        failures: 0,
        last: String::from(""),
    };

    /* Check if connection succeeded */
    match port {
        Ok(mut port) => {
            /* Update timeout for connection */
            let timeout = port.set_timeout(Duration::from_secs(2));

            /* Check if timeout update was successful */
            match timeout {
                Ok(_) => {
                    /* Start capturing */
                    println!("Device found, beginning capturing.");

                    loop {
                        /* Check if we captured the requested amount of messages */
                        if stats.counter == dw {
                            break;
                        }

                        /* CAPTURE MESSAGE SECTION */
                        /* Construct vector that holds incoming data */
                        let mut incoming_data: Vec<u8> = Vec::new();

                        /* Wait until we get a '{' that indicates an incoming data stream */
                        loop {
                            let mut incoming_char: Vec<u8> = vec![0; 1];

                            /* Read incoming data */
                            let read = port.read(&mut incoming_char);
                            match read {
                                /* Could read incoming data */
                                Ok(_) => {
                                    /* Convert incoming data to string */
                                    match String::from_utf8(incoming_char.clone()) {
                                        /* Successful conversion */
                                        Ok(incoming) => {
                                            /* Append '{' to incoming data vector once we receive it and break
                                            this loop to continue capturing the rest of the message*/
                                            if incoming == "{" {
                                                incoming_data
                                                    .append(incoming_char.clone().as_mut());
                                                break;
                                            }
                                        }
                                        /* Could not convert incoming data */
                                        Err(_) => {
                                            println!("Could not start capturing");
                                            std::process::exit(255);
                                        }
                                    }
                                }
                                /* Could not read incoming data */
                                Err(_) => {
                                    println!("Could not read data. Is the device sending data?");
                                    std::process::exit(255);
                                }
                            }
                        }

                        /* Capture data until we get a '}' that indicates the end of a data stream */
                        loop {
                            let mut incoming_char: Vec<u8> = vec![0; 1];

                            /* Read incoming data */
                            let read = port.read(&mut incoming_char);
                            match read {
                                /* Successful read */
                                Ok(_) => {
                                    /* Append all incoming data to data vector */
                                    incoming_data.append(incoming_char.clone().as_mut());

                                    /* Check if last incoming byte was a '}' and break loop if this is true */
                                    match String::from_utf8(incoming_char.clone()) {
                                        Ok(last) => {
                                            if last == "}" {
                                                break;
                                            }
                                        }
                                        /* Could not decode incoming data */
                                        Err(_) => {
                                            // Fail transmission.
                                            fail_receipt(&mut port, &mut stats);

                                            // Do not retransmit as we do not know whether we actually reached the end of the message
                                            continue;
                                        }
                                    }
                                }
                                /* Could not read data */
                                Err(_) => {
                                    println!("Could not read data. Is the device sending data?");
                                    std::process::exit(255);
                                }
                            }
                        }

                        /* POST-PROCESS CAPTURED MESSAGE SECTION */
                        /* Serialize received data into CapturedData struct */
                        let received_data: Result<CapturedData, serde_json::Error> =
                            serde_json::from_slice(&incoming_data.trim_ascii());

                        /* Check if data could be deserialized */
                        match received_data {
                            /* Data is serialized */
                            Ok(received_data) => {
                                /* Obtain transmitted data */
                                let binary_string = received_data.clone().data;
                                let crc = received_data.clone().crc;

                                /* Check if data was transmitted correctly by checking CRC sum */
                                /* Create bit message from string message first */
                                let bits = string_to_bits(binary_string.clone());
                                match bits {
                                    /* Successful conversion into bits */
                                    Some(bits) => {
                                        /* Generate checksum in little endian */
                                        let bits: [u8; 4] = u32::to_le_bytes(bits.to_le());
                                        let checksum = CRC.checksum(&bits);

                                        /* Check if computed checksum match received checksum */
                                        if crc != format!("{:04x}", checksum) {
                                            //println!("Checksums do not match");
                                            fail_receipt(&mut port, &mut stats);
                                            continue;
                                        } else {
                                            // confirm successful transmission
                                            confirm_receipt(&mut port);

                                            /* Store captured data if it differs from last one
                                            If retransmissions happen, we might receive data twice.
                                            Hence we must filter out duplicates.
                                            The risk of receiving the same data twice legitimate is
                                            1 / 2^32 so we can neglect this. */
                                            if stats.last != binary_string {
                                                collection.items.push(received_data);
                                                stats.counter += 1;
                                                stats.last = binary_string;
                                            }
                                        }
                                    }
                                    /* Could not convert data (e.g. due to invalid string received) */
                                    None => {
                                        //println!("Invalid string data received");
                                        // attempt retransmission
                                        fail_receipt(&mut port, &mut stats);
                                        continue;
                                    }
                                }
                            }
                            /* Could not serialize data (due to transmission fail) */
                            Err(_) => {
                                //println!("Data transmission failed");
                                // attempt retransmission
                                fail_receipt(&mut port, &mut stats);
                                continue;
                            }
                        }

                        /* Increase progressbar */
                        pb.inc(1);
                    }

                    /* Store data as bitstream */
                    let mut continous_bits = String::new();

                    for item in  collection.items {
                        continous_bits += &item.data;
                        continous_bits += "\n";
                    }

                    match fs::write(path, continous_bits) {
                        /* Saved successfully */
                        Ok(_) => {
                            println!("Wrote captured data as raw string");
                        }
                        /* Error */
                        Err(error) => {
                            println!("Could not write raw data: {}", error.to_string());
                            std::process::exit(255);
                        }
                    }
                }
                /* Could not update timeout - show error message */
                Err(_) => {
                    println!("Could not set timeout for reading incoming data");
                    std::process::exit(255);
                }
            }
        }
        /* Connection failed - show error message */
        Err(_) => {
            println!("Could not connect to device on port {}", &device);
            std::process::exit(255);
        }
    }

    /* Finish progressbar */
    pb.finish_with_message("Done");

    /* Stats */
    println!(
        "Captured: {}, retransmissions: {},  failure percentage: {}%",
        stats.counter,
        stats.failures,
        stats.failures as f32 / stats.counter as f32 * 100.0
    );
}

// Confirm data receipt
fn confirm_receipt(port: &mut TTYPort) {
    let _result = port.write(&SUCCESS);
    match _result {
        Ok(_) => {}
        Err(err) => {
            println!("Can not send receipt message: {}", err);
        }
    }
}

// Indicate failure
fn fail_receipt(port: &mut TTYPort, stats: &mut CaptureStats) {
    let _result = port.write(&FAILURE);
    match _result {
        Ok(_) => {
            stats.failures += 1;
        }
        Err(err) => {
            println!("Can not send failure message: {}", err);
        }
    }
}
