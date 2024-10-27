use rppal::gpio::{ Gpio, Level, Trigger };
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const GPIO_BUTTON: u8 = 17;

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

    // Настраиваем прерывание на falling edge (нажатие кнопки)
    button.set_interrupt(Trigger::FallingEdge)?;

    println!("Программа запущена. Нажмите Ctrl+C для выхода.");

    // Ждем прерывания
    while running.load(Ordering::SeqCst) {
        match button.poll_interrupt(true, Some(Duration::from_millis(100)))? {
            Some(Level::Low) => {
                println!("Кнопка нажата!");
                // Добавляем задержку для устранения дребезга контактов
                thread::sleep(Duration::from_millis(300));
            },
            Some(Level::High) => (),
            None => (),
        }
    }

    println!("Программа завершена");
    Ok(())
}