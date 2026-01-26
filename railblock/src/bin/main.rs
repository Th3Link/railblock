#![no_std]
#![no_main]

use core::panic::PanicInfo;
use embassy_executor::Spawner;
use embassy_time::Duration;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_hal_embassy::main;
use esp_println::println;
use railblock::button;
use railblock::can;
use railblock::cli;
use railblock::config;
use railblock::gpio_interrupt;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    esp_hal::system::software_reset();
}

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz));

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);
    println!("starting railblock");
    config::init().await;

    if let Some(device_id) = config::config().await.get_u8(config::Key::DeviceId).await {
        println!("loaded device_id {device_id}");
    }

    button::init(
        peripherals.GPIO33,
        peripherals.GPIO25,
        peripherals.GPIO26,
        &spawner,
    );
    gpio_interrupt::init(peripherals.IO_MUX);
    can::init(
        peripherals.TWAI0,
        peripherals.GPIO14,
        peripherals.GPIO13,
        &spawner,
    )
    .await;

    Timer::after(Duration::from_millis(2_000)).await;
    cli::init(
        peripherals.UART0,
        peripherals.GPIO3,
        peripherals.GPIO1,
        &spawner,
    )
    .await;

    loop {
        Timer::after(Duration::from_millis(3_000)).await;
    }
}
