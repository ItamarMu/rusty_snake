extern crate piston_window;
use std::hash::Hash;

use piston_window::*;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

mod color {
    pub const PURPLE: [f32; 4] = [0.21, 0.157, 0.64, 1.0];
    pub const GREEN: [f32; 4] = [0.0, 0.44, 0.0, 1.0];
    pub const LIGHT_GREEN: [f32; 4] = [0.0, 0.6, 0.3, 1.0];
    pub const RED: [f32; 4] = [0.8, 0.1, 0.1, 1.0];
}

const WIDTH: usize = 20;
const HEIGHT: usize = 14;
const SIZE: f64 = 40.0;
const SPEED_1: u64 = 1; // 1ups
const SPEED_2: u64 = 3; // 3ups
const SPEED_3: u64 = 5; // 5ups
const SPEED_4: u64 = 10; // 10ups
const SPEED_5: u64 = 20; // 20ups
const SPEED: u64 = SPEED_5;

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const RIGHT: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (-1, 0);

struct Snake {
    direction: (i32, i32),
    last_direction: (i32, i32),
    nodes: Vec<Node>,
    food: Node,
    moves: Vec<piston_window::Key>,
}

impl Snake {
    pub fn nodes_display_data(&mut self) -> [[&[f32; 4]; HEIGHT]; WIDTH] {
        let head = self.nodes.get(0).unwrap();
        let mut grid = [[&[0f32; 4]; HEIGHT]; WIDTH];

        for node in &self.nodes {
            grid[node.x][node.y] = &color::GREEN;
        }
        grid[head.x][head.y] = &color::LIGHT_GREEN;

        grid
    }

    pub fn read_direction(&mut self) {
        if let Some(next_move) = self.moves.pop() {
            self.direction = match next_move {
                Key::Up if self.last_direction != DOWN => UP,
                Key::Down if self.last_direction != UP => DOWN,
                Key::Right if self.last_direction != LEFT => RIGHT,
                Key::Left if self.last_direction != RIGHT => LEFT,
                _ => self.direction,
            }
        }
    }

    pub fn handle_key(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            self.moves.insert(0, key);
        }
    }

    pub fn new_food(&mut self) {
        let mut rng = rand::thread_rng();
        let x = Uniform::new(0, WIDTH).sample(&mut rng);
        let y = Uniform::new(0, HEIGHT).sample(&mut rng);
        self.food = Node::new(x, y);
    }

    pub fn head<'a>(&'a self) -> &'a Node {
        &self.nodes[0]
    }

    pub fn update(&mut self) {
        self.read_direction();
        self.last_direction = self.direction;

        if *self.head() == self.food {
            self.new_food();
        } else {
            self.nodes.pop();
        }

        let head = self.head();
        self.nodes.insert(
            0,
            Node::new(
                (head.x as i32 + self.direction.0).rem_euclid(WIDTH as i32) as usize,
                (head.y as i32 + self.direction.1).rem_euclid(HEIGHT as i32) as usize,
            ),
        );
    }
}

impl Default for Snake {
    fn default() -> Snake {
        Snake {
            direction: (1, 0),
            last_direction: (1, 0),
            nodes: vec![
                Node::new(8, 4),
                Node::new(7, 4),
                Node::new(6, 4),
                Node::new(5, 4),
            ],
            food: Node::new(12, 12),
            moves: vec![],
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Node {
    x: usize,
    y: usize,
}

impl From<&Node> for [f64; 4] {
    fn from(node: &Node) -> [f64; 4] {
        [node.x as f64 * SIZE, node.y as f64 * SIZE, SIZE, SIZE]
    }
}

impl Node {
    fn new(x: usize, y: usize) -> Node {
        Self { x, y }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Rusty Snake",
        (WIDTH as u32 * (SIZE as u32), HEIGHT as u32 * (SIZE as u32)),
    )
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.set_event_settings(EventSettings::new().ups(SPEED));

    let mut snake = Snake::default();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear(color::PURPLE, g);
            let display_data = snake.nodes_display_data();
            for (i, row) in display_data.iter().enumerate() {
                for (j, _) in row.iter().enumerate() {
                    rectangle(
                        *display_data[i][j],
                        [i as f64 * SIZE, j as f64 * SIZE, SIZE, SIZE],
                        _c.transform,
                        g,
                    );
                }
            }
            rectangle(color::RED, <[f64; 4]>::from(&snake.food), _c.transform, g);
        });

        if let Some(b) = e.press_args() {
            snake.handle_key(&b);
        }

        if e.update_args().is_some() {
            snake.update();
        }
    }
}
