use ::rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub weakness: String,
    pub speed: f32,
    pub direction: f32,
    pub move_timer: f32,
}

impl Enemy {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let enemy_types = vec!["User", "Order", "Product", "Payment"];
        let weaknesses = vec!["GET", "POST", "PUT"];

        let enemy_type = enemy_types[rng.gen_range(0..enemy_types.len())];
        let id = rng.gen_range(100..999);
        let hp = rng.gen_range(80..120);

        Enemy {
            x: rng.gen_range(80.0..720.0),
            y: rng.gen_range(140.0..260.0), // 20px下げた (120→140, 240→260)
            size: 35.0,
            name: format!("{}-{}", enemy_type, id),
            hp,
            max_hp: hp,
            weakness: weaknesses[rng.gen_range(0..weaknesses.len())].to_string(),
            speed: rng.gen_range(120.0..200.0),
            direction: if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
            move_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.move_timer += dt;

        if self.move_timer > 1.5 {
            let mut rng = thread_rng();
            if rng.gen_bool(0.7) {
                self.direction *= -1.0;
            }
            if rng.gen_bool(0.3) {
                self.y += rng.gen_range(-30.0..30.0);
                self.y = self.y.clamp(140.0, 260.0); // 移動範囲も20px下げた
            }
            self.move_timer = 0.0;
        }

        self.x += self.direction * self.speed * dt;

        if self.x < 80.0 || self.x > 720.0 {
            self.direction *= -1.0;
            self.x = self.x.clamp(80.0, 720.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_creation() {
        let enemy = Enemy::new();
        // HP validation
        assert!(enemy.hp > 0);
        assert!(enemy.max_hp > 0);
        assert!(enemy.hp >= 80 && enemy.hp <= 120);
        assert_eq!(enemy.hp, enemy.max_hp);

        // Weakness validation
        assert!(["GET", "POST", "PUT"].contains(&enemy.weakness.as_str()));

        // Position validation
        assert!(enemy.x >= 80.0 && enemy.x <= 720.0);
        assert!(enemy.y >= 140.0 && enemy.y <= 260.0);

        // Name format validation
        assert!(enemy.name.contains("-"));
        assert!(enemy.name.len() > 5); // "Type-123" format
    }

    #[test]
    fn test_enemy_types() {
        // Test multiple enemies to ensure variety
        let mut enemy_types = std::collections::HashSet::new();
        let mut weaknesses = std::collections::HashSet::new();

        for _ in 0..20 {
            let enemy = Enemy::new();
            let parts: Vec<&str> = enemy.name.split('-').collect();
            enemy_types.insert(parts[0].to_string());
            weaknesses.insert(enemy.weakness.clone());
        }

        // Should have multiple enemy types
        assert!(enemy_types.len() > 1);
        // Should have multiple weakness types
        assert!(weaknesses.len() > 1);
    }

    #[test]
    fn test_enemy_movement_bounds() {
        let mut enemy = Enemy::new();

        // Test boundary clamping
        enemy.x = -10.0; // Below minimum
        enemy.update(0.1);
        assert!(enemy.x >= 80.0);

        enemy.x = 800.0; // Above maximum
        enemy.update(0.1);
        assert!(enemy.x <= 720.0);
    }

    #[test]
    fn test_weakness_coverage() {
        let weaknesses = ["GET", "POST", "PUT"];

        // Test that all weakness types can be generated
        let mut found_weaknesses = std::collections::HashSet::new();
        for _ in 0..50 {
            let enemy = Enemy::new();
            found_weaknesses.insert(enemy.weakness.clone());
        }

        // Should find all 3 weakness types
        for weakness in &weaknesses {
            assert!(
                found_weaknesses.contains(*weakness),
                "Weakness '{}' was not generated",
                weakness
            );
        }
    }

    #[test]
    fn test_enemy_name_format() {
        let enemy = Enemy::new();
        let parts: Vec<&str> = enemy.name.split('-').collect();

        // Should have exactly 2 parts: Type-Number
        assert_eq!(parts.len(), 2);

        // First part should be valid enemy type
        assert!(["User", "Order", "Product", "Payment"].contains(&parts[0]));

        // Second part should be a number
        let id: Result<i32, _> = parts[1].parse();
        assert!(id.is_ok());
        let id_num = id.unwrap();
        assert!(id_num >= 100 && id_num <= 999);
    }
}
