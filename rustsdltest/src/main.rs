use sdl2::render::{Canvas, Texture, TextureAccess};
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::{thread, time::Duration};

/// Emulated screen width in pixels
const SCREEN_WIDTH: usize = 256;
/// Emulated screen height in pixels
const SCREEN_HEIGHT: usize = 240;
/// Screen texture size in bytes
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

const SCALE: usize = 1;

// the following.. goes to screen as streaming texture?
/*fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let mut window_builder = video_subsystem.window(
        "test",
        (SCREEN_WIDTH as usize * SCALE) as u32,
        (SCREEN_HEIGHT as usize * SCALE) as u32,
        );
    let window = window_builder.position_centered().build().unwrap();

    let renderer = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    let texture_creator = renderer.texture_creator();
    let texture_creator_pointer = &texture_creator as *const TextureCreator<WindowContext>;
    let texture = unsafe { &*texture_creator_pointer }
        .create_texture(
            PixelFormatEnum::BGR24,
            TextureAccess::Streaming,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .unwrap();

    thread::sleep(Duration::from_millis(4000));
}*/

// this one is just the basic SDL example from https://rust-sdl2.github.io/rust-sdl2/sdl2/
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("test",
                                        (SCREEN_WIDTH as usize * SCALE) as u32,
                                        (SCREEN_HEIGHT as usize * SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}