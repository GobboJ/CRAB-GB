mod cpu;

use std::{fs, env, sync::Arc};

use cpu::CPU;

use pixels::{SurfaceTexture, Pixels};
use winit::{event_loop::EventLoop, dpi::LogicalSize};
use winit::window::WindowBuilder;
use game_loop::game_loop;


fn read_rom(path: &str) -> Vec<u8> {
    fs::read(path).expect("File not found")
}


struct Game {
    pixels: Pixels,
    cpu: CPU
}

impl Game {
    fn new(pixels: Pixels) -> Self {
        Self { pixels, cpu: CPU::new() }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new({
        let size = LogicalSize::new(160, 144);
        WindowBuilder::new()
            .with_title(format!("CRAB-GB [{}]", &args[1]))
            .with_inner_size(size)
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
            
        });
}
