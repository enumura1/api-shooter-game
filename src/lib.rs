pub mod bullet;
pub mod enemy;
pub mod game_state;
pub mod player;

pub use bullet::{Bullet, EnemyBullet};
pub use enemy::Enemy;
pub use game_state::GameState;
pub use player::Player;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let game = GameState::new();
        assert_eq!(game.score, 0);
        assert_eq!(game.enemies_defeated, 0);
        assert!(!game.game_over);
        assert!(!game.victory_screen);
        assert!(!game.enemy_weakness_revealed);
        assert!(game.enemy.is_some());
        assert_eq!(game.bullets.len(), 0);
        assert_eq!(game.enemy_bullets.len(), 0);
    }

    #[test]
    fn test_all_components_work_together() {
        let player = Player::new();
        let enemy = Enemy::new();
        let game = GameState::new();

        // Ensure all components can be created
        assert!(player.hp > 0);
        assert!(enemy.hp > 0);
        assert!(game.enemy.is_some());
    }
}
