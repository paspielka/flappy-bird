use macroquad::audio::PlaySoundParams;
use macroquad::audio::play_sound;
use macroquad::prelude::*;

use crate::Manager;
use crate::State;

pub struct Player {
    jump_power: f32,
    velocity: f32,
    gravity: f32,

    pos: Vec2,

    manager: Manager,
}

impl Player {
    pub async fn new() -> Self {
        Self {
            jump_power: 7.,
            velocity: -5.,
            gravity: 0.2,  

            pos: Vec2 {
                x: screen_width()/2.3,
                y: screen_height()/2.
            },
            
            manager: Manager::new().await,
        }
    }

    // draws every frame
    pub fn draw(&self) {
        draw_texture_ex(
            self.manager.bird,
            self.pos.x,
            self.pos.y,

            WHITE,

            DrawTextureParams {
                rotation: self.velocity * 0.1,

                ..Default::default()
            }
        )
    }

    // updates player stats every frame
    pub fn update(&mut self, state: &mut State) {
        self.velocity += self.gravity;
        self.pos.y += self.velocity;

        // handle keyboard input
        if is_key_pressed(KeyCode::Space) {
            self.velocity -= self.jump_power;
            play_sound(self.manager.wing, PlaySoundParams { looped: false, volume: 0.2 });
        }

        // handle possible death cases
        if self.pos.y > 500. {
            *state = State::DEAD;
            play_sound(self.manager.hit, PlaySoundParams{ looped: false, volume: 0.2 });
        }
        else if self.pos.y < 0. {
            *state = State::DEAD;
            play_sound(self.manager.hit, PlaySoundParams{ looped: false, volume: 0.2 });
        }
    }

    // reset player on death
    pub fn reset(&mut self) {
        self.pos.y = screen_height()/2.;
        self.velocity = -5.;
    }
}