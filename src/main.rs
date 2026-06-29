#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;

const DOT_MS: u32 = 300;
const DASH_MS: u32 = DOT_MS * 4;

fn flash_letter(letter: char, red_led: &mut Pin<Output>, green_led: &mut Pin<Output>,
                blue_led: &mut Pin<Output>) {
    let morse_code = match letter.to_ascii_uppercase() {
        'A' => ".-",
        'B' => "-...",
        'C' => "-.-.",
        'D' => "-..",
        'E' => ".",
        'F' => "..-.",
        'G' => "--.",
        'H' => "....",
        'I' => "..",
        'J' => ".---",
        'K' => "-.-",
        'L' => ".-..",
        'M' => "--",
        'N' => "-.",
        'O' => "---",
        'P' => ".--.",
        'Q' => "--.-",
        'R' => ".-.",
        'S' => "...",
        'T' => "-",
        'U' => "..-",
        'V' => "...-",
        'W' => ".--",
        'X' => "-..-",
        'Y' => "-.--",
        'Z' => "--..",
        _ => "",
    };

    if letter == ' ' {
        arduino_hal::delay_ms(DOT_MS * 6);
        return;
    }

    for signal in morse_code.chars() {
        match signal {
            '.' => {
                red_led.set_low();
                green_led.set_high();
                blue_led.set_high();
                arduino_hal::delay_ms(DOT_MS);
            },
            '-' => {
                red_led.set_high();
                green_led.set_low();
                blue_led.set_high();
                arduino_hal::delay_ms(DASH_MS);
            },
            _ => (),
        }
        red_led.set_high();
        green_led.set_high();
        blue_led.set_high();
        arduino_hal::delay_ms(DOT_MS);
    }

    arduino_hal::delay_ms(DOT_MS * 2);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut red_led = pins.d2.into_output().downgrade();
    let mut green_led = pins.d3.into_output().downgrade();
    let mut blue_led = pins.d5.into_output().downgrade();

    loop {
        flash_letter('S', &mut red_led, &mut green_led, &mut blue_led);
        flash_letter('O', &mut red_led, &mut green_led, &mut blue_led);
        flash_letter('S', &mut red_led, &mut green_led, &mut blue_led);
        arduino_hal::delay_ms(200);
    }
}
