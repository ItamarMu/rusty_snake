extern crate piston_window;
use piston_window::*;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

mod color {
    pub const PURPLE: [f32; 4] = [0.21, 0.157, 0.64, 1.0];
    pub const GREEN: [f32; 4] = [0.0, 0.44, 0.0, 1.0];
    pub const LIGHT_GREEN: [f32; 4] = [0.0, 0.6, 0.3, 1.0];
    pub const RED: [f32; 4] = [0.8, 0.1, 0.1, 1.0];
}

const WIDTH: u32 = 20;
const HEIGHT: u32 = 14;
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
    // 14 X 20
    pub fn nodes_display_data(&mut self) -> Vec<[f64; 4]> {
        self.nodes
            .iter()
            .map(|node| node.display_data())
            .collect::<Vec<_>>()
    }

    pub fn read_direction(&mut self) {
        if let Some(next_move) = self.moves.pop() {
            self.direction = match next_move {
                Key::K if self.last_direction != DOWN => UP,
                Key::J if self.last_direction != UP => DOWN,
                Key::L if self.last_direction != LEFT => RIGHT,
                Key::H if self.last_direction != RIGHT => LEFT,
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
        let x = Uniform::new(0, WIDTH).sample(&mut rng) as i32;
        let y = Uniform::new(0, HEIGHT).sample(&mut rng) as i32;
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
                (head.x + self.direction.0).rem_euclid(WIDTH as i32),
                (head.y + self.direction.1).rem_euclid(HEIGHT as i32),
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

#[derive(PartialEq)]
struct Node {
    x: i32,
    y: i32,
}

impl Node {
    fn new(x: i32, y: i32) -> Node {
        Self { x: x, y: y }
    }

    fn display_data(&self) -> [f64; 4] {
        [self.x as f64 * SIZE, self.y as f64 * SIZE, SIZE, SIZE]
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Rusty Snake",
        (WIDTH * (SIZE as u32), HEIGHT * (SIZE as u32)),
    )
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.set_event_settings(EventSettings::new().ups(SPEED));

    let mut snake = Snake::default();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear(color::PURPLE, g);
            let snake_nodes = snake.nodes_display_data();
            for x in snake_nodes[1..].iter() {
                rectangle(color::GREEN, *x, _c.transform, g);
            }
            rectangle(color::LIGHT_GREEN, snake_nodes[0], _c.transform, g);
            rectangle(color::RED, snake.food.display_data(), _c.transform, g);
        });

        if let Some(b) = e.press_args() {
            snake.handle_key(&b);
        }

        if e.update_args().is_some() {
            snake.update();
        }
    }
}
