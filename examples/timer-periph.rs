//! Start and stop a periodic peripheral timer.
//!
//! This example should run on all stm32f4xx boards but it was tested with
//! stm32f4-discovery board (model STM32F407G-DISC1).
//!
//! ```bash
//! cargo run --release --features stm32f407,rt  --example timer-periph
//! ```

#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use embedded_hal::timer::Cancel;
use hal::timer;
use hal::timer::Timer;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(24.MHz()).freeze();

    // Create a timer based on SysTick
    let mut timer = Timer::new(dp.TIM1, &clocks).start_count_down(1.Hz());

    hprintln!("hello!").unwrap();
    // wait until timer expires
    nb::block!(timer.wait()).unwrap();
    hprintln!("timer expired 1").unwrap();

    // the function syst() creates a periodic timer, so it is automatically
    // restarted
    nb::block!(timer.wait()).unwrap();
    hprintln!("timer expired 2").unwrap();

    // cancel current timer
    timer.cancel().unwrap();

    // start it again
    timer.start(1.Hz());
    nb::block!(timer.wait()).unwrap();
    hprintln!("timer expired 3").unwrap();

    timer.cancel().unwrap();
    let cancel_outcome = timer.cancel();
    assert_eq!(cancel_outcome, Err(timer::Error::Disabled));
    hprintln!("ehy, you cannot cancel a timer two times!").unwrap();
    // this time the timer was not restarted, therefore this function should
    // wait forever
    nb::block!(timer.wait()).unwrap();
    // you should never see this print
    hprintln!("if you see this there is something wrong").unwrap();
    panic!();
}
