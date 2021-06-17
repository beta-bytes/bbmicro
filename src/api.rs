use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::rect::Rect;

use serde::Deserialize;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/*
PICO 8 is 128 x 128 pixels
each tile is 8x8
a single screen is 16x16 tiles

API for PICO-8

** Graphics **
camera([x,] [y])
circ(x, y, r, [col])
circfill(x, y, r, [col])
clip([x,] [y,] [w,] [h])
cls()
color(col)
cursor([x,] [y,] [col])
fget(n, [f])
fillp([pat])
fset(n, [f,] [v])
line(x0, y0, x1, y1, [col])
pal([c0,] [c1,] [p])
palt([c,] [t])
pget(x, y)
print(str, [x,] [y,] [col])
pset(x, y, [c])
rect(x0, y0, x1, y1, [col])
rectfill(x0, y0, x1, y1, [col])
sget(x, y)
spr(n, x, y, [w,] [h,] [flip_x,] [flip_y])
sset(x, y, [c])
sspr(sx, sy, sw, sh, dx, dy, [dw,] [dh,] [flip_x,] [flip_y])
tline(x0, y0, x1, y1, mx, my, [mdx,] [mdy])

** Sound **
music(n, [fadems,] [channelmask])
sfx(n, [channel,] [offset,] [length])

** Input **
btn
btnp

**

*/

type Color = u8;

pub enum Button {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    A,
    B,
}

pub struct InputState {
    pub left_down: bool,
    pub left_pressed: bool,

    pub right_down: bool,
    pub right_pressed: bool,

    pub up_down: bool,
    pub up_pressed: bool,

    pub down_down: bool,
    pub down_pressed: bool,

    pub a_down: bool,
    pub a_pressed: bool,

    pub b_down: bool,
    pub b_pressed: bool,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            left_down: false,
            left_pressed: false,
            right_down: false,
            right_pressed: false,

            up_down: false,
            up_pressed: false,

            down_down: false,
            down_pressed: false,

            a_down: false,
            a_pressed: false,

            b_down: false,
            b_pressed: false,
        }
    }
}

struct DrawState {
    camera_x: f32,
    camera_y: f32,
    pen: Color,
}

pub struct BBMicroApi<'a> {
    canvas: &'a mut sdl2::render::WindowCanvas,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    sprites_texture: sdl2::render::Texture<'a>,
    font_texture: sdl2::render::Texture<'a>,
    font_entries: HashMap<char, FontEntry>,
    draw_state: DrawState,
    input_state: InputState,
    map_data: [u8; 256 * 256],
}

#[derive(Deserialize, Debug)]
struct FontEntry {
    top_x: f64,
    top_y: f64,
    bottom_x: f64,
    bottom_y: f64,
    bottom_offset: f64,
}

impl FontEntry {
    fn width(&self) -> f64 {
        self.bottom_x - self.top_x
    }

    fn height(&self) -> f64 {
        self.bottom_y - self.top_y
    }
}

fn load_font() -> Result<HashMap<char, FontEntry>, Box<dyn Error>> {
    let file = File::open("font.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let font_entries: HashMap<char, FontEntry> = serde_json::from_reader(reader)?;
    // Return the `User`.
    Ok(font_entries)
}

impl<'a> BBMicroApi<'a> {
    pub fn new(
        canvas: &'a mut sdl2::render::WindowCanvas,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) -> BBMicroApi<'a> {
        let sprites_texture = texture_creator
            .load_texture("sprites.png")
            .expect("Couldn't load the texture");
        let font_texture = texture_creator
            .load_texture("font.png")
            .expect("Couldn't load the texture");

        let font_entries = load_font().expect("Could not load the font.json");

        BBMicroApi {
            canvas: canvas,
            texture_creator: texture_creator,
            sprites_texture: sprites_texture,
            font_texture: font_texture,
            font_entries: font_entries,
            draw_state: DrawState {
                camera_x: 0.0,
                camera_y: 0.0,
                pen: 0,
            },
            input_state: InputState::new(),
            map_data: [0; 256 * 256],
        }
    }

    /* Internal utility */
    pub fn update_input(&mut self, input_state: InputState) {
        self.input_state = input_state;
    }

    fn to_sdl_color(&self, col: Color) -> pixels::Color {
        match col {
            0 => pixels::Color::RGB(0, 0, 0),
            1 => pixels::Color::RGB(29, 43, 83),
            2 => pixels::Color::RGB(126, 37, 83),
            3 => pixels::Color::RGB(0, 135, 81),
            4 => pixels::Color::RGB(117, 82, 54),
            5 => pixels::Color::RGB(95, 87, 79),
            6 => pixels::Color::RGB(194, 195, 199),
            7 => pixels::Color::RGB(255, 241, 232),
            8 => pixels::Color::RGB(255, 0, 77),
            9 => pixels::Color::RGB(255, 163, 0),
            10 => pixels::Color::RGB(255, 236, 39),
            11 => pixels::Color::RGB(0, 228, 54),
            12 => pixels::Color::RGB(41, 173, 255),
            13 => pixels::Color::RGB(131, 118, 156),
            14 => pixels::Color::RGB(255, 119, 168),
            15 => pixels::Color::RGB(0255, 204, 170),
            _ => pixels::Color::RGB(0, 0, 0),
        }
    }

    fn to_camera(&self, x: f32, y: f32) -> (f32, f32) {
        (x - self.draw_state.camera_x, y - self.draw_state.camera_y)
    }

    /* PICO 8 API */
    pub fn camera(&mut self, x: f32, y: f32) {
        self.draw_state.camera_x = x;
        self.draw_state.camera_y = y;
    }

    pub fn circ(&mut self, x: f32, y: f32, r: f32, col: Color) {}

    pub fn circfill(&mut self, x: f32, y: f32, r: f32, col: Color) {}

    pub fn clip(&mut self, x: f32, y: f32, w: f32, h: f32) {}

    pub fn cls(&mut self, col: Color) {
        self.canvas.set_draw_color(self.to_sdl_color(col));
        self.canvas.clear()
    }

    pub fn color(&mut self, col: Color) {
        self.draw_state.pen = col;
    }

    pub fn spr(&mut self, n: u8, x: f32, y: f32, w: f32, h: f32, flip_x: bool, flip_y: bool) {
        let (x, y) = self.to_camera(x, y);

        let src_x = (n % 16) * 8;
        let src_y = (n / 16) * 8;

        let src_rect = Rect::new(src_x as i32, src_y as i32, 8, 8);
        let dst_rect = Rect::new(x as i32, y as i32, 8, 8);
        self.canvas
            .copy(&self.sprites_texture, src_rect, dst_rect)
            .unwrap();
    }

    pub fn print(&mut self, text: &str, x: f32, y: f32, use_camera: bool) {
        let mut curr_x = x;
        let mut curr_y = y;

        let offset_x = if use_camera {
            self.draw_state.camera_x
        } else {
            0.0
        };
        let offset_y = if use_camera {
            self.draw_state.camera_y
        } else {
            0.0
        };

        for character in text.chars() {
            match self.font_entries.get(&character) {
                Some(font_entry) => {
                    let src_rect = Rect::new(
                        font_entry.top_x as i32,
                        font_entry.top_y as i32,
                        font_entry.width() as u32,
                        font_entry.height() as u32,
                    );
                    let dst_rect =
                        Rect::new((curr_x - offset_x) as i32, (curr_y - offset_y) as i32, 8, 8);
                    self.canvas
                        .copy(&self.font_texture, src_rect, dst_rect)
                        .unwrap();
                }
                None => {}
            }
            curr_x += 8.0;
        }
    }

    pub fn rect(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, col: Color) {
        let (x0, y0) = self.to_camera(x0, y0);
        let (x1, y1) = self.to_camera(x1, y1);

        self.canvas.set_draw_color(self.to_sdl_color(col));
        let w = x1 - x0;
        let h = y1 - y0;
        let r = Rect::new(x0 as i32, y0 as i32, w as u32, h as u32);
        self.canvas.draw_rect(r);
    }

    pub fn flip(&mut self) {
        self.canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    pub fn btn(&self, b: Button) -> bool {
        // Returns whether a button is being held down.
        match b {
            Button::LEFT => self.input_state.left_down,
            Button::RIGHT => self.input_state.right_down,
            Button::UP => self.input_state.up_down,
            Button::DOWN => self.input_state.down_down,
            Button::A => self.input_state.a_down,
            Button::B => self.input_state.b_down,
        }
    }

    pub fn btnp(&self, b: Button) -> bool {
        // Returns whether a button was just pressed.
        match b {
            Button::LEFT => self.input_state.left_pressed,
            Button::RIGHT => self.input_state.right_pressed,
            Button::UP => self.input_state.up_pressed,
            Button::DOWN => self.input_state.down_pressed,
            Button::A => self.input_state.a_pressed,
            Button::B => self.input_state.b_pressed,
        }
    }

    pub fn mset(&mut self, celx: u32, cely: u32, snum: u8) {
        assert_eq!(celx < 256, true);
        assert_eq!(cely < 256, true);
        self.map_data[celx as usize + (cely as usize) * 256] = snum;
    }

    pub fn mget(&mut self, celx: u32, cely: u32, snum: u8) -> u8 {
        assert_eq!(celx < 256, true);
        assert_eq!(cely < 256, true);
        return self.map_data[celx as usize + (cely as usize) * 256];
    }

    pub fn map(&mut self, celx: u32, cely: u32, sx: f32, sy: f32, celw: u32, celh: u32, layer: u8) {
        // for now we ignore layer

        for i_x in 0..celw {
            for i_y in 0..celh {
                let t_x = (i_x + celx) as usize;
                let t_y = (i_y + cely) as usize;
                if !(t_x <= 255 && t_y <= 255) {
                    continue;
                }

                let tile = self.map_data[t_x + t_y * 256];

                // Get the sprite value at the current location
                self.spr(
                    tile,
                    sx + (i_x as f32 * 8.0),
                    sy + (i_y as f32 * 8.0),
                    8 as f32,
                    8 as f32,
                    false,
                    false,
                );
            }
        }
    }
}

pub trait BBMicroGame {
    fn init(&mut self, api: &mut BBMicroApi);
    fn update(&mut self, api: &mut BBMicroApi);
    fn draw(&mut self, api: &mut BBMicroApi);
}
