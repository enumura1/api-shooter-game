use api_shooter_cli::GameState;
use macroquad::prelude::*;

#[macroquad::main("API Shooter")]
async fn main() {
    let mut game = GameState::new();

    loop {
        let dt = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::R) && game.game_over {
            game = GameState::new();
        }

        game.update(dt);
        game.draw();

        next_frame().await
    }
}
