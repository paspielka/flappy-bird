use macroquad::{prelude::*, rand::RandomRange, audio::{Sound, play_sound, PlaySoundParams}};

use crate::State;

pub struct Pipe {
    pos: Vec2,
}

impl Pipe {
    pub async fn new(_pos: Vec2) -> Self {
        Self {
            pos: _pos,
        }
    }

    pub async fn spawn(arr: &mut Vec<Pipe>, pos: Vec2) {
        // generate offset to spawn the pipes at random intervals
        let offset = RandomRange::gen_range(0, 150) as f32;

        arr.push(Pipe::new(pos - Vec2 { x: 0., y: 0. - offset }).await);
        arr.push(Pipe::new(pos - Vec2 { x: 0., y: 420. -offset }).await);
    }

    pub async fn draw(arr: &mut [Pipe], texture_up: Texture2D, texture_down: Texture2D, player_hitbox: &Rect, game_state: &mut State, hit: Sound) {
        for pipe in arr.iter() {
            // draw differently depending on rotation
            if pipe.pos.y < 250.{
                Pipe::draw_boxes(pipe, texture_up, player_hitbox, game_state, hit);
            }
            else {
                Pipe::draw_boxes(pipe, texture_down, player_hitbox, game_state, hit);
            }
        }
    }

    pub fn draw_boxes(&self, texture: Texture2D, player_hitbox: &Rect, game_state: &mut State, hit: Sound) {
        draw_texture(texture, self.pos.x, self.pos.y, WHITE);

        let hitbox = Rect {
            x: self.pos.x,
            y: self.pos.y,
            w: 52.,
            h: 320.
        };

        // player touched a pipe
        if !hitbox.overlaps(player_hitbox) {
            return;
        }
        *game_state = State::DEAD;
        play_sound(hit, PlaySoundParams { looped: false, volume: 0.2 });
    }

    pub fn update(arr: &mut Vec<Pipe>) {
        // move pipes
        for pipe in arr.iter_mut() {
            pipe.pos.x -= 2.;
        }

        arr.retain(|x| x.pos.x > -40.);
    }
}
