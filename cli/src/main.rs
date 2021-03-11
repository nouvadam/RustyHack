use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use std::env;
use std::fs;

use RustyHack::*;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 256;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hack: a 16-bit computer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut hidpi_factor = window.scale_factor();

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let args: Vec<String> = env::args().collect();

    let file;

    if args.len() == 2 {
        file = fs::read_to_string(&args[1]).expect("Something went wrong with reading the file");
    } else {
        file = fs::read_to_string("../rom/Pong.hex")
            .expect("Something went wrong with reading the file");
        println!("Usage: hack filename \nExample: hack rom/Bichromia.hex");
    }

    let mut cpu = Hack::new(file);

    let mut key_pressed = 0;
    let mut key_stack = Vec::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            cpu.draw(pixels.get_frame());
            pixels.render().unwrap();
        }

        // Handle keyboard event
        if let Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                },
            ..
        } = event
        {
            key_pressed = special_keyboard_keys(input);
            match input.state {
                winit::event::ElementState::Pressed => {
                    if !key_stack.iter().any(|&x| x == key_pressed) {
                        key_stack.push(key_pressed);
                    }
                }
                winit::event::ElementState::Released => {
                    key_stack.retain(|x| *x != key_pressed);
                }
            }
            key_pressed = match key_stack.last() {
                Some(key) => *key,
                None => 0,
            };
        }

        // Put pressed key value into its memory map
        cpu.ram[24576] = key_pressed as i16;

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                hidpi_factor = factor;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            let hz = cpu.update();

            window.set_title(&format!(
                "Hack: a 16-bit computer | {0:.3} MHz | Key: {1}",
                (hz as f32) / 1_000_000_f32,
                key_pressed
            ));

            window.request_redraw();
        }
    });
}

// keyboard mapping as stated in the Hack specification
fn special_keyboard_keys(input: winit::event::KeyboardInput) -> u32 {
    match input.virtual_keycode {
        Some(key) => match key {
            VirtualKeyCode::Return => 128,
            VirtualKeyCode::Back => 129,
            VirtualKeyCode::Left => 130,
            VirtualKeyCode::Up => 131,
            VirtualKeyCode::Right => 132,
            VirtualKeyCode::Down => 133,
            VirtualKeyCode::Home => 134,
            VirtualKeyCode::End => 135,
            VirtualKeyCode::PageUp => 136,
            VirtualKeyCode::PageDown => 137,
            VirtualKeyCode::Insert => 138,
            VirtualKeyCode::Delete => 139,
            VirtualKeyCode::Escape => 140,
            VirtualKeyCode::F1 => 141,
            VirtualKeyCode::F2 => 142,
            VirtualKeyCode::F3 => 143,
            VirtualKeyCode::F4 => 144,
            VirtualKeyCode::F5 => 145,
            VirtualKeyCode::F6 => 146,
            VirtualKeyCode::F7 => 147,
            VirtualKeyCode::F8 => 148,
            VirtualKeyCode::F9 => 149,
            VirtualKeyCode::F10 => 150,
            VirtualKeyCode::F11 => 151,
            VirtualKeyCode::F12 => 152,
            _ => input.scancode,
        },
        None => input.scancode,
    }
}
