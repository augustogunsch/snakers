mod snake;
mod vec2;
mod event;
mod food;

use std::sync::Mutex;
use std::io;
use std::thread;
use std::time::Duration;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders};
use tui::style::Color;
use tui::widgets::canvas::{Canvas};
use termion::screen::AlternateScreen;
use termion::raw::IntoRawMode;
use termion::event::Key;

use event::Events;
use snake::Snake;
use vec2::Vec2;
use food::Food;

const SCREEN_WIDTH: f64 = 360.0;
const SCREEN_HEIGHT: f64 = 180.0;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut snake = Snake::new(0.0, 0.0, Color::LightBlue);
    let mut food_pool: Vec<Food> = Vec::new();

    while food_pool.len() < 15 {
        food_pool.push(Food::new());
    }

    let food_pool = Mutex::new(food_pool);

    let events = Events::new();
    loop {
        thread::sleep(Duration::from_millis(15));

        terminal.draw(|f| {
            let canvas = Canvas::default()
                .block(Block::default()
                             .title("Canvas")
                             .borders(Borders::ALL))
                             .x_bounds([-SCREEN_WIDTH/2.0, SCREEN_WIDTH/2.0])
                             .y_bounds([-SCREEN_HEIGHT/2.0, SCREEN_HEIGHT/2.0])
                .paint(|ctx| {
                    //ctx.draw(&snake);
                    snake.draw_big(ctx);

                    for food in &*food_pool.lock().unwrap() {
                        food.draw(ctx);
                    }
                });

            f.render_widget(canvas, f.size());
        })?;

        snake.advance();

        if snake.collides_with(&snake) {
            return Ok(());
        }

        for key in events.last() {
            match key {
                Key::Ctrl(c) => if c == 'c' { return Ok(()); },
                Key::Up => snake.change_direction(Vec2::new(0.0, 1.0)),
                Key::Left => snake.change_direction(Vec2::new(-1.0, 0.0)),
                Key::Down => snake.change_direction(Vec2::new(0.0, -1.0)),
                Key::Right => snake.change_direction(Vec2::new(1.0, 0.0)),
                _ => {}
            }
        }
        let mut food_pool = food_pool.lock().unwrap();
        snake.try_eat(&mut food_pool);

        for food in &mut *food_pool {
            food.tick_spoil();
        }

        for i in 0..food_pool.len() {
            if food_pool[i].rotten() {
                food_pool.swap_remove(i);
                food_pool.push(Food::new());
            }
        }
    }
}
