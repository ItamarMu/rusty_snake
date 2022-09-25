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
const SPEED: u64 = SPEED_4;

struct Board {active: bool, start: i32}

impl Board { // 14 X 20
    pub fn nodes (&self) -> Vec<[f64; 4]> {
        (self.start+0..self.start+5)
        .map(|i| Node::new(i * (SIZE as i32), 1 * (SIZE as i32)).display_data())
        .collect::<Vec<_>>()
    }

    pub fn update(&mut self) {
        self.start += 1;
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            active: true,
            start: 0
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
        [self.x as f64, self.y as f64, SIZE, SIZE]
    }
}

fn main() {
    let mut frame: u64 = 0;
    let speed = SPEED;
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (WIDTH * (SIZE as u32), HEIGHT * (SIZE as u32)))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    window.set_event_settings(EventSettings::new().ups(UPS));

    let mut board = Board::default();
    let sq = board.nodes();
    let mut pos_x = 50.0;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear(color::PURPLE, g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [123.0, 50.0, 150.0, 50.0],
                _c.transform,
                g,
            );
            rectangle(
                color::GREEN,
                [pos_x, 50.0, 150.0, 50.0],
                _c.transform,
                g,
            );
            for x in board.nodes().iter() {
                rectangle(
                    color::GREEN,
                    *x,
                    _c.transform,
                    g,
                );
            }
        });
        if let Some(args) = e.update_args() {
            pos_x += 0.5;
            if frame % (UPS / speed) == 0 {
                board.update();
            }
            frame = (frame + 1) % UPS;
            // rotation += 0.25;
        }
    }
}

