//! Test the serial interface
//!
//! This example requires you to short (connect) the TX and RX pins.
#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

extern crate stm32l432xx_hal as hal;


use hal::prelude::*;
use hal::stm32l4::stm32l4x2;
use hal::tsc::Tsc;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let p = stm32l4x2::Peripherals::take().unwrap();
    // let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    // let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let _clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (clocks run at nearly the maximum frequency)
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    // let mut delay = Delay::new(cp.SYST, clocks);
    let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let sample_pin = gpiob.pb4.into_touch_sample(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let mut c1 = gpiob.pb5.into_touch_channel(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let mut c2 = gpiob.pb6.into_touch_channel(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    // let mut c3 = gpiob.pb7.into_touch_channel(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    
    // , (c1, c2, c3) 
    let tsc = Tsc::tsc(p.TSC, sample_pin, &mut rcc.ahb1);

    tsc.start(&mut c1);
    let baseline = tsc.wait().unwrap();
    let threshold = (baseline / 100) * 60;
    loop {
        tsc.start(&mut c1);
        let touched = tsc.wait().unwrap();
        tsc.start(&mut c2);
        let _touched_c2 = tsc.wait().unwrap();
        if touched < threshold {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
