//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead. If you have
    // a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here.
    let a0 = pins.gpio0.into_pull_up_input();
    let a1 = pins.gpio1.into_pull_up_input();
    let a2 = pins.gpio2.into_pull_up_input();
    let a3 = pins.gpio3.into_pull_up_input();

    let mut d0 = pins.gpio4.into_push_pull_output();
    let mut d1 = pins.gpio5.into_push_pull_output();
    let mut d2 = pins.gpio6.into_push_pull_output();
    let mut d3 = pins.gpio7.into_push_pull_output();
    let mut d4 = pins.gpio8.into_push_pull_output();
    let mut d5 = pins.gpio9.into_push_pull_output();
    let mut d6 = pins.gpio10.into_push_pull_output();
    let mut d7 = pins.gpio11.into_push_pull_output();

    let program: [u8; 16] = [
        0b10110011, 0b10110110, 0b10111100, 0b10111000, 0b10111000, 0b10111100, 0b10110110,
        0b10110011, 0b10110001, 0b11110000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
        0b00000000, 0b00000000,
    ];

    loop {
        let mut addr: usize = 0;
        addr |= if a0.is_high().unwrap() { 0x1 } else { 0x0 };
        addr |= if a1.is_high().unwrap() { 0x2 } else { 0x0 };
        addr |= if a2.is_high().unwrap() { 0x4 } else { 0x0 };
        addr |= if a3.is_high().unwrap() { 0x8 } else { 0x0 };

        let d = program[addr];
        if d & 0b00000001 == 0b00000001 {
            d0.set_high().unwrap();
        } else {
            d0.set_low().unwrap();
        }
        if d & 0b00000010 == 0b00000010 {
            d1.set_high().unwrap();
        } else {
            d1.set_low().unwrap();
        }
        if d & 0b00000100 == 0b00000100 {
            d2.set_high().unwrap();
        } else {
            d2.set_low().unwrap();
        }
        if d & 0b00001000 == 0b00001000 {
            d3.set_high().unwrap();
        } else {
            d3.set_low().unwrap();
        }
        if d & 0b00010000 == 0b00010000 {
            d4.set_high().unwrap();
        } else {
            d4.set_low().unwrap();
        }
        if d & 0b00100000 == 0b00100000 {
            d5.set_high().unwrap();
        } else {
            d5.set_low().unwrap();
        }
        if d & 0b01000000 == 0b01000000 {
            d6.set_high().unwrap();
        } else {
            d6.set_low().unwrap();
        }
        if d & 0b10000000 == 0b10000000 {
            d7.set_high().unwrap();
        } else {
            d7.set_low().unwrap();
        }
    }
}

// End of file
