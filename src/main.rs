use rppal::gpio::{ Gpio, Level, Trigger };
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const GPIO_BUTTON: u8 = 17;

const GPIO_PIN_LAZER: u8 = 18;

// fn blink(pin: &mut rppal::gpio::OutputPin, count: u32, duration: Duration) {
//     for _ in 0..count {
//         pin.set_high();
//         thread::sleep(duration);
//         pin.set_low();
//         thread::sleep(duration);
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    // Флаг для корректного завершения программы
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Обработчик Ctrl+C
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let gpio = Gpio::new()?;
    let mut button = gpio.get(GPIO_BUTTON)?.into_input_pullup();

    let mut pin = gpio.get(GPIO_PIN_LAZER)?.into_output();

    // Настраиваем прерывание на falling edge (нажатие кнопки)
    button.set_interrupt(Trigger::FallingEdge)?;

    println!("Программа запущена. Нажмите Ctrl+C для выхода.");

    // Ждем прерывания
    while running.load(Ordering::SeqCst) {
        match button.poll_interrupt(true, Some(Duration::from_millis(100)))? {
            Some(Level::Low) => {
                println!("Кнопка нажата!");
                pin.set_high();
                println!("Лазер включен");
                // Добавляем задержку для устранения дребезга контактов
                thread::sleep(Duration::from_millis(1000));

                pin.set_low();
            },
            Some(Level::High) => (),
            None => (),
        }
    }

    pin.set_low();
    println!("Программа завершена");
    Ok(())
}