use std::ptr;
use std::collections::LinkedList;
use tui::widgets::canvas::{Shape, Painter, Line, Context};
use tui::style::Color;
use crate::vec2::Vec2;
use crate::food::Food;

pub struct Snake {
    body: LinkedList<Vec2>,
    color: Color,
    direction: Vec2,
    speed: Vec2,
}

impl Shape for Snake {
    fn draw(&self, painter: &mut Painter) {
        let mut body = self.body.iter();
        let mut last = body.next().unwrap();

        for node in body {
            let line = Line {
                x1: last.x,
                y1: last.y,
                x2: node.x,
                y2: node.y,
                color: self.color,
            };
            line.draw(painter);
            last = node;
        }

        let head = self.get_head();
        let head_point = Line {
            x1: head.x,
            y1: head.y,
            x2: head.x,
            y2: head.y,
            color: Color::LightYellow
        };

        head_point.draw(painter);
    }
}

impl Snake {
    fn draw_segment(&self, node1: Vec2, node2: Vec2, ctx: &mut Context) {
        let x1 = node1.x as i32;
        let x2 = node2.x as i32;
        let y1 = node1.y as i32;
        let y2 = node2.y as i32;

        let x_range = if x2 > x1 {
            x1..=x2
        } else {
            x2..=x1
        };
        let y_range = if y2 > y1 {
            y1..=y2
        } else {
            y2..=y1
        };

        if x2 == x1 {
            for y in y_range {
                ctx.print(node1.x as f64, y as f64, "|", self.color);
            }
        } else {
            for x in x_range {
                ctx.print(x as f64, node2.y as f64, "-", self.color);
            }
        }
    }

    pub fn draw_big(&self, ctx: &mut Context) {
        let mut body = self.body.iter();
        let mut last = body.next().unwrap();

        for node in body {
            self.draw_segment(*last, *node, ctx);
            last = node;
        }
        ctx.layer();

        let head = self.get_head();
        ctx.print(head.x, head.y, "*", Color::LightYellow);
    }

    pub fn new(x: f64, y: f64, color: Color) -> Self {
        Self {
            body: LinkedList::from([Vec2::new(x, y), Vec2::new(x-5.0, y)]),
            color,
            direction: Vec2::new(1.0, 0.0),
            speed: Vec2::from_single(1.0),
        }
    }

    pub fn change_direction(&mut self, direction: Vec2) {
        if self.direction != direction  && self.direction != -direction {
            self.direction = direction;

            // Create a new edge at current head position
            let head = self.body.pop_front().unwrap();
            self.body.push_front(head);
            self.body.push_front(head);
        }
    }

    pub fn grow(&mut self, multiplier: Vec2) {
        let mut prev_tail = self.body.pop_back().unwrap();
        let tail = self.body.pop_back().unwrap();

        if tail == prev_tail {
            self.body.push_back(tail);
            self.grow(multiplier);
            return;
        }

        prev_tail += tail.direction_to(&prev_tail) * multiplier;
        self.body.push_back(tail);
        self.body.push_back(prev_tail);
    }

    pub fn advance(&mut self) {
        let mut head = self.body.pop_front().unwrap();
        head += self.direction * self.speed;
        self.body.push_front(head);

        self.grow(-self.speed);
    }

    fn nodes_collide(target: &Vec2, node1: &Vec2, node2: &Vec2) -> bool {
        if node1.x == node2.x && node1.x == target.x {
            if node1.y < node2.y {
                target.y >= node1.y && target.y <= node2.y
            } else {
                target.y <= node1.y && target.y >= node2.y
            }
        } else if node1.y == node2.y && node1.y == target.y {
            if node1.x < node2.x {
                target.x >= node1.x && target.x <= node2.x
            } else {
                target.x <= node1.x && target.x >= node2.x
            }
        } else {
            false
        }
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        let my_head = self.get_head();

        let mut nodes = other.body.iter();
        let mut last = nodes.next().unwrap();

        // If checking against itself, skip first section
        if ptr::eq(self, other) {
            last = nodes.next().unwrap();
        }

        for node in nodes {
            if Self::nodes_collide(my_head, last, node) {
                return true;
            }
            last = node;
        }

        false
    }

    pub fn get_head(&self) -> &Vec2 {
        self.body.iter().next().unwrap()
    }

    pub fn try_eat(&mut self, food_pool: &mut Vec<Food>) {
        for i in 0..food_pool.len() {
            if (*food_pool[i].get_pos() - *self.get_head()).len() <= self.speed.x + 2.0 {
                food_pool.swap_remove(i);
                self.grow(Vec2::from_single(12.0));
                break;
            }
        }
    }
}
