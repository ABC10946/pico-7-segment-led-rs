//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    bsp_pins,
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut a_pin = pins.gpio0.into_push_pull_output();
    let mut b_pin = pins.gpio1.into_push_pull_output();
    let mut c_pin = pins.gpio2.into_push_pull_output();
    let mut d_pin = pins.gpio3.into_push_pull_output();
    let mut e_pin = pins.gpio4.into_push_pull_output();
    let mut f_pin = pins.gpio5.into_push_pull_output();
    let mut g_pin = pins.gpio6.into_push_pull_output();
    let mut dp_pin = pins.gpio7.into_push_pull_output();
    let mut one_digit_pin = pins.gpio8.into_push_pull_output();
    let mut second_digit_pin = pins.gpio9.into_push_pull_output();
    let mut third_digit_pin = pins.gpio10.into_push_pull_output();
    let mut fourth_digit_pin = pins.gpio11.into_push_pull_output();

    one_digit_pin.set_low().unwrap();
    second_digit_pin.set_low().unwrap();
    third_digit_pin.set_low().unwrap();
    fourth_digit_pin.set_low().unwrap();

    a_pin.set_high().unwrap();
    b_pin.set_high().unwrap();
    c_pin.set_high().unwrap();
    d_pin.set_high().unwrap();
    e_pin.set_high().unwrap();
    f_pin.set_high().unwrap();
    g_pin.set_high().unwrap();
    dp_pin.set_high().unwrap();

    loop {
        a_pin.set_low().unwrap();
        one_digit_pin.set_high().unwrap();
        delay.delay_ms(5);
        a_pin.set_high().unwrap();
        one_digit_pin.set_low().unwrap();

        b_pin.set_low().unwrap();
        second_digit_pin.set_high().unwrap();
        delay.delay_ms(5);
        b_pin.set_high().unwrap();
        second_digit_pin.set_low().unwrap();

        c_pin.set_low().unwrap();
        third_digit_pin.set_high().unwrap();
        delay.delay_ms(5);
        c_pin.set_high().unwrap();
        third_digit_pin.set_low().unwrap();

        d_pin.set_low().unwrap();
        fourth_digit_pin.set_high().unwrap();
        delay.delay_ms(5);
        d_pin.set_high().unwrap();
        fourth_digit_pin.set_low().unwrap();
    }
}

// End of file
