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
}
