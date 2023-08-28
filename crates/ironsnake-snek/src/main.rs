extern crate piston_window;
extern crate rand;
extern crate gfx_device_gl;

use piston_window::glyph_cache::rusttype::GlyphCache;

use piston_window::*;
use rand::Rng;
use std::collections::LinkedList;

const CELL_SIZE: f64 = 20.0;
const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Snake {
    direction: Direction,
    body: LinkedList<(f64, f64)>,
    grow: bool,
}

struct Food {
    position: (f64, f64),
}

impl Snake {
    fn new() -> Snake {
        let mut body = LinkedList::new();
        body.push_back((5.0, 5.0));
        Snake {
            direction: Direction::Right,
            body,
            grow: false,
        }
    }

    fn update(&mut self) {
        let head = *self.body.front().unwrap();
        let new_head = match self.direction {
            Direction::Right => (head.0 + 1.0, head.1),
            Direction::Left => (head.0 - 1.0, head.1),
            Direction::Up => (head.0, head.1 - 1.0),
            Direction::Down => (head.0, head.1 + 1.0),
        };

        self.body.push_front(new_head);
        if !self.grow {
            self.body.pop_back().unwrap();
        } else {
            self.grow = false;
        }
    }

    fn collides_with(&self, position: (f64, f64)) -> bool {
        self.body.contains(&position)
    }

    fn collides_with_self(&self) -> bool {
        let head = *self.body.front().unwrap();
        for segment in self.body.iter().skip(1) {
            if *segment == head {
                return true;
            }
        }
        false
    }

    fn head_out_of_bounds(&self) -> bool {
        let head = *self.body.front().unwrap();
        head.0 < 0.0
            || head.0 >= (WIDTH / CELL_SIZE)
            || head.1 < 0.0
            || head.1 >= (HEIGHT / CELL_SIZE)
    }
}

impl Food {
    fn new() -> Food {
        Food {
            position: (
                rand::thread_rng().gen_range(0..(WIDTH / CELL_SIZE) as usize) as f64,
                rand::thread_rng().gen_range(0..(HEIGHT / CELL_SIZE) as usize) as f64,
            ),
        }
    }

    fn randomize_position(&mut self) {
        self.position = (
            rand::thread_rng().gen_range(0..(WIDTH / CELL_SIZE) as usize) as f64,
            rand::thread_rng().gen_range(0..(HEIGHT / CELL_SIZE) as usize) as f64,
        );
    }
}

fn draw_score(score: i32, c: Context, g: &mut G2d, glyph_cache: &mut GlyphCache<'static,
    piston_window::TextureContext<
        gfx_device_gl::Factory,
        gfx_device_gl::Resources,
        gfx_device_gl::CommandBuffer
    >,
    Texture<gfx_device_gl::Resources>>) {
    text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
        .draw(
            &format!("Score: {}", score),
            glyph_cache,
            &c.draw_state,
            c.transform.trans(10.0, 30.0),
            g,
        )
        .unwrap();
}

fn draw_game_over(c: Context, g: &mut G2d, glyph_cache: &mut GlyphCache<'static,
    piston_window::TextureContext<
        gfx_device_gl::Factory,
        gfx_device_gl::Resources,
        gfx_device_gl::CommandBuffer
    >,
    Texture<gfx_device_gl::Resources>>) {
    text::Text::new_color([1.0, 0.0, 0.0, 1.0], 48)
        .draw(
            "Game Over!",
            glyph_cache,
            &c.draw_state,
            c.transform.trans(WIDTH / 4.0, HEIGHT / 2.0),
            g,
        )
        .unwrap();
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Snek", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut glyph_cache = Glyphs::new(
        "./assets/FiraSans-Regular.ttf",
        texture_context,
        TextureSettings::new(),
    )
    .unwrap();

    let mut snake = Snake::new();
    let mut food = Food::new();
    let mut score = 0;
    let mut game_over = false;

    while let Some(event) = window.next() {
        // Handle input for snake direction
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Right => {
                    if snake.direction != Direction::Left {
                        snake.direction = Direction::Right;
                    }
                }
                Key::Left => {
                    if snake.direction != Direction::Right {
                        snake.direction = Direction::Left;
                    }
                }
                Key::Up => {
                    if snake.direction != Direction::Down {
                        snake.direction = Direction::Up;
                    }
                }
                Key::Down => {
                    if snake.direction != Direction::Up {
                        snake.direction = Direction::Down;
                    }
                }
                _ => {}
            }
        }

        snake.update();

        // Check for collisions with food
        if snake.collides_with(food.position) {
            score += 1;
            snake.grow = true;
            food.randomize_position();
        }

        // Check for game over conditions
        if snake.head_out_of_bounds() || snake.collides_with_self() {
            game_over = true;
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0, 1.0, 1.0, 1.0], g);

            // Draw food
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [
                    food.position.0 * CELL_SIZE,
                    food.position.1 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                ],
                c.transform,
                g,
            );

            // Draw snake
            for &(x, y) in snake.body.iter() {
                rectangle(
                    [0.0, 1.0, 0.0, 1.0],
                    [x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE],
                    c.transform,
                    g,
                );
            }

            draw_score(score, c, g, &mut glyph_cache);

            if game_over {
                draw_game_over(c, g, &mut glyph_cache);
            }
        });

        if game_over {
            break;
        }
    }
}
