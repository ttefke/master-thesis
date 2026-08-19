use crate::println;
use core::fmt::Write;
use core::ptr;
use pac::sec_eng::RegisterBlock;

static mut TRNG_BUFFER: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
static mut TRNG_IDX: usize = 0;
const TRNG_LOOP_COUNTER: u8 = 17;

// Macro to update the random data obtained by the RNG
macro_rules! update_random_values {
    ($secEng: ident) => {
        /* 1./ Clear interrupt bits */
        $secEng.se_trng_0_ctrl_0().modify(|_r, w| {
            w.se_trng_0_int_clr_1t().set_bit();
            w.se_trng_0_trig_1t().clear_bit()
        });

        /* 2. Obtain new random values */
        unsafe {
            ptr::write_volatile(
                &mut TRNG_BUFFER[0],
                $secEng.se_trng_0_dout_0().read().bits(),
            );
            ptr::write_volatile(
                &mut TRNG_BUFFER[1],
                $secEng.se_trng_0_dout_1().read().bits(),
            );
            ptr::write_volatile(
                &mut TRNG_BUFFER[2],
                $secEng.se_trng_0_dout_2().read().bits(),
            );
            ptr::write_volatile(
                &mut TRNG_BUFFER[3],
                $secEng.se_trng_0_dout_3().read().bits(),
            );
            ptr::write_volatile(
                &mut TRNG_BUFFER[4],
                $secEng.se_trng_0_dout_4().read().bits(),
            );
            ptr::write_volatile(
                &mut TRNG_BUFFER[5],
                $secEng.se_trng_0_dout_5().read().bits(),
            );
            ptr::write_volatile(
                &mut TRNG_BUFFER[6],
                $secEng.se_trng_0_dout_6().read().bits(),
            );
            ptr::write_volatile(
                &mut TRNG_BUFFER[7],
                $secEng.se_trng_0_dout_7().read().bits(),
            );
        }
    };
}

/* RNG initialization function */
pub fn init() {
    let secEng: &RegisterBlock = unsafe { &*pac::SecEng::ptr() };
    trigger(secEng);
    feed(secEng);
    trigger(secEng);
    feed(secEng);
    crate::irq::enable(crate::irq::Interrupt::RNG);
}

/* Trigger generation of new random numbers */
fn trigger(secEng: &RegisterBlock) {
    /* 1. Check if RNG is busy */
    if secEng
        .se_trng_0_ctrl_0()
        .read()
        .se_trng_0_busy()
        .bit_is_set()
    {
        return;
    }

    /* 2. Reseed */
    unsafe {
        secEng
            .se_trng_0_ctrl_1()
            .write(|w| w.bits(ptr::read_volatile(&TRNG_BUFFER[0])));
        secEng
            .se_trng_0_ctrl_2()
            .write(|w| w.bits(ptr::read_volatile(&TRNG_BUFFER[1])));
    }

    /* 3. Enable RNG if not enabled yet */
    secEng.se_trng_0_ctrl_0().modify(|_r, w| {
        w.se_trng_0_int_set_1t().set_bit();
        w.se_trng_0_int_clr_1t().set_bit();
        w.se_trng_0_en().set_bit();
        w.se_trng_0_trig_1t().set_bit()
    });
}

/* Feed function */
fn feed(secEng: &RegisterBlock) {
    /* 1. Wait until RNG is no longer busy */
    while secEng
        .se_trng_0_ctrl_0()
        .read()
        .se_trng_0_busy()
        .bit_is_set()
    {}

    /* 2. Update random data */
    update_random_values!(secEng);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
fn RandomNumberGenerator_ISR() {
    crate::irq::disable(crate::irq::Interrupt::RNG);
    crate::irq::clear(crate::irq::Interrupt::RNG);

    /* Update random data */
    let secEng: &RegisterBlock = unsafe { &*pac::SecEng::ptr() };
    update_random_values!(secEng);

    crate::irq::enable(crate::irq::Interrupt::RNG);
}

fn get_random_word() -> u32 {
    unsafe {
        TRNG_IDX = TRNG_IDX % 8;
        if TRNG_IDX == 0 {
            trigger(&*pac::SecEng::ptr());
        }
        let retval = ptr::read_volatile(&TRNG_BUFFER[TRNG_IDX]);
        TRNG_IDX += 1;
        retval
    }
}

pub fn rand() -> u32 {
    let mut counter: u8 = 0;
    let mut value: u32;

    loop {
        value = get_random_word();
        if counter > TRNG_LOOP_COUNTER {
            println!("Rand: could not get random number!");
            break;
        }
        // First values we will receive after restart are zeroes, loop until we get something else
        if value != 0 {
            break;
        }
        counter += 1;
    }
    value >>= 1; // This is the flaw in the original RNG
    value
}

pub fn rand_stream<const N: usize>(vec: &mut heapless::Vec<u32, N>) {
    let cap = vec.capacity();
    for i in 1..cap {
        vec[i] = rand();
    }
}
