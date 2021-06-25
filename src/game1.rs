use crate::api::{BBMicroApi, BBMicroGame, Button};

use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::VecDeque;

pub struct Game1 {
    height: f32,
    width: f32,
    cat_x: f32,
    cat_y: f32,
    gems: VecDeque<Gem>,
    clouds: VecDeque<Cloud>,
    bullets: VecDeque<Bullet>,
    rng: ThreadRng,
    charge_bar: u32,
    frame: u32,
}

pub struct Gem {
    art: u8,
    pos_x: f32,
    pos_y: f32,
    hit: bool
}

pub struct Cloud {
    art: u8,
    pos_x: f32,
    pos_y: f32,
    width: u8
}


pub struct Bullet {
    pos_x: f32,
    pos_y: f32
}

impl Game1 {
    pub fn new() -> Game1 {
        Game1 {
            height: 128.0,
            width: 128.0,
            cat_x: 10.0,
            cat_y: 10.0,
            rng: rand::thread_rng(),
            charge_bar: 0,
            gems: VecDeque::new(),
            clouds: VecDeque::new(),
            bullets: VecDeque::new(),
            frame: 0,
        }
    }

    pub fn bound(&self, x: f32, y: f32) -> (f32, f32) {
        (
            x.min(self.width - 8.0).max(0.0),
            y.min(self.height - 8.0).max(0.0),
        )
    }

    fn get_tile_position(&mut self, position: (f32, f32)) -> (u32, u32) {
        ((position.0 + 4.0) as u32 / 8, (position.1 + 4.0) as u32 / 8)
    }

    fn update_bullets(&mut self) {
        for bullet in self.bullets.iter_mut() {
            bullet.pos_x = bullet.pos_x + 2.0;
            
            for gem in self.gems.iter_mut() {
                if (gem.pos_x - bullet.pos_x).abs() < 8.0 && (bullet.pos_y - gem.pos_y).abs() < 8.0  && !gem.hit  {
                    gem.hit = true;
                    gem.art = self.rng.gen_range(1..4) * 16; //80;
                }
            }
        }

        if self.bullets.len() > 0 && self.bullets[0].pos_x < 0.0 {
            self.bullets.pop_front();
        }
    }

    fn update_gems(&mut self) {

        for gem in self.gems.iter_mut() {
            gem.pos_x = gem.pos_x - 1.0;
            if gem.hit {
                gem.pos_y = gem.pos_y - 1.0;
            }
        }
        
        if self.gems.len() > 0 && (self.gems[0].pos_x < 0.0 || self.gems[0].pos_y < 0.0) {
            self.gems.pop_front();
        }

        if self.gems.len() < 5 && self.rng.gen_range(0..5) == 1 {
            self.gems.push_back(Gem {
                art: 4,
                pos_x: 128.0,
                pos_y: self.rng.gen_range(32..96) as f32,
                hit: false
            });
        }
    }

    fn update_clouds(&mut self) {
        
        for x in self.clouds.iter_mut() {
            x.pos_x = x.pos_x - x.pos_y / 128.0 + 0.7;
        }

        if self.clouds.len() > 0 && self.clouds[0].pos_x < 0.0 {
            self.clouds.pop_front();
        }

        if self.clouds.len() < 5 && self.rng.gen_range(0..5) == 1 {
            self.clouds.push_back(Cloud {
                art: 10,
                pos_x: 128.0,
                pos_y: self.rng.gen_range(0..128) as f32,
                width: 1
            });
        }
    }
}


enum Tiles {
    Cat = 2,
    Enemy = 3,
    Background1 = 0,
    Background2 = 16,
    Background3 = 17,
}

impl BBMicroGame for Game1 {
    fn init(&mut self, api: &mut BBMicroApi) {
        // Draw the base map on layer 0.
        for x in 0..128 {
            for y in 0..128 {
                api.mset(x, y, 0, Tiles::Background1 as u8);
                // let choice = self.rng.gen_range(0..10);
                // if choice == 2 {
                //     api.mset(x, y, 0, Tiles::Background2 as u8);
                // }
            }
        }

        // Play BGM
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

        if api.btnp(Button::A) {
            print!("pewpew\n");
            self.bullets.push_back(Bullet {
                pos_x: self.cat_x,
                pos_y: self.cat_y,
            })
        }

        let new_cat_pos = self.bound(self.cat_x, self.cat_y);
        self.cat_x = new_cat_pos.0;
        self.cat_y = new_cat_pos.1;

        let cat_tile_position = self.get_tile_position((self.cat_x, self.cat_y));

        self.update_clouds();
        self.update_gems();
        self.update_bullets();
        //update_gems();
        //self.charge_bar += 1;
        //api.mget(cat_tile_position.0, cat_tile_position.1, 0) == Tiles::Clean as u8
    }
    
    

    fn draw(&mut self, api: &mut BBMicroApi) {
        // Draw map layer 0.
        api.map(0, 0, 0.0, 0.0, 256, 256, 0);

        for cloud in self.clouds.iter_mut() {
            api.spr_abs(
                256,
                0,
                16,
                50,
                cloud.pos_x,
                cloud.pos_y,
                false,
                false,
            );
        }
        
        for gem in self.gems.iter_mut() {
            let mut anchor_y = 60;
            if gem.hit {
                anchor_y = anchor_y + 64 + gem.art
            }
            if(self.frame % 12 > 5){    
                api.spr_abs(
                    256,
                    anchor_y.into(), 
                    16,
                    16,
                    gem.pos_x,
                    gem.pos_y,
                    false,
                    false,
                );
            } else { 
                api.spr_abs(
                    256 + (self.frame % 6 * 16),
                    anchor_y.into(), 
                    16,
                    16,
                    gem.pos_x,
                    gem.pos_y,
                    false,
                    false,
                );
            }
        }
        
        api.spr_abs(
            8 + (self.frame % 8 * 32),
            180,
            16,
            16,
            self.cat_x,
            self.cat_y,
            false,
            false,
        );

        for bullet in self.bullets.iter_mut() {
            api.spr_abs(
                258,
                53,
                6,
                16,    
                bullet.pos_x,
                bullet.pos_y,
                false,
                false,
            );
        }
        

        let charge_percent = (self.charge_bar as f32) / 15.0;

        api.rect(105.0, 5.0, 105.0 + 20.0, 8.0, 3, true, true);
        api.rect(105.0, 5.0, 105.0 + charge_percent * 20.0, 8.0, 2, true, true);

        self.frame = self.frame + 1;
    }
}
