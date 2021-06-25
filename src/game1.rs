use crate::api::{BBMicroApi, BBMicroGame, Button};

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: other.x + self.x,
            y: other.y + self.y,
        }
    }
}

struct Heart {
    pt: Point,
    color: u8,
}

#[derive(Copy, Clone)]
enum EnemyAI {
    Straight = 0,
}

struct Enemy {
    pt: Point,
    color: Tiles,
    ai: EnemyAI,
}

pub struct Game1 {
    height: f32,
    width: f32,
    cat_pt: Point,
    rng: ThreadRng,
    star_count: u32,
    hearts: Vec<Heart>,
    enemies: Vec<Enemy>,
    color_spaces: Vec<u8>,
}

impl Game1 {
    pub fn new() -> Game1 {
        Game1 {
            height: 128.0,
            width: 128.0,
            cat_pt: Point { x: 64.0, y: 80.0 },
            rng: rand::thread_rng(),
            star_count: 0,
            hearts: vec![],
            enemies: vec![],
            color_spaces: vec![
                Tiles::Red as u8,
                Tiles::Orange as u8,
                Tiles::Yellow as u8,
                Tiles::Purple as u8,
                Tiles::Blue as u8,
                Tiles::Green as u8,
                Tiles::Violet as u8,
            ],
        }
    }

    fn get_tile_position(&mut self, position: Point) -> (u32, u32) {
        ((position.x + 4.0) as u32 / 8, (position.y + 4.0) as u32 / 8)
    }

    fn legal_move(&mut self, api: &mut BBMicroApi, p: Point) -> bool {
        let tile_pos = self.get_tile_position(p);
        if tile_pos.0 <= 0 {
            return false;
        }
        self.color_spaces
            .contains(&api.mget(tile_pos.0, tile_pos.1, 0))
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
    VioletRoomba = 60,
    PurpleRoomba = 59,
    GreenRoomba = 58,
    BlueRoomba = 57,
    YellowRoomba = 56,
    OrangeRoomba = 55,
    RedRoomba = 54,
    VioletHeart = 44,
    PurpleHeart = 43,
    BlueHeart = 42,
    GreenHeart = 41,
    YellowHeart = 40,
    OrangeHeart = 39,
    RedHeart = 38,
    Star1 = 49,
    Star2 = 50,
    Star3 = 51,
    Star4 = 52,
    Star5 = 53,
}

impl BBMicroGame for Game1 {
    fn init(&mut self, api: &mut BBMicroApi) {
        // Draw the base map on layer 0.
        for x in 0..16 {
            for y in 0..16 {
                let choice = [
                    Tiles::Star1,
                    Tiles::Star2,
                    Tiles::Star3,
                    Tiles::Star4,
                    Tiles::Star5,
                ]
                .choose(&mut self.rng)
                .unwrap();
                api.mset(x, y, 0, *choice as u8);
            }
        }

        api.music("bgm", 0, 0);
    }

    fn update(&mut self, api: &mut BBMicroApi) {
        let mut mv_x = Point { x: 0.0, y: 0.0 };
        let mut mv_y = Point { x: 0.0, y: 0.0 };

        if api.btn(Button::LEFT) {
            mv_x.x -= 2.0;
        }
        if api.btn(Button::RIGHT) {
            mv_x.x += 2.0;
        }
        if api.btn(Button::UP) {
            mv_y.y -= 2.0;
        }
        if api.btn(Button::DOWN) {
            mv_y.y += 2.0;
        }

        // First go right
        if self.legal_move(api, self.cat_pt.add(&mv_x)) {
            self.cat_pt = self.cat_pt.add(&mv_x)
        }

        if self.legal_move(api, self.cat_pt.add(&mv_y)) {
            self.cat_pt = self.cat_pt.add(&mv_y)
        }

        let curr_point = self.get_tile_position(self.cat_pt);
        let curr_color = api.mget(curr_point.0, curr_point.1, 0);
        let heart_color: u8;
        match curr_color {
            13 => heart_color = 38,
            14 => heart_color = 39,
            15 => heart_color = 40,
            31 => heart_color = 43,
            30 => heart_color = 42,
            29 => heart_color = 41,
            45 => heart_color = 44,
            _ => heart_color = 43,
        }

        if api.btnp(Button::A) {
            self.hearts.push(Heart {
                pt: Point {
                    x: self.cat_pt.x,
                    y: self.cat_pt.y - 4.0,
                },
                color: heart_color,
            })
        }

        for heart in &mut self.hearts {
            heart.pt.y -= 4.0;
        }

        // Spawn Enemies
        let chance: f32 = self.rng.gen();
        if chance > 0.5 {
            // Spawn an enemy
            let choice = [
                Tiles::BlueRoomba,
                Tiles::RedRoomba,
                Tiles::GreenRoomba,
                Tiles::YellowRoomba,
                Tiles::PurpleRoomba,
                Tiles::VioletRoomba,
            ]
            .choose(&mut self.rng)
            .unwrap();

            let ai_choice = [EnemyAI::Straight].choose(&mut self.rng).unwrap();

            self.enemies.push(Enemy {
                pt: Point {
                    x: self.rng.gen::<f32>() * 128.0,
                    y: -1.0 as f32,
                },
                ai: *ai_choice,
                color: *choice,
            });
        }

        for enemy in &mut self.enemies {
            enemy.pt.y += 4.0;
        }
    }

    fn draw(&mut self, api: &mut BBMicroApi) {
        // Set the tiles behind.
        for x in 0..16 {
            for y in 0..16 {
                let chance: f32 = self.rng.gen();
                if chance > 0.95 {
                    let choice = [
                        Tiles::Star1,
                        Tiles::Star2,
                        Tiles::Star3,
                        Tiles::Star4,
                        Tiles::Star5,
                    ]
                    .choose(&mut self.rng)
                    .unwrap();
                    api.mset(x, y, 0, *choice as u8);
                }
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

        // Draw map layer 0.
        api.map(0, 0, 0.0, 0.0, 256, 256, 0);

        api.spr(
            Tiles::Cat as u8,
            self.cat_pt.x,
            self.cat_pt.y,
            8.0,
            8.0,
            false,
            false,
        );

        for heart in &self.hearts {
            api.spr(heart.color, heart.pt.x, heart.pt.y, 8.0, 8.0, false, false);
        }

        for enemy in &self.enemies {
            api.spr(
                enemy.color as u8,
                enemy.pt.x,
                enemy.pt.y,
                8.0,
                8.0,
                false,
                false,
            );
        }
    }
}
