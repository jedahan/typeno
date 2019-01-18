extern crate linux_embedded_hal;
use linux_embedded_hal::spidev::{self, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::{Pin, Spidev};

extern crate ssd1675;
pub use ssd1675::DisplayInterface;
use ssd1675::{Builder, Dimensions, Display, GraphicDisplay, Rotation};

// Graphics
extern crate embedded_graphics;
use embedded_graphics::prelude::*;

const ROWS: u16 = 212;
const COLS: u8 = 104;

#[rustfmt::skip]
const LUT_FAST_YELLOW: [u8; 70] = [
    // Phase 0     Phase 1     Phase 2     Phase 3     Phase 4     Phase 5     Phase 6
    // A B C D     A B C D     A B C D     A B C D     A B C D     A B C D     A B C D
    0b11111010, 0b10010100, 0b10001100, 0b11000000, 0b11010000,  0b00000000, 0b00000000,  // LUT0 - Black
    0b11111010, 0b10010100, 0b00101100, 0b10000000, 0b11100000,  0b00000000, 0b00000000,  // LUTT1 - White
    0b11111010, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  0b00000000, 0b00000000,  // IGNORE
    0b11111010, 0b10010100, 0b11111000, 0b10000000, 0b01010000,  0b00000000, 0b11001100,  // LUT3 - Yellow (or Red)
    0b10111111, 0b01011000, 0b11111100, 0b10000000, 0b11010000,  0b00000000, 0b00010001,  // LUT4 - VCOM

    // Duration            | Repeat
    // A   B     C     D   |
    0,     0,   64,   16,    1,
    8,    16,    4,    4,    2,
    8,     8,    3,    8,    2,
    8,     4,    0,    0,    2,
    16,    8,    8,    0,    4,
    0,     0,    0,    0,    0,
    0,     0,    0,    0,    0,
];

pub struct Screen<'a> {
    display: GraphicDisplay<'a>
}

impl<'a> Screen<'a> {
    pub fn new() -> Self {
        // Configure SPI
        let mut spi = Spidev::open("/dev/spidev0.0").expect("SPI device");
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(4_000_000)
            .mode(spidev::SPI_MODE_0)
            .build();
        spi.configure(&options).expect("SPI configuration");

        // https://pinout.xyz/pinout/inky_phat
        // Configure Digital I/O Pins
        let cs = Pin::new(8); // BCM8
        cs.export().expect("cs export");
        while !cs.is_exported() {}
        cs.set_direction(Direction::Out).expect("CS Direction");
        cs.set_value(1).expect("CS Value set to 1");

        let busy = Pin::new(17); // BCM17
        busy.export().expect("busy export");
        while !busy.is_exported() {}
        busy.set_direction(Direction::In).expect("busy Direction");

        let dc = Pin::new(22); // BCM22
        dc.export().expect("dc export");
        while !dc.is_exported() {}
        dc.set_direction(Direction::Out).expect("dc Direction");
        dc.set_value(1).expect("dc Value set to 1");

        let reset = Pin::new(27); // BCM27
        reset.export().expect("reset export");
        while !reset.is_exported() {}
        reset
            .set_direction(Direction::Out)
            .expect("reset Direction");
        reset.set_value(1).expect("reset Value set to 1");
        println!("Pins configured");

        let controller = ssd1675::Interface::new(spi, cs, busy, dc, reset);

        let mut black_buffer = [0u8; ROWS as usize * COLS as usize / 8];
        let mut color_buffer = [0u8; ROWS as usize * COLS as usize / 8];
        let config = Builder::new()
            .dimensions(Dimensions {
                rows: ROWS,
                cols: COLS,
            })
            .rotation(Rotation::Rotate270)
            .lut(&LUT_FAST_YELLOW)
            .yellow(&true)
            .build()
            .expect("invalid configuration");
        let display = Display::new(controller, config);

        Self {
            display: GraphicDisplay::new(display, &mut black_buffer, &mut color_buffer)
        }
    }
}
