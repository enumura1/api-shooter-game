use macroquad::prelude::Color;

#[derive(Debug, Clone)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub bullet_type: String,
    pub damage: i32,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct EnemyBullet {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub damage: i32,
    pub attack_name: String,
    pub color: Color,
}

#[cfg(test)]
mod tests {
    use super::*;
    use macroquad::prelude::*;

    #[test]
    fn test_bullet_creation() {
        let bullet = Bullet {
            x: 100.0,
            y: 200.0,
            speed: 500.0,
            bullet_type: "POST".to_string(),
            damage: 25,
            color: GREEN,
        };

        assert_eq!(bullet.x, 100.0);
        assert_eq!(bullet.y, 200.0);
        assert_eq!(bullet.bullet_type, "POST");
        assert_eq!(bullet.damage, 25);
    }

    #[test]
    fn test_enemy_bullet_creation() {
        let enemy_bullet = EnemyBullet {
            x: 300.0,
            y: 150.0,
            speed: 250.0,
            damage: 20,
            attack_name: "404 Not Found".to_string(),
            color: ORANGE,
        };

        assert_eq!(enemy_bullet.x, 300.0);
        assert_eq!(enemy_bullet.attack_name, "404 Not Found");
        assert_eq!(enemy_bullet.damage, 20);
    }

    #[test]
    fn test_bullet_types() {
        let http_methods = ["GET", "POST", "PUT", "DELETE"];
        
        for method in &http_methods {
            let bullet = Bullet {
                x: 0.0,
                y: 0.0,
                speed: 500.0,
                bullet_type: method.to_string(),
                damage: 10,
                color: WHITE,
            };
            
            assert_eq!(bullet.bullet_type, *method);
            assert!(bullet.speed > 0.0);
        }
    }

    #[test]
    fn test_http_error_attacks() {
        let error_codes = [
            "400 Bad Request",
            "401 Unauthorized", 
            "403 Forbidden",
            "429 Too Many Requests",
            "500 Internal Server Error"
        ];
        
        for error in &error_codes {
            let enemy_bullet = EnemyBullet {
                x: 0.0,
                y: 0.0,
                speed: 250.0,
                damage: 20,
                attack_name: error.to_string(),
                color: ORANGE,
            };
            
            assert_eq!(enemy_bullet.attack_name, *error);
            assert!(enemy_bullet.attack_name.len() > 10);
            assert!(enemy_bullet.damage > 0);
        }
    }
}
