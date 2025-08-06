#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    // Best practice with error handling
   

    // Simple alternative for QEMU
    let _ = hprintln!("Loop started");
    
    loop {
        for _ in 0..1_000_000 { cortex_m::asm::nop(); }
        let _ = hprintln!("Heartbeat");
    }
}