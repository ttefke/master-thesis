use crate::uart::*;
use core::fmt::Write;

pub fn puts(out: &str) {
    print(out);
    data_send(UART_ID_Type_UART0_ID, b'\r');
    data_send(UART_ID_Type_UART0_ID, b'\n');
}

pub fn print(out: &str) {
    let ptr: &[u8] = out.as_bytes();
    for byte in ptr {
        data_send(UART_ID_Type_UART0_ID, *byte);
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $additional: expr)*) => {
        (|| {
            let mut s: heapless::String<96> = heapless::String::new();
            write!(s, $fmt $(, $additional)*).unwrap();
            crate::puts(&s);
        })();
    };
}

// Panic handler (required if no_std is used)
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::basic_init();
    println!(">>>>> PANIC <<<<<");
    let message = info.message().as_str();
    match message {
        Some(msg) => {
            puts(msg);
            if let Some(location) = info.location() {
                println!(">>>>> LOCATION <<<<<");
                println!("File: {}, line {}", location.file(), location.line());
            }
        }
        None => {
            // Check for mcause register
            if riscv::register::mcause::read().is_exception() {
                println!("A RISC-V exception occurred:");
                match riscv::register::mcause::read().code() {
                    0 => puts("Instruction address misaligned"),
                    1 => puts("Instruction access fault"),
                    2 => puts("Illegal instruction"),
                    3 => puts("Breakpoint"),
                    4 => puts("Load access misaligned"),
                    5 => puts("Load access fault"),
                    6 => puts("Store/AMO address misaligned"),
                    7 => puts("Store/AMO fault"),
                    8 => puts("Environment call from U-mode"),
                    11 => puts("Environment call from M-mode"),
                    _ => puts("No exception code found."),
                };
            } else {
                puts(
                    "No (valid) panic message given. Please note that formatting strings are unsupported here.",
                );
            }
        }
    }
    loop {}
}
