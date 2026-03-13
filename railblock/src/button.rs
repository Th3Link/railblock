use crate::can::send_can_message;
use crate::can_id::Command;
use crate::can_id::S88Event;
use crate::config;
use crate::config::config;
use crate::gpio_interrupt::GpioChannel;
use crate::gpio_interrupt::register_gpio_handler;
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_time::{Duration, Timer};
use esp_hal::gpio::Event;
use esp_hal::gpio::Input;
use esp_hal::gpio::InputConfig;
use esp_hal::gpio::Pull;

const DEBOUNCE_TIME: Duration = Duration::from_millis(10); // Entprellzeit

pub struct Button {
    state: bool,
}

pub fn init(
    button0: impl esp_hal::gpio::InputPin + 'static,
    button1: impl esp_hal::gpio::InputPin + 'static,
    button2: impl esp_hal::gpio::InputPin + 'static,
    spawner: &Spawner,
) {
    let config = InputConfig::default().with_pull(Pull::Up);
    let mut button0 = Input::new(button0, config);
    let mut button1 = Input::new(button1, config);
    let mut button2 = Input::new(button2, config);

    button0.listen(Event::AnyEdge);
    button1.listen(Event::AnyEdge);
    button2.listen(Event::AnyEdge);

    let ch0 = register_gpio_handler(button0).unwrap();
    let ch1 = register_gpio_handler(button1).unwrap();
    let ch2 = register_gpio_handler(button2).unwrap();

    spawner.spawn(run(0, ch0)).unwrap();
    spawner.spawn(run(1, ch1)).unwrap();
    spawner.spawn(run(2, ch2)).unwrap();
}

impl Button {
    pub async fn iterate(&mut self, index: u16, channel: &GpioChannel) {
        let debounce_time = Timer::after(DEBOUNCE_TIME);
        let next_state = channel.receive();
        match select(next_state, debounce_time).await {
            Either::First(_) => {
                //bounces
                return;
            }
            Either::Second(_) => {
                // debounced go on
            }
        }

        let next_state = channel.receive().await;
        if next_state != self.state {
            let event = S88Event {
                node_id: config().await.get_u8(config::Key::DeviceId).await.unwrap() as u16,
                address: index + 1,
                old_state: self.state as u8,
                new_state: next_state as u8,
                time: 0,
            };
            let bytes = event.as_bytes();
            send_can_message(Command::S88Event, &bytes, false).await;

            self.state = next_state;
            // send can message
        }
    }
}

#[embassy_executor::task(pool_size = 4)]
pub async fn run(index: u16, channel: &'static GpioChannel) {
    let mut button = Button { state: false };
    loop {
        button.iterate(index, channel).await;
    }
}
