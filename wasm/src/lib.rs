extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;
use std::panic;
#[wasm_bindgen]
pub fn init_panic_hook() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Hack {
    hack: RustyHack::Hack,
    draw_frame: [u8; 524288],
    key_stack: Vec<i16>,
}

#[wasm_bindgen]
impl Hack {
    pub fn new() -> Hack {
        Hack {
            hack: RustyHack::Hack::new(),
            draw_frame: [0; 524288],
            key_stack: Vec::new(),
        }
    }

    pub fn tick(&mut self, how_many_ticks: u32) {
        //log!("{:?}", how_many_ticks);

        for _ in 0..how_many_ticks {
            self.hack.tick()
        }
    }

    ///Reset computer state
    pub fn reset(&mut self) {
        self.hack.reset();
    }

    pub fn load_rom(&mut self, file: String) {
        self.hack.load_rom(file);
    }

    pub fn set_key(&mut self, input: i16) {
        //self.hack.ram[24576] = special_keyboard_keys(key_pressed);

        let key_pressed = special_keyboard_keys(input);
        // log!("{}", key_pressed);

        if !self.key_stack.iter().any(|&x| x == key_pressed) {
            self.key_stack.push(key_pressed);
        }

        let key_pressed = match self.key_stack.last() {
            Some(key) => *key,
            None => 0,
        };
        self.hack.ram[24576] = key_pressed;
    }

    pub fn release_key(&mut self, input: i16) {
        let key_pressed = special_keyboard_keys(input);

        self.key_stack.retain(|x| *x != key_pressed);

        let key_pressed = match self.key_stack.last() {
            Some(key) => *key,
            None => 0,
        };

        self.hack.ram[24576] = key_pressed;
    }

    pub fn frame(&self) -> *const u8 {
        self.draw_frame.as_ptr()
    }

    // draw screen memory map to physical screen buffer
    pub fn draw(&mut self) {
        self.hack.draw(&mut self.draw_frame);
    }
}

fn special_keyboard_keys(input: i16) -> i16 {
    match input {
        13 => 128,
        8 => 129,
        37 => 130,
        38 => 131,
        39 => 132,
        40 => 133,
        57 => 32,
        36 => 134,
        35 => 135,
        33 => 136,
        34 => 137,
        45 => 138,
        46 => 139,
        27 => 140,
        112 => 141,
        113 => 142,
        114 => 143,
        115 => 144,
        116 => 145,
        117 => 146,
        118 => 147,
        119 => 148,
        120 => 149,
        121 => 150,
        122 => 151,
        123 => 152,
        _ => input,
    }
}
