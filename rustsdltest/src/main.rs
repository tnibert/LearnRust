use sdl2::render::{Canvas, Texture, TextureAccess};
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use std::{thread, time::Duration};

/// Emulated screen width in pixels
const SCREEN_WIDTH: usize = 256;
/// Emulated screen height in pixels
const SCREEN_HEIGHT: usize = 240;
/// Screen texture size in bytes
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

const SCALE: usize = 1;

fn main() {
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
}
