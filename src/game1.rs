use crate::api::{BBMicroApi, BBMicroGame, Button};

use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Goomba {
    x: f32,
    y: f32,
}

pub struct Game1 {
    count: u32,
    x: f32,
    y: f32,
    goombas: Vec<Goomba>,
    rng: ThreadRng,
}

impl Game1 {
    pub fn new() -> Game1 {
        Game1 {
            count: 0,
            x: 100.0,
            y: 100.0,
            goombas: vec![],
            rng: rand::thread_rng(),
        }
    }
}

impl BBMicroGame for Game1 {
    fn init(&mut self, api: &mut BBMicroApi) {
        for _ in 0..100 {
            self.goombas.push(Goomba {
                x: self.rng.gen_range(0.0..300.0),
                y: self.rng.gen_range(0.0..300.0),
            });
        }
    }

    fn update(&mut self, api: &mut BBMicroApi) {
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
        }
    }

    fn draw(&mut self, api: &mut BBMicroApi) {
        api.camera(self.x - 60.0, self.y - 60.0);

        api.map(0, 0, 0.0, 0.0, 256, 256, 0);

        let spr = if self.count > 50 { 1 } else { 2 };

        api.rect(10.0, 10.0, 20.0, 20.0, 1);

        api.spr(spr, self.x, self.y, 8.0, 8.0, false, false);

        for goomba in &self.goombas {
            api.spr(8, goomba.x, goomba.y, 8.0, 8.0, false, false);
        }

        api.print("HELLO BETABYTES!", 5.0, 5.0, false);
    }
}
