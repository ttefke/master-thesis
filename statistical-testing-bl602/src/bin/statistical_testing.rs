#![no_std]
#![no_main]

use bl602_hal::{basic_init, platform::*, println, sec::rng::rand, timer::delay_us};
use core::fmt::{Display, Formatter, Write};
use crc::Crc;

#[derive(Debug, Copy, Clone)]
struct RandomData {
    value: u32,
    crc: u16,
}

// Implement display trait for random data
// This is basically a simple JSON serialization
// Even though no JSON library is used this 'hacky' way is
// ok for such simple data structures, especially because
// the 'proper' ways add more complexity because we are in a no_std
// environment and the traditional libraries do not work here
impl Display for RandomData {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        // This prints the random value as 32 bit binary value.
        // Not to be confused with the trailing zero, this is a formatting
        // specifier, not an additional zero.
        // See https://doc.rust-lang.org/std/fmt/index.html
        write!(
            formatter,
            "{{\"data\": \"{:032b}\", \"crc\": \"{:04x}\"}}",
            self.value, self.crc
        )
    }
}

#[riscv_rt::entry]
fn main() -> ! {
    // Initialize system
    basic_init();

    // Initialize CRC checksum generator
    const CRC: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);

    loop {
        /* 1. Get random value in little endian*/
        let random_value = rand().to_le();

        /* 2. Compute CRC checksum in little endian */
        /* 2.1. Get byte representation of the random value
        See https://doc.rust-lang.org/core/primitive.u32.html#method.to_ne_bytes  for details*/
        let random_value_bytes: [u8; 4] = u32::to_le_bytes(random_value.to_le());

        /* 2.2. Compute CRC checksum */
        let checksum = CRC.checksum(&random_value_bytes);

        /* 3. Send data to PC */
        let data = RandomData {
            value: random_value,
            crc: checksum,
        };
        println!("{}", data);

        /* 4. Wait for confirmation */
        loop {
            let confirmation = bl602_hal::uart::data_receive(0);
            if confirmation == 0xFF {
                /* Success -> next number */
                break;
            } else {
                /* Failure, resend */
                println!("{}", data);
            }
        }

        // Delay
        delay_us(1_000);
    }
}
