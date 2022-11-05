use macroquad::window::Conf;
use macroquad::prelude::*;
use manager::Manager;
use pipe::Pipe;

mod manager;
mod pipe;
mod bird;

pub enum State {
    PLAYING,
    MENU,
    DEAD,
}

#[macroquad::main(window_config)]
async fn main() {
    let manager = Manager::new().await;
    let mut arr: Vec<Pipe> = vec![];

    let mut timer: u16 = 0;
    let mut despawn: u16 = 0;

    let mut player = bird::Player::new().await;
    let mut game_state = State::MENU;

    loop {
        // draw background
        draw_texture(manager.background,0.,0.,WHITE);

        // manage game states
        match game_state {
            State::PLAYING => {
                despawn += 1;
                timer += 1;

                // player
                player.update(&mut game_state);
                player.draw();

                // pipes
                if timer > 100 {
                    timer = 0;
                    Pipe::spawn(&mut arr).await;
                }

                if despawn > 300 {
                    despawn = 0;
                    // remove pipes out of bounds
                    for _ in 0..2 {
                        arr.remove(0);
                    }
                }

                Pipe::draw(&mut arr).await;
                Pipe::update(&mut arr)
            },

            State::MENU => {
                draw_texture(manager.menu, screen_width()/6., screen_height()/6., WHITE);

                if is_key_pressed(KeyCode::Space) {
                    game_state = State::PLAYING;
                }
            },

            State::DEAD => {
                draw_texture(manager.gameover, screen_width()/6., screen_height()/6., WHITE);
                player.reset();

                if is_key_pressed(KeyCode::Space) {
                    game_state = State::MENU;
                }
            }
        }

        next_frame().await;
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: String::from("Flappy bird"),
        window_resizable: false,

        window_width: 288,
        window_height: 512,

        ..Default::default()
    }
}
