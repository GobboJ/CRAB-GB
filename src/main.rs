mod cpu;

use std::{fs, env, sync::Arc};

use cpu::CPU;

use pixels::{SurfaceTexture, Pixels};
use winit::{event_loop::EventLoop, dpi::LogicalSize};
use winit::window::WindowBuilder;
use winit::keyboard::KeyCode;
use game_loop::game_loop;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

fn read_rom(path: &str) -> Vec<u8> {
    fs::read(path).expect("File not found")
}


struct Game {
    pixels: Pixels,
    input: WinitInputHelper,
    cpu: CPU
}

impl Game {
    fn new(pixels: Pixels) -> Self {
        Self { pixels, input: WinitInputHelper::new(), cpu: CPU::new() }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new({
        let size = LogicalSize::new(WIDTH, HEIGHT);
        let scaled_size = LogicalSize::new(WIDTH * 3, HEIGHT * 3);
        WindowBuilder::new()
            .with_title(format!("CRAB-GB [{}]", &args[1]))
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    });

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(160, 144, surface_texture).unwrap()
    };

    let mut game = Game::new(pixels);
    game.cpu.load_rom(read_rom(&args[1]));

    game_loop(event_loop, window, game, 60, 0.5, 
        move |g| {
            g.game.cpu.update();
        }, 
        move |g| {
            let fb = g.game.cpu.get_framebuffer();
            let f: &mut [u8] = g.game.pixels.frame_mut();

            f.copy_from_slice(&fb);

            g.game.pixels.render();

        }, 
        |g, h| {
            if g.game.input.update(h) {
                if g.game.input.key_pressed(KeyCode::Escape) || g.game.input.close_requested() {
                    g.exit();
                    return;
                }

                if g.game.input.key_held(KeyCode::KeyW) {
                    g.game.cpu.set_button(cpu::joypad::Button::U);
                } else {
                    g.game.cpu.unset_button(cpu::joypad::Button::U);
                }

                if g.game.input.key_held(KeyCode::KeyS) {
                    g.game.cpu.set_button(cpu::joypad::Button::D);
                } else {
                    g.game.cpu.unset_button(cpu::joypad::Button::D);
                }

                if g.game.input.key_held(KeyCode::KeyA) {
                    g.game.cpu.set_button(cpu::joypad::Button::L);
                } else {
                    g.game.cpu.unset_button(cpu::joypad::Button::L);
                }

                if g.game.input.key_held(KeyCode::KeyD) {
                    g.game.cpu.set_button(cpu::joypad::Button::R);
                } else {
                    g.game.cpu.unset_button(cpu::joypad::Button::R);
                }

                if g.game.input.key_held(KeyCode::KeyI) {
                    g.game.cpu.set_button(cpu::joypad::Button::A);
                } else {
                    g.game.cpu.unset_button(cpu::joypad::Button::A);
                }

                if g.game.input.key_held(KeyCode::KeyJ) {
                    g.game.cpu.set_button(cpu::joypad::Button::B);
                } else {
                    g.game.cpu.unset_button(cpu::joypad::Button::B);
                }

                if g.game.input.key_held(KeyCode::KeyN) {
                    g.game.cpu.set_button(cpu::joypad::Button::STA);
                } else {
                    g.game.cpu.unset_button(cpu::joypad::Button::STA);
                }

                if g.game.input.key_held(KeyCode::KeyB) {
                    g.game.cpu.set_button(cpu::joypad::Button::SEL);
                } else {
                    g.game.cpu.unset_button(cpu::joypad::Button::SEL);
                }

            }
        });
}
