use macroquad::prelude::*;

use crate::manager::Manager;

pub struct Pipe {
    manager: Manager,
    pos: Vec2,
}

impl Pipe {
    pub async fn new() -> Self {
        Self {
            manager: Manager::new().await,
            pos: Vec2 { x: screen_height()/2., y: screen_width()/2.},
        }
    }

    pub fn spawn() {
        
    }
}