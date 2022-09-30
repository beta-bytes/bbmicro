use crate::api::{BBMicroApi, BBMicroGame, Button};

use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Goomba {
    id: u32,
    x: f32,
    y: f32,
}

pub struct Game1 {
    count: u32,
    x:f32,
    y:f32,
    p1x: f32,
    p1y:f32,
    p2x:f32,
    p2y: f32,
}

impl Game1 {
    pub fn new() -> Game1 {
        Game1 {
            count: 0,
            x:100.0,
            y:100.0,
            p1x: 100.0,
            p1y: 100.0,
            p2x: 100.0,
            p2y:100.0
        }
    }
}

enum Tiles {
    Grass = 48,
    WaterTL = 16,
    WaterTR = 17,
    WaterBL = 32,
    WaterBR = 33,
    WaterL = 34,
    WaterR = 19,
    Bird = 2
}

impl BBMicroGame for Game1 {
    fn init(&mut self, api: &mut BBMicroApi) {
        // Draw the base map on layer 0.
        for x in 0..256 {
            for y in 0..15 {
                api.mset(x, y, 0, Tiles::Grass as u8);
            }
        }

        // Draw on layer 1 a bird.
        //api.mset(15, 0, 1, Tiles::Bird as u8);

        //Play BGM
        api.music("bgm", 0,0); //Uncomment after adding music.mp3
    }

    fn update(&mut self, api: &mut BBMicroApi) {
        if api.btn(Button::RIGHT) {
            self.p1x += 2.0;
        }
        if api.btn(Button::D){
            self.p2x += 2.0;
        }
    }

    fn draw(&mut self, api: &mut BBMicroApi) {
        api.camera(self.x - 60.0, self.y - 60.0);

        // Draw map layer 0.
        api.map(0, 0, 0.0, 0.0, 256, 256, 0);

        let spr = 1;

        api.rect(10.0, 10.0, 20.0, 20.0, 1);

        //Makes sprites
        api.spr(spr, self.p1x - 60.0, self.p1y, 8.0, 8.0, false, false);
        api.spr(spr, self.p2x - 60.0, self.p2y-20.0, 8.0, 8.0, false, false);

        // Draw map layer 1.
        api.map(0, 0, 0.0, 0.0, 256, 256, 1);

        //Draw dead gooombas
        /* for goomba in &self.deadgoombas {
            api.spr(9, goomba.x, goomba.y, 8.0, 8.0, false, false);
        } */

        //api.print("HELLO BETABYTES!", 5.0, 5.0, false);
    }
}
