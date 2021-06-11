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
    x: f32,
    y: f32,
    goombas: Vec<Goomba>,
    rng: ThreadRng,
    deadgoombas: Vec<Goomba>,
}

impl Game1 {
    pub fn new() -> Game1 {
        Game1 {
            count: 0,
            x: 100.0,
            y: 100.0,
            goombas: vec![],
            deadgoombas: vec![],
            rng: rand::thread_rng(),
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
        for id in 0..100 {
            self.goombas.push(Goomba {
                id: id,
                x: self.rng.gen_range(0.0..300.0),
                y: self.rng.gen_range(0.0..300.0),
            });
        }

        // Draw the base map on layer 0.
        for x in 0..256 {
            for y in 0..256 {
                api.mset(x, y, 0, Tiles::Grass as u8);
            }
        }

        // Draw a little island.
        api.mset(10, 10, 0, Tiles::WaterTL as u8);
        api.mset(11, 10, 0, Tiles::WaterTR as u8);
        api.mset(10, 11, 0, Tiles::WaterL as u8);
        api.mset(11, 11, 0, Tiles::WaterR as u8);
        api.mset(10, 12, 0, Tiles::WaterBL as u8);
        api.mset(11, 12, 0, Tiles::WaterBR as u8);

        // Draw on layer 1 a bird.
        api.mset(15, 15, 1, Tiles::Bird as u8);
    }

    fn update(&mut self, api: &mut BBMicroApi) {

        api.sfx("ghost.wav",1,0,0);

        self.count += 1;

        if self.count > 100 {
            self.count = 0;
        }

        if api.btn(Button::LEFT) {
            self.x -= 2.0;
        }
        if api.btn(Button::RIGHT) {
            self.x += 2.0;
        }
        if api.btn(Button::UP) {
            self.y -= 2.0;
        }
        if api.btn(Button::DOWN) {
            self.y += 2.0;
        }

        for goomba in &mut self.goombas {
            goomba.x += self.rng.gen_range(-1.0..1.0);
            goomba.y += self.rng.gen_range(-1.0..1.0);

            if((goomba.x -self.x).abs() < 3.0 && (goomba.y -self.y).abs() < 3.0) {
                api.sfx("ghost.wav",1,0,0);
                self.deadgoombas.push(Goomba {
                    id: goomba.id,
                    x: goomba.x,
                    y: goomba.y
                });
            }
        }

        for goomba in &mut self.deadgoombas {
            if let Some(pos) = self.goombas.iter().position(|x| x.id == goomba.id) {
                self.goombas.remove(pos);
            }
        }

    }

    fn draw(&mut self, api: &mut BBMicroApi) {
        api.camera(self.x - 60.0, self.y - 60.0);

        // Draw map layer 0.
        api.map(0, 0, 0.0, 0.0, 256, 256, 0);

        let spr = if self.count > 50 { 1 } else { 2 };

        api.rect(10.0, 10.0, 20.0, 20.0, 1);

        api.spr(spr, self.x, self.y, 8.0, 8.0, false, false);

        for goomba in &self.goombas {
            api.spr(8, goomba.x, goomba.y, 8.0, 8.0, false, false);
        }

<<<<<<< HEAD
        // Draw map layer 1.
        api.map(0, 0, 0.0, 0.0, 256, 256, 1);
=======
        for goomba in &self.deadgoombas {
            api.spr(9, goomba.x, goomba.y, 8.0, 8.0, false, false);
        }
>>>>>>> 77ae882 (trying to get audio to work)

        api.print("HELLO BETABYTES!", 5.0, 5.0, false);
    }
}
