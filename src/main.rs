use rppal::gpio::{Gpio, InputPin, Level};
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

const IR_PIN: u8 = 17; // GPIO пин, к которому подключен TL1838
const TIMEOUT_MS: u64 = 100; // Таймаут ожидания сигнала в миллисекундах

struct IRReceiver {
    pin: InputPin,
}

impl IRReceiver {
    fn new() -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(IR_PIN)?.into_input();
        Ok(IRReceiver { pin })
    }

    // Чтение одного ИК импульса
    fn read_pulse(&self, expected_level: Level) -> Option<u32> {
        let start = Instant::now();

        while self.pin.read() != expected_level {
            if start.elapsed() > Duration::from_millis(TIMEOUT_MS) {
                return None;
            }
        }

        let pulse_start = Instant::now();

        while self.pin.read() == expected_level {
            if pulse_start.elapsed() > Duration::from_millis(TIMEOUT_MS) {
                return None;
            }
        }

        Some(pulse_start.elapsed().as_micros() as u32)
    }

    // Декодирование NEC протокола
    fn decode_nec(&self) -> Option<u32> {
        // Ждем начальный импульс
        let lead_pulse = self.read_pulse(Level::Low)?;
        let lead_space = self.read_pulse(Level::High)?;

        // Проверяем, соответствует ли начальный импульс протоколу NEC
        if !(lead_pulse > 8000 && lead_pulse < 10000 &&
            lead_space > 4000 && lead_space < 5000) {
            return None;
        }

        let mut data: u32 = 0;

        // Читаем 32 бита данных
        for i in 0..32 {
            let pulse = self.read_pulse(Level::Low)?;
            let space = self.read_pulse(Level::High)?;

            if pulse > 400 && pulse < 700 {
                if space > 1500 && space < 1800 {
                    // Логическая 1
                    data |= 1 << (31 - i);
                } else if space > 400 && space < 700 {
                    // Логический 0
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(data)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let receiver = IRReceiver::new()?;
    println!("IR receiver initialized. Waiting for signals...");

    loop {
        if let Some(code) = receiver.decode_nec() {
            println!("Received IR code: 0x{:08X}", code);
        }
        thread::sleep(Duration::from_millis(100));
    }
}