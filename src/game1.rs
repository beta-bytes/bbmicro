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
    clean_count: u32,
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
            clean_count: 0,
        }
    }

    pub fn bound(&self, x: f32, y: f32) -> (f32, f32) {
        (
            x.min(self.width - 8.0).max(0.0),
            y.min(self.height - 8.0).max(0.0),
        )
    }

    fn ai_move(&mut self, api: &mut BBMicroApi) {
        //clean path detection
        let tile_position = self.get_tile_position((self.roomba_x, self.roomba_y));
        let avail_positions = &Game1::get_avail_positions(tile_position, api);

        //move
        if avail_positions.len() > 0 {
            if matches!(avail_positions[0], Directions::LEFT) {
                self.roomba_x -= 1.0;
            } else if matches!(avail_positions[0], Directions::RIGHT) {
                self.roomba_x += 1.0;
            } else if matches!(avail_positions[0], Directions::UP) {
                self.roomba_y -= 1.0;
            } else if matches!(avail_positions[0], Directions::DOWN) {
                self.roomba_y += 1.0
            }
        }

        let new_roomba_pos = self.bound(self.roomba_x, self.roomba_y);
        self.roomba_x = new_roomba_pos.0;
        self.roomba_y = new_roomba_pos.1;
    }

    fn get_tile_position(&mut self, position: (f32, f32)) -> (u32, u32) {
        ((position.0 + 4.0) as u32 / 8, (position.1 + 4.0) as u32 / 8)
    }

    fn get_avail_positions(current_tile: (u32, u32), api: &mut BBMicroApi) -> Vec<Directions> {
        let mut avail_positions: Vec<Directions> = Vec::new();
        let curr_x = current_tile.0 as i32;
        let curr_y = current_tile.1 as i32;

        if curr_x + 1 <= 15 && api.mget(current_tile.0 + 1, current_tile.1, 0) == Tiles::Dirty as u8
        {
            avail_positions.push(Directions::RIGHT);
        } else if curr_x - 1 >= 0
            && api.mget(current_tile.0 - 1, current_tile.1, 0) == Tiles::Dirty as u8
        {
            avail_positions.push(Directions::LEFT);
        } else if curr_y - 1 >= 0
            && api.mget(current_tile.0, current_tile.1 - 1, 0) == Tiles::Dirty as u8
        {
            avail_positions.push(Directions::UP);
        } else if curr_y + 1 <= 15
            && api.mget(current_tile.0, current_tile.1 + 1, 0) == Tiles::Dirty as u8
        {
            avail_positions.push(Directions::DOWN);
        }
        return avail_positions;
    }
}

enum Directions {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
enum Tiles {
    Dirty = 1,
    Cat = 2,
    Roomba = 3,
    Clean = 7,
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
            // self.roomba_x -= 2.0;
        }
        if api.btn(Button::RIGHT) {
            self.cat_x += 2.0;
            // self.roomba_x += 2.0;
        }
        if api.btn(Button::UP) {
            self.cat_y -= 2.0;
            // self.roomba_y -= 2.0;
        }
        if api.btn(Button::DOWN) {
            self.cat_y += 2.0;
            // self.roomba_y += 2.0;
        }

        let new_cat_pos = self.bound(self.cat_x, self.cat_y);
        self.cat_x = new_cat_pos.0;
        self.cat_y = new_cat_pos.1;

        let cat_tile_position = self.get_tile_position((self.cat_x, self.cat_y));
        let roomba_tile_pos = self.get_tile_position((self.roomba_x, self.roomba_y));

        if roomba_tile_pos.0 >= 0
            && roomba_tile_pos.0 < 256
            && roomba_tile_pos.1 >= 0
            && roomba_tile_pos.1 < 256
        {
            if api.mget(roomba_tile_pos.0, roomba_tile_pos.1, 0) == Tiles::Dirty as u8 {
                api.mset(roomba_tile_pos.0, roomba_tile_pos.1, 0, Tiles::Clean as u8);
                self.clean_count += 1;
            }
        }

        if cat_tile_position.0 >= 0
            && cat_tile_position.0 < 256
            && cat_tile_position.1 >= 0
            && cat_tile_position.1 < 256
        {
            if api.mget(cat_tile_position.0, cat_tile_position.1, 0) == Tiles::Clean as u8 {
                api.mset(
                    cat_tile_position.0,
                    cat_tile_position.1,
                    0,
                    Tiles::Dirty as u8,
                );
                self.clean_count -= 1;
            }
        }

        self.ai_move(api);
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

        let clean_percent = (self.clean_count as f32) / (16.0 * 16.0);

        api.rect(105.0, 5.0, 105.0 + 20.0, 8.0, 3, true, true);
        api.rect(105.0, 5.0, 105.0 + clean_percent * 20.0, 8.0, 2, true, true);
    }
}
