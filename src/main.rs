#![feature(impl_trait_in_assoc_type)]
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// Required for initializing `CORE1`
#![allow(static_mut_refs)]

use embassy_executor;
use embassy_futures;

use embassy_rp;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals;
use embassy_rp::multicore;
use embassy_rp::usb;
use embassy_rp::gpio;

// Required for `assign_resources!` macro
use embassy_rp::Peri;

use embassy_time::{Duration, Timer};

use static_cell::StaticCell;

use {defmt_rtt as _, panic_probe as _};

static mut CORE1_STACK: multicore::Stack<4096> = multicore::Stack::new();
static mut CORE1_EXEC: StaticCell<embassy_executor::Executor> = StaticCell::new();

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<peripherals::USB>;
});

assign_resources::assign_resources! {
    led: LedResources {
        pin: PIN_25
    },
    flash: FlashResources {
        flash: FLASH,
        dma: DMA_CH0
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());
    let r = split_resources!(p);

    // Give program a moment before starting fully
    Timer::after_millis(1000).await;
    defmt::info!("Starting program");

    // Spawn the LED task on the second core
    multicore::spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor = unsafe {
                CORE1_EXEC.init(embassy_executor::Executor::new())
            };
            executor.run(|spawner| spawner.spawn(led_task(r.led).unwrap()));
        }
    );

    // Loop forever on primary core
    defmt::info!("Entering main loop");
    loop {
        Timer::after(Duration::from_millis(1000)).await;
        defmt::info!("Looping");
    }
}

#[embassy_executor::task]
async fn led_task(led: LedResources) {
    let mut led = gpio::Output::new(led.pin, gpio::Level::Low);

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(250)).await;

        led.set_low();
        Timer::after(Duration::from_millis(750)).await;
    }
}
