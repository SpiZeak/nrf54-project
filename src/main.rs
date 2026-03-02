#![no_main]
#![no_std]

use ariel_os::{
    debug::log::info,
    gpio::{Level, Output},
    time::Timer,
};

use ariel_os::hal::peripherals;

ariel_os::hal::define_peripherals!(LedPeripherals {
    led0: P2_09,
    led1: P1_10,
    led2: P2_07,
    led3: P1_14,
});

ariel_os::hal::group_peripherals!(Peripherals {
    leds: LedPeripherals,
});

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: Peripherals) {
    info!(
        "Blinking LEDs! Running on a {} board.",
        ariel_os::buildinfo::BOARD
    );

    let mut led0 = Output::new(peripherals.leds.led0, Level::Low);
    let mut led1 = Output::new(peripherals.leds.led1, Level::Low);
    let mut led2 = Output::new(peripherals.leds.led2, Level::Low);
    let mut led3 = Output::new(peripherals.leds.led3, Level::Low);

    let mut state = 0;

    loop {
        led0.set_low();
        led1.set_low();
        led2.set_low();
        led3.set_low();

        match state {
            0 => led0.set_high(),
            1 => led1.set_high(),
            2 => led2.set_high(),
            3 => led3.set_high(),
            _ => {}
        }

        state = (state + 1) % 4;
        Timer::after_millis(200).await;
    }
}
