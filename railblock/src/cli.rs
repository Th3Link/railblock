use core::str::FromStr;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use esp_hal::Async;
use esp_hal::gpio::{InputPin, OutputPin};
use esp_hal::uart;
use esp_println::println;
use heapless::String;
use heapless::Vec;

use crate::config::{self, config};

pub static UART_CHANNEL: Channel<CriticalSectionRawMutex, String<100>, 4> = Channel::new();

pub async fn init(
    uart: esp_hal::peripherals::UART0<'static>,
    rx: impl InputPin + 'static,
    tx: impl OutputPin + 'static,
    spawner: &Spawner,
) {
    let uart = uart::Uart::new(uart, uart::Config::default())
        .unwrap()
        .with_rx(rx)
        .with_tx(tx)
        .into_async();

    let (rx, tx) = uart.split();

    spawner.spawn(uart_send_task(tx)).unwrap();
    spawner.spawn(uart_recieve_task(rx)).unwrap();
}

async fn handle_line(line: &str) -> Option<String<100>> {
    let parts: Vec<&str, 8> = line.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    match parts[0] {
        "device_id" => cmd_device_id(&parts[1..]).await,
        "help" => Some(help_text()),
        _ => Some(str_msg("unknown command\r\n")),
    }
}

async fn cmd_device_id(args: &[&str]) -> Option<String<100>> {
    if args.len() != 1 {
        return Some(str_msg("usage: device_id <u8>\r\n"));
    }

    match u8::from_str(args[0]) {
        Ok(id) => {
            // 👉 deine echte Logik
            config()
                .await
                .set_u8(config::Key::DeviceId, id)
                .await
                .unwrap();
            let mut msg: heapless::String<100> = heapless::String::new();
            let _ = msg.push_str("device id set: ");
            crate::helper::write_u8(&mut msg, id).ok();
            let _ = msg.push_str("\r\n");

            Some(msg)
        }
        Err(_) => Some(str_msg("invalid number\r\n")),
    }
}

fn str_msg(s: &str) -> String<100> {
    let mut out = String::new();
    let _ = out.push_str(s);
    out
}

fn help_text() -> String<100> {
    str_msg(
        "commands:\r\n\
         device_id <u8>\r\n\
         help\r\n",
    )
}

pub async fn dispatch(rx_buf: &[u8], line: &mut String<100>) {
    for &byte in rx_buf {
        match byte {
            b'\r' | b'\n' => {
                // Zeilenende → sende Zeile an Channel
                if !line.is_empty() {
                    if let Some(resp) = handle_line(line.as_str()).await {
                        UART_CHANNEL.send(resp).await;
                    }
                    line.clear();
                }
                UART_CHANNEL.send(str_msg("> ")).await; // Prompt
            }

            0x08 | 0x7F => {
                // Backspace
                line.pop();
                // Backspace auch zurückschicken, damit Terminal korrekt löscht
                let mut bs = heapless::String::<100>::new();
                let _ = bs.push_str("\x08 \x08");
                UART_CHANNEL.send(bs).await;
            }

            _ => {
                // Normales Zeichen → in Line puffern + echo
                if line.push(byte as char).is_ok() {
                    let mut echo = heapless::String::<100>::new();
                    let _ = echo.push(byte as char);
                    UART_CHANNEL.send(echo).await;
                }
            }
        }
    }
}
/*
async fn dispatch(rx_buf: &[u8], n: usize, line: &mut String<100>) {
    for &byte in &rx_buf[..n] {
        match byte {
            b'\r' | b'\n' => {
                if !line.is_empty() {
                    if let Some(resp) = handle_line(line.as_str()).await {
                        UART_CHANNEL.send(resp).await;
                    }
                    line.clear();
                }
                UART_CHANNEL.send(str_msg("> ")).await;
            }

            0x08 | 0x7F => {
                line.pop();
            }

            _ => {
                let _ = line.push(byte as char);
            }
        }
    }
}
*/
#[embassy_executor::task]
pub async fn uart_recieve_task(mut rx: uart::UartRx<'static, Async>) {
    println!("uart_recieve_task started");
    let mut rx_buf = [0u8; 32];
    let mut line: String<100> = String::new();
    loop {
        if let Ok(bytes) = rx.read_async(&mut rx_buf).await {
            dispatch(&rx_buf[..bytes], &mut line).await;
        } else {
            println!("recv error");
        }
    }
}

#[embassy_executor::task]
pub async fn uart_send_task(mut tx: uart::UartTx<'static, Async>) {
    println!("uart_send_task started");
    loop {
        let line = UART_CHANNEL.receive().await;
        tx.write_async(line.as_bytes()).await.unwrap();
    }
}
