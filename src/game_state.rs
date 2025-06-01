use crate::{Bullet, Enemy, EnemyBullet, Player};
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

pub struct GameState {
    pub player: Player,
    pub enemy: Option<Enemy>,
    pub bullets: Vec<Bullet>,
    pub enemy_bullets: Vec<EnemyBullet>,
    pub score: i32,
    pub enemies_defeated: i32,
    pub game_over: bool,
    pub victory_screen: bool,
    pub enemy_weakness_revealed: bool,
    pub enemy_attack_timer: f32,
    pub ui_message: String,
    pub ui_message_timer: f32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player: Player::new(),
            enemy: Some(Enemy::new()),
            bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            score: 0,
            enemies_defeated: 0,
            game_over: false,
            victory_screen: false,
            enemy_weakness_revealed: false,
            enemy_attack_timer: 0.0,
            ui_message: "Use WASD to move, 1/2/3/4 to shoot different HTTP methods!".to_string(),
            ui_message_timer: 3.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.game_over {
            return;
        }

        if self.victory_screen {
            if is_key_pressed(KeyCode::Space) {
                self.enemy = Some(Enemy::new());
                self.victory_screen = false;
                self.enemy_weakness_revealed = false;
                self.ui_message = "New target acquired!".to_string();
                self.ui_message_timer = 2.0;
            }
            return;
        }

        if self.ui_message_timer > 0.0 {
            self.ui_message_timer -= dt;
        }

        // Player movement
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.player.x = (self.player.x - self.player.speed * dt).max(self.player.size);
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.player.x = (self.player.x + self.player.speed * dt).min(800.0 - self.player.size);
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.player.y = (self.player.y - self.player.speed * dt).max(250.0);
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.player.y = (self.player.y + self.player.speed * dt).min(550.0);
        }

        // Shooting
        if self.enemy.is_some() {
            if is_key_pressed(KeyCode::Key1) && self.player.get_ammo > 0 {
                self.fire_bullet("GET");
            }
            if is_key_pressed(KeyCode::Key2) && self.player.post_ammo > 0 {
                self.fire_bullet("POST");
            }
            if is_key_pressed(KeyCode::Key3) && self.player.put_ammo > 0 {
                self.fire_bullet("PUT");
            }
            if is_key_pressed(KeyCode::Key4) && self.player.delete_ammo > 0 {
                self.fire_bullet("DELETE");
            }
        }

        // Update enemy
        if let Some(ref mut enemy) = self.enemy {
            enemy.update(dt);

            self.enemy_attack_timer += dt;
            if self.enemy_attack_timer > 1.5 {
                self.enemy_attack();
                self.enemy_attack_timer = 0.0;
            }
        }

        // Update bullets
        self.bullets.retain_mut(|bullet| {
            bullet.y -= bullet.speed * dt;
            bullet.y > 0.0
        });

        self.enemy_bullets.retain_mut(|bullet| {
            bullet.y += bullet.speed * dt;
            bullet.y < 600.0
        });

        self.check_collisions();

        // Check if enemy defeated
        if let Some(ref enemy) = self.enemy {
            if enemy.hp <= 0 {
                self.score += 100;
                self.enemies_defeated += 1;
                self.enemy = None;
                self.victory_screen = true;
                self.enemy_bullets.clear();
            }
        }

        if self.player.hp <= 0 {
            self.game_over = true;
        }
    }

    pub fn fire_bullet(&mut self, bullet_type: &str) {
        let (damage, color) = match bullet_type {
            "GET" => {
                self.player.get_ammo -= 1;
                (0, BLUE)
            }
            "POST" => {
                self.player.post_ammo -= 1;
                let damage = if let Some(ref enemy) = self.enemy {
                    if enemy.weakness == "POST" {
                        40
                    } else {
                        20
                    }
                } else {
                    20
                };
                (damage, GREEN)
            }
            "PUT" => {
                self.player.put_ammo -= 1;
                let damage = if let Some(ref enemy) = self.enemy {
                    if enemy.weakness == "PUT" {
                        45
                    } else {
                        25
                    }
                } else {
                    25
                };
                (damage, YELLOW)
            }
            "DELETE" => {
                self.player.delete_ammo -= 1;
                (100, RED)
            }
            _ => (10, WHITE),
        };

        self.bullets.push(Bullet {
            x: self.player.x,
            y: self.player.y - self.player.size,
            speed: 500.0,
            bullet_type: bullet_type.to_string(),
            damage,
            color,
        });
    }

    pub fn enemy_attack(&mut self) {
        if let Some(ref enemy) = self.enemy {
            let mut rng = thread_rng();
            let attacks = vec![
                ("400 Bad Request", 15),
                ("401 Unauthorized", 20),
                ("403 Forbidden", 25),
                ("429 Too Many Requests", 30),
                ("500 Internal Server Error", 35),
            ];

            let (attack_name, damage) = &attacks[rng.gen_range(0..attacks.len())];

            self.enemy_bullets.push(EnemyBullet {
                x: enemy.x,
                y: enemy.y + enemy.size,
                speed: 250.0,
                damage: *damage,
                attack_name: attack_name.to_string(),
            });
        }
    }

    pub fn check_collisions(&mut self) {
        if let Some(ref mut enemy) = self.enemy {
            self.bullets.retain(|bullet| {
                let hit = (bullet.x - enemy.x).abs() < enemy.size
                    && (bullet.y - enemy.y).abs() < enemy.size;

                if hit {
                    if bullet.bullet_type == "GET" {
                        if !self.enemy_weakness_revealed {
                            self.enemy_weakness_revealed = true;
                            self.ui_message = format!("Target weakness: {}", enemy.weakness);
                            self.ui_message_timer = 3.0;
                        }
                    } else {
                        enemy.hp -= bullet.damage;
                        self.score += bullet.damage;

                        if enemy.weakness == bullet.bullet_type {
                            self.ui_message = "CRITICAL HIT!".to_string();
                            self.ui_message_timer = 1.0;
                        }
                    }
                    false
                } else {
                    true
                }
            });
        }

        self.enemy_bullets.retain(|bullet| {
            let hit = (bullet.x - self.player.x).abs() < self.player.size
                && (bullet.y - self.player.y).abs() < self.player.size;

            if hit {
                self.player.hp -= bullet.damage;
                self.ui_message = format!("Hit by {}!", bullet.attack_name);
                self.ui_message_timer = 1.5;
                false
            } else {
                true
            }
        });
    }

    pub fn draw(&self) {
        draw_rectangle(0.0, 0.0, 800.0, 600.0, Color::new(0.05, 0.05, 0.15, 1.0));

        if self.game_over {
            self.draw_game_over();
            return;
        }

        if self.victory_screen {
            self.draw_victory_screen();
            return;
        }

        // Draw player
        draw_circle(
            self.player.x,
            self.player.y,
            self.player.size + 2.0,
            Color::new(0.0, 0.5, 1.0, 0.5),
        );
        draw_triangle(
            Vec2::new(self.player.x, self.player.y - self.player.size),
            Vec2::new(
                self.player.x - self.player.size * 0.7,
                self.player.y + self.player.size * 0.5,
            ),
            Vec2::new(
                self.player.x + self.player.size * 0.7,
                self.player.y + self.player.size * 0.5,
            ),
            BLUE,
        );

        // Draw enemy
        if let Some(ref enemy) = self.enemy {
            draw_circle(
                enemy.x,
                enemy.y,
                enemy.size + 3.0,
                Color::new(1.0, 0.0, 0.0, 0.3),
            );

            draw_circle(enemy.x, enemy.y, enemy.size, RED);
            draw_circle(
                enemy.x,
                enemy.y,
                enemy.size * 0.7,
                Color::new(0.8, 0.0, 0.0, 1.0),
            );

            let health_width = 120.0;
            let health_fraction = enemy.hp as f32 / enemy.max_hp as f32;

            draw_rectangle(
                enemy.x - health_width / 2.0,
                enemy.y - enemy.size - 25.0,
                health_width,
                6.0,
                DARKGRAY,
            );

            let health_color = if health_fraction > 0.7 {
                GREEN
            } else if health_fraction > 0.3 {
                YELLOW
            } else {
                RED
            };

            draw_rectangle(
                enemy.x - health_width / 2.0,
                enemy.y - enemy.size - 25.0,
                health_width * health_fraction,
                6.0,
                health_color,
            );
        }

        // Draw bullets
        for bullet in &self.bullets {
            for i in 1..5 {
                let alpha = 0.8 - (i as f32 * 0.2);
                let trail_color = Color::new(bullet.color.r, bullet.color.g, bullet.color.b, alpha);
                draw_circle(
                    bullet.x,
                    bullet.y + (i as f32 * 8.0),
                    4.0 - i as f32,
                    trail_color,
                );
            }
            draw_circle(bullet.x, bullet.y, 6.0, bullet.color);
        }

        for bullet in &self.enemy_bullets {
            draw_circle(bullet.x, bullet.y, 8.0, ORANGE);
            draw_circle(bullet.x, bullet.y, 4.0, YELLOW);
        }

        self.draw_modern_ui();
    }

    fn draw_modern_ui(&self) {
        // Top HUD
        draw_rectangle(0.0, 0.0, 800.0, 100.0, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_rectangle_lines(0.0, 0.0, 800.0, 100.0, 2.0, Color::new(0.0, 0.8, 1.0, 0.5));

        draw_text(
            "API SHOOTER",
            20.0,
            35.0,
            32.0,
            Color::new(0.0, 0.8, 1.0, 1.0),
        );
        draw_text(&format!("SCORE: {}", self.score), 20.0, 65.0, 24.0, YELLOW);
        draw_text(
            &format!("TARGETS: {}", self.enemies_defeated),
            200.0,
            65.0,
            24.0,
            GREEN,
        );

        // Enemy info panel
        if let Some(ref enemy) = self.enemy {
            let panel_x = 450.0;
            draw_rectangle(panel_x, 10.0, 340.0, 80.0, Color::new(0.8, 0.0, 0.0, 0.3));
            draw_rectangle_lines(panel_x, 10.0, 340.0, 80.0, 2.0, RED);

            draw_text("TARGET", panel_x + 10.0, 35.0, 20.0, WHITE);
            draw_text(&enemy.name, panel_x + 10.0, 55.0, 18.0, ORANGE);

            if self.enemy_weakness_revealed {
                draw_text(
                    &format!("WEAKNESS: {}", enemy.weakness),
                    panel_x + 10.0,
                    75.0,
                    16.0,
                    YELLOW,
                );
            } else {
                draw_text("Press 1 to scan", panel_x + 10.0, 75.0, 16.0, GRAY);
            }
        }

        // Bottom HUD
        draw_rectangle(0.0, 520.0, 800.0, 80.0, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_rectangle_lines(0.0, 520.0, 800.0, 80.0, 2.0, Color::new(0.0, 0.8, 1.0, 0.5));

        // HP bar
        let hp_bar_x = 20.0;
        let hp_bar_y = 540.0;
        let hp_bar_width = 300.0;
        let hp_bar_height = 20.0;

        draw_rectangle(
            hp_bar_x - 2.0,
            hp_bar_y - 2.0,
            hp_bar_width + 4.0,
            hp_bar_height + 4.0,
            Color::new(0.0, 0.5, 1.0, 0.3),
        );
        draw_rectangle(hp_bar_x, hp_bar_y, hp_bar_width, hp_bar_height, DARKGRAY);

        let hp_fraction = self.player.hp as f32 / self.player.max_hp as f32;
        let hp_color = if hp_fraction > 0.6 {
            GREEN
        } else if hp_fraction > 0.3 {
            YELLOW
        } else {
            RED
        };

        draw_rectangle(
            hp_bar_x,
            hp_bar_y,
            hp_bar_width * hp_fraction,
            hp_bar_height,
            hp_color,
        );

        draw_text("HEALTH", hp_bar_x, hp_bar_y - 5.0, 16.0, WHITE);
        draw_text(
            &format!("{}/{}", self.player.hp, self.player.max_hp),
            hp_bar_x + 10.0,
            hp_bar_y + 15.0,
            14.0,
            WHITE,
        );

        // Weapon display
        let weapon_x = 450.0;
        let weapon_y = 540.0;

        draw_rectangle(
            weapon_x,
            weapon_y,
            330.0,
            40.0,
            Color::new(0.2, 0.2, 0.2, 0.8),
        );
        draw_rectangle_lines(weapon_x, weapon_y, 330.0, 40.0, 1.0, GRAY);

        let weapons = [
            ("1:GET", self.player.get_ammo, BLUE),
            ("2:POST", self.player.post_ammo, GREEN),
            ("3:PUT", self.player.put_ammo, YELLOW),
            ("4:DEL", self.player.delete_ammo, RED),
        ];

        for (i, (name, ammo, color)) in weapons.iter().enumerate() {
            let x = weapon_x + 10.0 + (i as f32 * 75.0);
            draw_text(name, x, weapon_y + 15.0, 12.0, *color);
            draw_text(&ammo.to_string(), x + 5.0, weapon_y + 30.0, 14.0, WHITE);
        }

        draw_text("WASD: Move | 1-4: Weapons", 20.0, 590.0, 14.0, LIGHTGRAY);

        // UI Message
        if self.ui_message_timer > 0.0 {
            let msg_y = 300.0;
            let text_width = measure_text(&self.ui_message, None, 24, 1.0).width;
            draw_rectangle(
                400.0 - text_width / 2.0 - 20.0,
                msg_y - 30.0,
                text_width + 40.0,
                50.0,
                Color::new(0.0, 0.0, 0.0, 0.8),
            );
            draw_rectangle_lines(
                400.0 - text_width / 2.0 - 20.0,
                msg_y - 30.0,
                text_width + 40.0,
                50.0,
                2.0,
                YELLOW,
            );
            draw_text(
                &self.ui_message,
                400.0 - text_width / 2.0,
                msg_y,
                24.0,
                YELLOW,
            );
        }
    }

    fn draw_victory_screen(&self) {
        draw_rectangle(0.0, 0.0, 800.0, 600.0, Color::new(0.0, 0.0, 0.0, 0.7));

        let panel_width = 500.0;
        let panel_height = 300.0;
        let panel_x = (800.0 - panel_width) / 2.0;
        let panel_y = (600.0 - panel_height) / 2.0;

        draw_rectangle(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            Color::new(0.0, 0.2, 0.0, 0.9),
        );
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 3.0, GREEN);

        draw_text(
            "TARGET ELIMINATED!",
            panel_x + 80.0,
            panel_y + 60.0,
            36.0,
            GREEN,
        );
        draw_text(
            &format!("Score: +100 (Total: {})", self.score),
            panel_x + 120.0,
            panel_y + 120.0,
            24.0,
            YELLOW,
        );
        draw_text(
            &format!("Targets Defeated: {}", self.enemies_defeated),
            panel_x + 100.0,
            panel_y + 160.0,
            20.0,
            WHITE,
        );

        draw_text(
            "Press SPACE for next target",
            panel_x + 100.0,
            panel_y + 220.0,
            20.0,
            YELLOW,
        );
        draw_text(
            "Press ESC to quit",
            panel_x + 150.0,
            panel_y + 250.0,
            18.0,
            LIGHTGRAY,
        );
    }

    fn draw_game_over(&self) {
        draw_rectangle(0.0, 0.0, 800.0, 600.0, Color::new(0.5, 0.0, 0.0, 0.8));

        let panel_width = 500.0;
        let panel_height = 300.0;
        let panel_x = (800.0 - panel_width) / 2.0;
        let panel_y = (600.0 - panel_height) / 2.0;

        draw_rectangle(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            Color::new(0.2, 0.0, 0.0, 0.9),
        );
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 3.0, RED);

        draw_text("MISSION FAILED", panel_x + 110.0, panel_y + 80.0, 36.0, RED);
        draw_text(
            &format!("Final Score: {}", self.score),
            panel_x + 150.0,
            panel_y + 140.0,
            24.0,
            WHITE,
        );
        draw_text(
            &format!("Targets Eliminated: {}", self.enemies_defeated),
            panel_x + 120.0,
            panel_y + 180.0,
            20.0,
            WHITE,
        );

        draw_text(
            "Press R to restart",
            panel_x + 160.0,
            panel_y + 230.0,
            20.0,
            YELLOW,
        );
        draw_text(
            "Press ESC to quit",
            panel_x + 160.0,
            panel_y + 260.0,
            18.0,
            LIGHTGRAY,
        );
    }
}
