extern crate piston_window;
use std::time::{SystemTime, UNIX_EPOCH};
use piston_window::*;

mod color {
    pub const PURPLE: [f32; 4] = [0.21, 0.157, 0.64, 1.0];
    pub const GREEN:  [f32; 4] = [0.0, 0.44, 0.0, 1.0];
}

const WIDTH: u32 = 20;
const HEIGHT: u32 = 14;
const SIZE: f64 = 30.0;
const UPS: u64 = 60; // 1 - 20
const SPEED_1: u64 = 1; // 1ups
const SPEED_2: u64 = 3; // 3ups
const SPEED_3: u64 = 5; // 5ups
const SPEED_4: u64 = 10; // 10ups
const SPEED_5: u64 = 20; // 20ups
const SPEED: u64 = SPEED_3;

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const RIGHT: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (-1, 0);

struct Snake {direction: (i32, i32), last_direction: (i32, i32), nodes: Vec<Node>}

impl Snake { // 14 X 20
    pub fn nodes_display_data (&mut self) -> Vec<[f64; 4]> {
        self.nodes.iter().map(|node| node.display_data()).collect::<Vec<_>>()
    }

    pub fn handle_key (&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            self.direction = match key {
                Key::Up if self.last_direction != DOWN => UP,
                Key::Down if self.last_direction != UP => DOWN,
                Key::Right if self.last_direction != LEFT => RIGHT,
                Key::Left if self.last_direction != RIGHT => LEFT,
                _ => self.direction
            }
        }
    }

    pub fn update(&mut self) {
        self.last_direction = self.direction;
        self.nodes.pop();
        let head = &self.nodes[0];
        self.nodes.insert(0, Node::new(head.x + self.direction.0, head.y + self.direction.1));
    }
}

impl Default for Snake {
    fn default() -> Snake {
        Snake {
            direction: (1, 0),
            last_direction: (1, 0),
            nodes: vec![Node::new(8, 4), Node::new(7, 4), Node::new(6, 4), Node::new(5, 4)]
        }
    }
}

struct Node {
    x: i32, y: i32
}

impl Node {
    fn new(x: i32, y: i32) -> Node {
        Self {x: x, y: y}
    }

    fn display_data(&self) -> [f64; 4]{
        [self.x as f64 * SIZE, self.y as f64 * SIZE, SIZE, SIZE]
    }
}

fn main() {
    let mut frame: u64 = 0;
    let speed = SPEED;
    let mut window: PistonWindow = WindowSettings::new("Rusty Snake", (WIDTH * (SIZE as u32), HEIGHT * (SIZE as u32)))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    window.set_event_settings(EventSettings::new().ups(UPS));

    let mut snake = Snake::default();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear(color::PURPLE, g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [123.0, 50.0, 150.0, 50.0],
                _c.transform,
                g,
            );
            for x in snake.nodes_display_data().iter() {
                rectangle(
                    color::GREEN,
                    *x,
                    _c.transform,
                    g,
                );
            }
        });

        if let Some(b) = e.press_args() {
            snake.handle_key(&b);
        }

        if let Some(_) = e.update_args() {
            if frame % (UPS / speed) == 0 {
                snake.update();
            }
            frame = (frame + 1) % UPS;
        }
    }
}

