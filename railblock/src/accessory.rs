use crate::can_id::Accessory;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use esp_hal::gpio::{Level, Output, OutputConfig};

pub static ACCESSORY_CHANNEL: Channel<CriticalSectionRawMutex, Accessory, 4> = Channel::new();

pub fn init(accessory: [impl esp_hal::gpio::OutputPin + 'static; 7], spawner: &Spawner) {
    let accessory_output =
        accessory.map(|pin| Output::new(pin, Level::Low, OutputConfig::default()));
    spawner.spawn(accessory_task(accessory_output)).unwrap();
}

#[embassy_executor::task]
async fn accessory_task(mut accessory: [Output<'static>; 7]) {
    loop {
        let recv = ACCESSORY_CHANNEL.receive().await;
        match recv.power {
            0 => accessory[recv.position as usize].set_low(),
            _ => accessory[recv.position as usize].set_high(),
        }
    }
}
