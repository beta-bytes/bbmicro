use crate::api::{Button, BBMicroApi, BBMicroGame};

pub struct Game1 {
    count: u32,
    x: f32,
    y: f32,
}

impl Game1 {
    pub fn new() -> Game1 {
        Game1 {
            count: 0,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl BBMicroGame for Game1 {
    fn init(&mut self, api: &mut BBMicroApi) {}

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
    }

    fn draw(&mut self, api: &mut BBMicroApi) {
        api.camera(self.x, self.y);

        api.map(0, 0, 0.0, 0.0, 256, 256, 0);

        let spr = if self.count > 50 { 1 } else { 2 };

        api.rect(10.0, 10.0, 20.0, 20.0, 1);

        api.spr(spr, 40.0, 40.0, 8.0, 8.0, false, false);
    }
}
