use core::ptr::read_volatile;

const CLIC_MTIME_REGISTER: u32 = 0x0200_BFF8;
const TICKS_PER_US: u32 = 10;

pub fn delay_us(us: u32) {
    unsafe {
        // Read start time
        let mtime = CLIC_MTIME_REGISTER as *mut u32;
        let start = read_volatile(mtime);

        // Compute end time
        let ticks = us * TICKS_PER_US;
        let end = start.checked_add(ticks);

        // Busy waiting until time is over
        match end {
            Some(end) => loop {
                let now = read_volatile(mtime);
                if end <= now {
                    break;
                }
            },
            None => {
                let end = u32::MAX - start;
                loop {
                    let now = read_volatile(mtime);
                    if now >= end {
                        break;
                    }
                }
            }
        }
    }
}
