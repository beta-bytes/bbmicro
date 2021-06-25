use crate::api::{BBMicroApi, BBMicroGame, Button};

use rand::rngs::ThreadRng;
use rand::Rng;

struct Point {
    x: f32,
    y: f32,
}

pub struct Game1 {
    height: f32,
    width: f32,
    cat_x: f32,
    cat_y: f32,
    rng: ThreadRng,
}

impl Game1 {
    pub fn new() -> Game1 {
        Game1 {
            height: 128.0,
            width: 128.0,
            cat_x: 10.0,
            cat_y: 10.0,
            rng: rand::thread_rng(),
        }
    }
}

#[derive(Copy, Clone)]
enum Tiles {
    Cat = 2,
    Roomba = 3,
    Red = 13,
    Orange = 14,
    Yellow = 15,
    Purple = 31,
    Blue = 30,
    Green = 29,
    Violet = 45,
}

impl BBMicroGame for Game1 {
    fn init(&mut self, api: &mut BBMicroApi) {
        // Draw the base map on layer 0.
        for x in 0..256 {
            for y in 0..256 {
                api.mset(x, y, 0, Tiles::Roomba as u8);
            }
        }

        for (i, color) in [
            Tiles::Violet,
            Tiles::Blue,
            Tiles::Green,
            Tiles::Yellow,
            Tiles::Orange,
            Tiles::Red,
        ]
        .iter()
        .enumerate()
        {
            let i: u32 = i as u32;
            api.mset(0, 15 - i, 0, *color as u8);
            api.mset(1, 15 - i, 0, *color as u8);
            api.mset(2, 14 - i, 0, *color as u8);
            api.mset(3, 14 - i, 0, *color as u8);
            api.mset(4, 13 - i, 0, *color as u8);
            api.mset(5, 13 - i, 0, *color as u8);
            api.mset(6, 12 - i, 0, *color as u8);
            api.mset(7, 12 - i, 0, *color as u8);

            api.mset(8, 12 - i, 0, *color as u8);
            api.mset(9, 12 - i, 0, *color as u8);
            api.mset(10, 13 - i, 0, *color as u8);
            api.mset(11, 13 - i, 0, *color as u8);
            api.mset(12, 14 - i, 0, *color as u8);
            api.mset(13, 14 - i, 0, *color as u8);
            api.mset(14, 15 - i, 0, *color as u8);
            api.mset(15, 15 - i, 0, *color as u8);
        }

        api.music("bgm", 0, 0);
    }

    fn update(&mut self, api: &mut BBMicroApi) {
        if api.btn(Button::LEFT) {
            self.cat_x -= 2.0;
        }
        if api.btn(Button::RIGHT) {
            self.cat_x += 2.0;
        }
        if api.btn(Button::UP) {
            self.cat_y -= 2.0;
        }
        if api.btn(Button::DOWN) {
            self.cat_y += 2.0;
        }
    }

    fn draw(&mut self, api: &mut BBMicroApi) {
        //api.camera(self.cat_x - 64.0 - 4.0, self.cat_y - 64.0 - 4.0);
        // Draw map layer 0.
        api.map(0, 0, 0.0, 0.0, 256, 256, 0);

        api.spr(
            Tiles::Cat as u8,
            self.cat_x,
            self.cat_y,
            8.0,
            8.0,
            false,
            false,
        );
    }
}
