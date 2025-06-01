#[derive(Debug, Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub speed: f32,
    pub get_ammo: i32,
    pub post_ammo: i32,
    pub put_ammo: i32,
    pub delete_ammo: i32,
    pub hp: i32,
    pub max_hp: i32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 400.0,
            y: 500.0,
            size: 30.0,
            speed: 300.0,
            get_ammo: 99,
            post_ammo: 20,
            put_ammo: 20,
            delete_ammo: 3,
            hp: 100,
            max_hp: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new();
        assert_eq!(player.hp, 100);
        assert_eq!(player.max_hp, 100);
        assert_eq!(player.get_ammo, 99);
        assert_eq!(player.post_ammo, 20);
        assert_eq!(player.put_ammo, 20);
        assert_eq!(player.delete_ammo, 3);
        assert_eq!(player.x, 400.0);
        assert_eq!(player.y, 500.0);
    }

    #[test]
    fn test_player_initial_position() {
        let player = Player::new();

        // Should be in center-bottom area
        assert_eq!(player.x, 400.0); // Center of 800px width
        assert_eq!(player.y, 500.0); // Near bottom of 600px height
        assert_eq!(player.size, 30.0);
        assert_eq!(player.speed, 300.0);
    }
}
