use crate::api::{BBMicroApi, BBMicroGame, Button};

use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Game1 {
    height: f32,
    width: f32,
    roomba_x: f32,
    roomba_y: f32,
    cat_x: f32,
    cat_y: f32,
    rng: ThreadRng,
}

impl Game1 {
    pub fn new() -> Game1 {
        Game1 {
            height: 128.0,
            width: 128.0,
            roomba_x: 100.0,
            roomba_y: 100.0,
            cat_x: 10.0,
            cat_y: 10.0,
            rng: rand::thread_rng(),
        }
    }

    pub fn bound(&self, x: f32, y: f32) -> (f32, f32) {
        (x.min(self.width - 8.0).max(0.0),
        y.min(self.height - 8.0).max(0.0))
    }
}

enum Tiles {
    Dirty = 1,
    Cat = 2,
    Roomba = 3,
    Clean = 6,
}

impl BBMicroGame for Game1 {
    fn init(&mut self, api: &mut BBMicroApi) {
        // Draw the base map on layer 0.
        for x in 0..256 {
            for y in 0..256 {
                api.mset(x, y, 0, Tiles::Dirty as u8);
            }
        }

        // Play BGM
        api.music("bgm", 0, 0);
    }

    fn update(&mut self, api: &mut BBMicroApi) {
        if api.btn(Button::LEFT) {
            self.cat_x -= 2.0;
            self.roomba_x -= 2.0;
        }
        if api.btn(Button::RIGHT){
            self.cat_x += 2.0;
            self.roomba_x += 2.0;
        }
        if api.btn(Button::UP) {
            self.cat_y -= 2.0;
            self.roomba_y -= 2.0;
        }
        if api.btn(Button::DOWN){
            self.cat_y += 2.0;
            self.roomba_y += 2.0;
        }

        let new_cat_pos = self.bound(self.cat_x, self.cat_y);
        self.cat_x = new_cat_pos.0;
        self.cat_y = new_cat_pos.1;

        let new_roomba_pos = self.bound(self.roomba_x, self.roomba_y);
        self.roomba_x = new_roomba_pos.0;
        self.roomba_y = new_roomba_pos.1;

        // Flip
        let tile_x = (self.roomba_x + 4.0) as u32 / 8;
        let tile_y = (self.roomba_y + 4.0) as u32 / 8;

        if tile_x >= 0 && tile_x < 256 && tile_y >= 0 && tile_y < 256 {
            api.mset(tile_x, tile_y, 0, Tiles::Clean as u8);
        }
    }

    fn draw(&mut self, api: &mut BBMicroApi) {
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

        api.spr(
            Tiles::Roomba as u8,
            self.roomba_x,
            self.roomba_y,
            8.0,
            8.0,
            false,
            false,
        );
    }
}
