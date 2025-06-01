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
            y: rng.gen_range(140.0..260.0),
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
                self.y = self.y.clamp(140.0, 260.0);
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
