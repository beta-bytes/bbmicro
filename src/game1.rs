use crate::api::{BBMicroApi, BBMicroGame, Button};

pub struct StopLight {
    top: u8,
    bott: u8,
    x: f32,
    y: f32,
}

pub struct Game1 {
    count: u32,
    x: f32,
    y: f32,
    p1x: f32,
    p1y:f32,
    p2x:f32,
    p2y: f32,
    p1w: bool,
    p2w: bool,
    stop_light: StopLight,
    green_light: bool
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
            p2y:100.0,
            p1w:false,
            p2w: false,
            stop_light: 
                StopLight{
                    top: Tiles::RDTop as u8,
                    bott: Tiles::RDBott as u8,
                    x: 100.0,
                    y: 52.0
                },
            green_light: false,
        }
    }
}

enum Tiles {
    FinishLine = 68,
    Grass = 48,
    RDTop = 64,
    RDBott = 80,
    GrTop = 65,
    GRBott = 81
}

impl BBMicroGame for Game1 {
    fn init(&mut self, api: &mut BBMicroApi) {
        // Draw the base map on layer 0.
        for x in 0..256 {
            for y in 0..16 {
                api.mset(x, y, 0, Tiles::Grass as u8);
            }
        }

        //Play BGM
        api.music("bgm", 0,0); //Uncomment after adding music.mp3
    }

    fn update(&mut self, api: &mut BBMicroApi) {
        if api.btn(Button::RIGHT) {
            if self.p1x < 200.0{
                self.p1x += 2.0;
            }else{
                self.p1w = true;
        }
    }
        if api.btn(Button::D){
            if self.p2x < 200.0 {
                self.p2x += 2.0;
            }else{
                self.p2w = true;
            }
        }
        if api.btn(Button::A) {
            self.green_light = true;
        }
        if api.btn(Button::B) {
            self.green_light = false;
        }

        self.stop_light.top = if self.green_light {Tiles::GrTop as u8} else {Tiles::RDTop as u8};
        self.stop_light.bott = if self.green_light {Tiles::GRBott as u8} else {Tiles::RDBott as u8};

        self.stop_light.x = self.x;
        self.stop_light.y = self.y - 48.0;
    }

    fn draw(&mut self, api: &mut BBMicroApi) {
        api.camera(self.x - 60.0, self.y - 60.0);

        // Draw map layer 0.
        api.map(0, 0, 0.0, 0.0, 256, 256, 0);

        let spr1 = 8;
        let spr2 = 9;

        api.rect(10.0, 10.0, 20.0, 20.0, 1);

        //Makes sprites
        api.spr(spr1, self.p1x - 60.0, self.p1y, 8.0, 8.0, false, false);
        api.spr(spr2, self.p2x - 60.0, self.p2y-20.0, 8.0, 8.0, false, false);

        //Draw finish line
        api.mset(20, 10, 0, Tiles::FinishLine as u8);
        api.mset(20, 11, 0, Tiles::FinishLine as u8);
        api.mset(20, 13, 0, Tiles::FinishLine as u8);
        api.mset(20, 12, 0, Tiles::FinishLine as u8);

        api.spr(self.stop_light.top, self.stop_light.x, self.stop_light.y, 8.0, 8.0, false, false);
        api.spr(self.stop_light.bott, self.stop_light.x, self.stop_light.y + 8.0, 8.0, 8.0, false, false);

        // Draw map layer 1.
        api.map(80, 0, 0.0, 0.0, 256, 256, 1);

        if self.p1w == true{
            api.print("PLAYER ONE WINS", 5.0, 5.0, false);
        }else if self.p2w == true {
            api.print("PLAYER TWO WINS", 5.0, 5.0, false);
        }
        
    }
}
