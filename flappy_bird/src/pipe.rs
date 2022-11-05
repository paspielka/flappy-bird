use macroquad::{prelude::*, rand::RandomRange};

use crate::manager::Manager;

pub struct Pipe {
    manager: Manager,
    pos: Vec2,
}

impl Pipe {
    pub async fn new(_pos: Vec2) -> Self {
        Self {
            manager: Manager::new().await,
            pos: _pos,
        }
    }

    pub async fn spawn(arr: &mut Vec<Pipe>) {
        // generate offset to spawn the pipes at random intervals
        arr.push(Pipe::new(Vec2 { x: screen_width(), y: screen_height()/2.+RandomRange::gen_range(100, 150) as f32 }).await);
        arr.push(Pipe::new(Vec2 { x: screen_width(), y: screen_height()/2.+RandomRange::gen_range(100, 150) as f32*-3. }).await);
    }

    pub async fn draw(arr: &mut [Pipe]) {
        for pipe in arr.iter() {
            if pipe.pos.y < 250.{
                draw_texture(pipe.manager.pipe_reversed, pipe.pos.x, pipe.pos.y, WHITE);
            }
            else {
                draw_texture(pipe.manager.pipe, pipe.pos.x, pipe.pos.y, WHITE);
            }
        }
    }

    pub fn update(arr: &mut [Pipe]) {
        // move pipes
        for pipe in arr.iter_mut() {
            pipe.pos.x -= 2.;
        }
    }
}
