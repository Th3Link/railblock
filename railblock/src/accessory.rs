use crate::can_id::Accessory;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_println::println;

pub static ACCESSORY_CHANNEL: Channel<CriticalSectionRawMutex, Accessory, 4> = Channel::new();

pub fn init(
    pin0: impl esp_hal::gpio::OutputPin + 'static,
    pin1: impl esp_hal::gpio::OutputPin + 'static,
    pin2: impl esp_hal::gpio::OutputPin + 'static,
    pin3: impl esp_hal::gpio::OutputPin + 'static,
    pin4: impl esp_hal::gpio::OutputPin + 'static,
    pin5: impl esp_hal::gpio::OutputPin + 'static,
    pin6: impl esp_hal::gpio::OutputPin + 'static,
    spawner: &Spawner,
) {
    let accessory_output = [
        Output::new(pin0, Level::Low, OutputConfig::default()),
        Output::new(pin1, Level::Low, OutputConfig::default()),
        Output::new(pin2, Level::Low, OutputConfig::default()),
        Output::new(pin3, Level::Low, OutputConfig::default()),
        Output::new(pin4, Level::Low, OutputConfig::default()),
        Output::new(pin5, Level::Low, OutputConfig::default()),
        Output::new(pin6, Level::Low, OutputConfig::default()),
    ];
    spawner.spawn(accessory_task(accessory_output)).unwrap();
}

#[embassy_executor::task]
async fn accessory_task(mut accessory: [Output<'static>; 7]) {
    println!("accessory task started");
    loop {
        let recv = ACCESSORY_CHANNEL.receive().await;
        println!(
            "received accessory powoer:{}, position:{}",
            recv.power, recv.position
        );
        match recv.power {
            0 => accessory[recv.position as usize].set_low(),
            _ => accessory[recv.position as usize].set_high(),
        }
    }
}
