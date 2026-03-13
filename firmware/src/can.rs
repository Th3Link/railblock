use crate::accessory;
use crate::can_id::{Accessory, CanId, Command};
use crate::config::{self, config};
use core::fmt::Write;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embedded_can::Frame;
use esp_hal::Async;
use esp_hal::gpio::{InputPin, OutputConfig, OutputPin};
use esp_hal::gpio::{Level, Output};
use esp_hal::twai::{self, EspTwaiFrame, TimingConfig, TwaiMode};
use esp_println::println;
use heapless::String;

pub static CAN_CHANNEL: Channel<CriticalSectionRawMutex, EspTwaiFrame, 32> = Channel::new();

pub async fn init(
    twai: esp_hal::peripherals::TWAI0<'static>,
    rx: impl InputPin + 'static,
    tx: impl OutputPin + 'static,
    standby: impl OutputPin + 'static,
    spawner: &Spawner,
) {
    const TC: TimingConfig = TimingConfig {
        baud_rate_prescaler: 80,
        sync_jump_width: 3,
        tseg_1: 15,
        tseg_2: 4,
        triple_sample: false,
    };

    const TWAI_BAUDRATE: twai::BaudRate = twai::BaudRate::Custom(TC);
    let device_id = config()
        .await
        .get_u8(config::Key::DeviceId)
        .await
        .unwrap_or(0);

    let twai_config = twai::TwaiConfiguration::new(twai, rx, tx, TWAI_BAUDRATE, TwaiMode::Normal);
    let twai = twai_config.into_async().start();
    while twai.is_bus_off() {
        println!("waiting for bus_off");
        embassy_time::Timer::after_millis(100).await;
    }
    let (rx, tx) = twai.split();
    let pin1_gpio = Output::new(standby, Level::Low, OutputConfig::default());
    spawner.spawn(can_send_task(tx)).unwrap();
    spawner.spawn(can_recieve_task(rx, device_id)).unwrap();
}

pub async fn dispatch(frame: &EspTwaiFrame, device_id: u8) {
    let id = match frame.id() {
        embedded_can::Id::Extended(id) => CanId::from(id),
        embedded_can::Id::Standard(id) => {
            println!("WARN: Ignoring standard ID: {:?}", id);
            return;
        }
    };
    if id.command == Command::Accessory as u8 {
        let a = Accessory::from(frame.data());
        if (a.loc_id & 0xFF) as u8 == device_id {
            println!("received accessory message");
            accessory::ACCESSORY_CHANNEL.send(a).await;
        }
    }
}

pub async fn send_can_message(command: Command, data: &[u8], rtr: bool) {
    let device_id = config().await.get_u8(config::Key::DeviceId).await.unwrap() as u16;
    let id: embedded_can::ExtendedId = CanId::new(0, command as u8, 0, device_id).into();
    let id: esp_hal::twai::ExtendedId = id.into();
    let frame = if rtr {
        EspTwaiFrame::new_remote(id, data.len()).unwrap()
    } else {
        EspTwaiFrame::new(id, data).unwrap()
    };

    CAN_CHANNEL.send(frame).await
}

fn log_frame(frame: &EspTwaiFrame) {
    let mut out: String<128> = String::new();

    // ID
    match frame.id() {
        embedded_can::Id::Standard(id) => {
            let _ = write!(out, "id: {:03X}", id.as_raw());
        }
        embedded_can::Id::Extended(id) => {
            let _ = write!(out, "id: {:08X}", id.as_raw());
        }
    }

    match frame.id() {
        embedded_can::Id::Extended(id) => {
            let id = CanId::from(id);
            let _ = write!(out, ", command: {:?}", id.command);
        }
        embedded_can::Id::Standard(id) => {
            println!("WARN: Ignoring standard ID: {:?}", id);
            return;
        }
    };

    // DLC
    let _ = write!(out, ", dlc: {}", frame.dlc());

    // Data
    let _ = write!(out, ", data: [");
    for (i, b) in frame.data().iter().enumerate() {
        if i > 0 {
            let _ = write!(out, " ");
        }
        let _ = write!(out, "{:02X}", b);
    }
    let _ = write!(out, "]");

    // Remote
    let _ = write!(out, ", is_remote: {}", frame.is_remote_frame());

    // Am Ende z. B. via defmt, rtt-target, log, oder println! ausgeben
    // (hier Beispiel mit defmt)
    println!("{}", out.as_str());
}

#[embassy_executor::task]
pub async fn can_recieve_task(mut rx: twai::TwaiRx<'static, Async>, device_id: u8) {
    println!("can_recieve_task started");
    loop {
        if let Ok(frame) = rx.receive_async().await {
            dispatch(&frame, device_id).await;
        } else {
            println!("recv error");
        }
    }
}

#[embassy_executor::task]
pub async fn can_send_task(mut tx: twai::TwaiTx<'static, Async>) {
    println!("can_send_task started");
    loop {
        let frame = CAN_CHANNEL.receive().await;
        tx.transmit_async(&frame).await.unwrap();
        log_frame(&frame);
    }
}
