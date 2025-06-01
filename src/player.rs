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
