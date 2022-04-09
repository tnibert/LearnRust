extern crate sdl2;

use sdl2::render::{Canvas, WindowCanvas, Texture, TextureAccess};
use sdl2::Sdl;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::rect::{Point, Rect};
use std::env;
use std::path::Path;
use std::{thread, time::Duration};

// TODO: separate the character into frames, match up for walking

/// Emulated screen width in pixels
const SCREEN_WIDTH: usize = 256*2;
/// Emulated screen height in pixels
const SCREEN_HEIGHT: usize = 240*2;
/// Screen texture size in bytes
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

const SCALE: usize = 1;

const SPRITE_W: u32 = 26;
const SPRITE_H: u32 = 36;

// handles renderable character
#[derive(Debug)]
struct Sprite {
    position: Point,
    area: Rect,
    speed: i32
}

impl Sprite {
    fn movespr(&self, xdiff: i32, ydiff: i32) {
        self.position.offset(xdiff, ydiff);
    }
}

// handles player
#[derive(Debug)]
struct Player {
    spr: Sprite
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    // get size of window
    let (width, height) = canvas.output_size()?;

    // world coordinate system
    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = player.spr.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.spr.area.width(), player.spr.area.height());

    canvas.copy(texture, player.spr.area, screen_rect)?;

    canvas.present();

    Ok(())
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // why can't I use ? instead of unwrap()?
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
 
    let window = video_subsystem.window("test",
                                        (SCREEN_WIDTH as usize * SCALE) as u32,
                                        (SCREEN_HEIGHT as usize * SCALE) as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string()).unwrap();
 
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string()).unwrap();
 
    let texture_creator = canvas.texture_creator();
    let png = Path::new("assets/reaper.png");
    let texture = texture_creator.load_texture(png).unwrap();

    let mut player = Player {
        spr: Sprite {
            position: Point::new(0, 0),
            // src position in the spritesheet
            area: Rect::new(0, 0, SPRITE_W, SPRITE_H),
            speed: 5
        }
    };

    let bg_color = Color::RGB(120, 255, 255);
    canvas.set_draw_color(bg_color);
    canvas.clear();
    canvas.present();

    // event pump is queried to find out if there are any pending events
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        canvas.set_draw_color(bg_color);
        canvas.clear();
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // player control
                // how to allow diagonal movement?
                // answer: don't set the player position directly here
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    player.spr.position = player.spr.position.offset(-player.spr.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    player.spr.position = player.spr.position.offset(player.spr.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    player.spr.position = player.spr.position.offset(0, -player.spr.speed);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    player.spr.position = player.spr.position.offset(0, player.spr.speed);
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // blit
        render(&mut canvas, bg_color, &texture, &player);

        // todo: use monotonic clock to find exact time for sleep
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

// some notes:
//
// Just remember that 32, Some(32), and None can all be passed into a function whose type implements Into<Option<i32>>.
// This pattern is a relatively easy way to implement optional/default arguments in Rust.
//
// copy_ex() is like copy() but with some more options