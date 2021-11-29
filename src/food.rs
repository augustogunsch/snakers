use crate::vec2::Vec2;
use rand::prelude::*;
use tui::widgets::canvas::{Context};
use tui::style::Color;

use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;

pub struct Food {
    pos: Vec2,
    spoil_in: i32,
}

impl Food {
    pub fn draw(&self, ctx: &mut Context) {
        if self.spoiled() {
            ctx.print(self.pos.x, self.pos.y, "*", Color::Green);
        } else {
            ctx.print(self.pos.x, self.pos.y, "*", Color::Red);
        }
    }

    pub fn new() -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(-(SCREEN_WIDTH/2.0) as i32..(SCREEN_WIDTH/2.0) as i32);
        let y = rng.gen_range(-(SCREEN_HEIGHT/2.0) as i32..(SCREEN_HEIGHT/2.0) as i32);

        Self {
            pos: Vec2::new(x as f64, y as f64),
            spoil_in: rng.gen_range(50..300),
        }
    }

    pub fn spoiled(&self) -> bool {
        self.spoil_in >= 0
    }

    pub fn rotten(&self) -> bool {
        self.spoil_in <= -100
    }

    pub fn tick_spoil(&mut self) {
        self.spoil_in -= 1;
    }

    pub fn get_pos(&self) -> &Vec2 {
        &self.pos
    }
}
