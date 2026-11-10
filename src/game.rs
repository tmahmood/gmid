use std::f32::consts::TAU;

use crate::entities::{Bullet, Launcher, Missile, Particle, Player};
use crate::geometry::{circles_overlap, shortest_angle_difference, Vec2};
use crate::theme::*;

/// Input collected from SDL for one simulation step.
#[derive(Clone, Copy)]
pub struct FrameInput {
    pub movement: Vec2,
    pub firing: bool,
    pub aim_target: Vec2,
}

struct LaunchEvent {
    position: Vec2,
    velocity: Vec2,
}

pub struct GameState {
    pub player: Player,
    pub bullets: Vec<Bullet>,
    pub launchers: Vec<Launcher>,
    pub missiles: Vec<Missile>,
    pub particles: Vec<Particle>,
    pub score: u32,
    pub launchers_destroyed: u32,
    pub survived_seconds: f32,
    pub game_over: bool,
    effect_rotation: f32,
}

impl GameState {
    pub fn new() -> Self {
        let player = Player::new(Vec2::new(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5));
        let launchers = vec![
            Launcher::new(Vec2::new(180.0, 235.0), 1.3),
            Launcher::new(Vec2::new(ARENA_WIDTH - 180.0, 220.0), 2.2),
            Launcher::new(Vec2::new(195.0, ARENA_HEIGHT - 155.0), 2.9),
            Launcher::new(Vec2::new(ARENA_WIDTH - 200.0, ARENA_HEIGHT - 145.0), 1.8),
        ];

        Self {
            player,
            bullets: Vec::new(),
            launchers,
            missiles: Vec::new(),
            particles: Vec::new(),
            score: 0,
            launchers_destroyed: 0,
            survived_seconds: 0.0,
            game_over: false,
            effect_rotation: 0.0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn active_launcher_count(&self) -> usize {
        self.launchers
            .iter()
            .filter(|launcher| launcher.alive)
            .count()
    }

    pub fn update(&mut self, delta_seconds: f32, input: FrameInput) {
        let step = delta_seconds.clamp(0.0, 0.05);

        if self.game_over {
            self.update_particles(step);
            return;
        }

        self.survived_seconds += step;
        self.player.invulnerability_remaining =
            (self.player.invulnerability_remaining - step).max(0.0);
        self.player.hit_flash_remaining = (self.player.hit_flash_remaining - step).max(0.0);

        let aim_delta = input.aim_target - self.player.position;
        self.player.aim_direction = aim_delta.normalized_or(self.player.aim_direction);

        self.update_player_movement(step, input.movement);
        self.update_player_weapon(step, input.firing);
        self.update_launchers(step);
        self.update_missiles(step);
        self.update_bullets(step);
        self.update_particles(step);
        self.cleanup_entities();
    }

    fn update_player_movement(&mut self, delta_seconds: f32, movement_input: Vec2) {
        let movement_direction = movement_input.normalized_or(Vec2::ZERO);
        self.player.position += movement_direction * (PLAYER_SPEED * delta_seconds);

        let border_padding = self.player.radius + 10.0;
        let minimum_position = Vec2::new(border_padding, border_padding);
        let maximum_position =
            Vec2::new(ARENA_WIDTH - border_padding, ARENA_HEIGHT - border_padding);
        self.player.position = self
            .player
            .position
            .clamped(minimum_position, maximum_position);
    }

    fn update_player_weapon(&mut self, delta_seconds: f32, firing: bool) {
        self.player.fire_cooldown_remaining =
            (self.player.fire_cooldown_remaining - delta_seconds).max(0.0);

        if firing && self.player.fire_cooldown_remaining <= 0.0 {
            self.fire_player_bullet();
            self.player.fire_cooldown_remaining = PLAYER_FIRE_COOLDOWN;
        }
    }

    fn fire_player_bullet(&mut self) {
        let direction = self.player.aim_direction.normalized_or(Vec2::new(1.0, 0.0));
        let muzzle_position = self.player.position + direction * (self.player.radius + 12.0);

        self.bullets
            .push(Bullet::new(muzzle_position, direction * BULLET_SPEED));
        self.spawn_muzzle_particles(muzzle_position, direction);
    }

    fn update_launchers(&mut self, delta_seconds: f32) {
        let mut launch_events: Vec<LaunchEvent> = Vec::new();

        for launcher in &mut self.launchers {
            if !launcher.alive {
                launcher.respawn_remaining -= delta_seconds;
                if launcher.respawn_remaining <= 0.0 {
                    launcher.alive = true;
                    launcher.health = launcher.max_health;
                    launcher.fire_cooldown_remaining = 1.5;
                }
                continue;
            }

            let towards_player = self.player.position - launcher.position;
            if towards_player.length_squared() > 1.0 {
                launcher.aim_angle = towards_player.angle();
            }

            launcher.fire_cooldown_remaining -= delta_seconds;
            if launcher.fire_cooldown_remaining <= 0.0 {
                let direction = towards_player.normalized_or(Vec2::new(1.0, 0.0));
                let spawn_position = launcher.position + direction * (launcher.radius + 10.0);

                launch_events.push(LaunchEvent {
                    position: spawn_position,
                    velocity: direction * MISSILE_INITIAL_SPEED,
                });
                launcher.fire_cooldown_remaining = LAUNCHER_FIRE_INTERVAL_SECONDS;
            }
        }

        for launch_event in launch_events {
            self.missiles
                .push(Missile::new(launch_event.position, launch_event.velocity));
            self.spawn_launch_particles(launch_event.position);
        }
    }

    fn update_missiles(&mut self, delta_seconds: f32) {
        for missile in &mut self.missiles {
            missile.lifetime_remaining -= delta_seconds;

            let towards_player = self.player.position - missile.position;
            let current_speed = missile.velocity.length();
            let current_angle = missile.velocity.angle();
            let desired_angle = if towards_player.length_squared() > 1.0 {
                towards_player.angle()
            } else {
                current_angle
            };

            let maximum_turn = MISSILE_TURN_RATE * delta_seconds;
            let turn_amount = shortest_angle_difference(current_angle, desired_angle)
                .clamp(-maximum_turn, maximum_turn);
            let new_speed =
                (current_speed + MISSILE_ACCELERATION * delta_seconds).min(MISSILE_MAX_SPEED);

            missile.velocity = Vec2::from_angle(current_angle + turn_amount, new_speed);
            missile.position += missile.velocity * delta_seconds;

            missile.trail_timer -= delta_seconds;
            if missile.trail_timer <= 0.0 {
                missile.trail.push(missile.position);
                missile.trail_timer = 0.03;
                if missile.trail.len() > 16 {
                    missile.trail.remove(0);
                }
            }
        }

        let mut missile_index = 0;
        while missile_index < self.missiles.len() {
            let missile = &self.missiles[missile_index];
            let missile_position = missile.position;
            let hit_player = circles_overlap(
                missile.position,
                missile.radius,
                self.player.position,
                self.player.radius,
            );

            if hit_player {
                self.missiles.swap_remove(missile_index);
                self.spawn_explosion(missile_position, COLOR_MISSILE, 20, 260.0);

                if self.player.invulnerability_remaining <= 0.0 {
                    self.player.invulnerability_remaining = PLAYER_INVULNERABILITY_SECONDS;
                    self.player.hit_flash_remaining = PLAYER_HIT_FLASH_SECONDS;
                    self.damage_player(MISSILE_DAMAGE);
                }
                continue;
            }

            missile_index += 1;
        }
    }

    fn update_bullets(&mut self, delta_seconds: f32) {
        for bullet in &mut self.bullets {
            bullet.position += bullet.velocity * delta_seconds;
            bullet.lifetime_remaining -= delta_seconds;
        }

        let mut bullet_index = 0;
        while bullet_index < self.bullets.len() {
            let bullet_position = self.bullets[bullet_index].position;
            let bullet_radius = self.bullets[bullet_index].radius;
            let mut bullet_consumed = false;

            let mut missile_index = 0;
            while missile_index < self.missiles.len() {
                let missile = &self.missiles[missile_index];
                if circles_overlap(
                    bullet_position,
                    bullet_radius,
                    missile.position,
                    missile.radius,
                ) {
                    let missile_position = missile.position;
                    self.missiles.swap_remove(missile_index);
                    self.score += MISSILE_SCORE;
                    self.spawn_explosion(missile_position, COLOR_MISSILE, 14, 220.0);
                    bullet_consumed = true;
                    break;
                }
                missile_index += 1;
            }

            if !bullet_consumed {
                let mut launcher_index = 0;
                while launcher_index < self.launchers.len() {
                    let (launcher_position, launcher_radius, launcher_alive) = {
                        let launcher = &self.launchers[launcher_index];
                        (launcher.position, launcher.radius, launcher.alive)
                    };

                    if launcher_alive
                        && circles_overlap(
                            bullet_position,
                            bullet_radius,
                            launcher_position,
                            launcher_radius,
                        )
                    {
                        self.launchers[launcher_index].health -= BULLET_DAMAGE;
                        let launcher_destroyed = self.launchers[launcher_index].health <= 0.0;

                        if launcher_destroyed {
                            self.launchers[launcher_index].health = 0.0;
                            self.launchers[launcher_index].alive = false;
                            self.launchers[launcher_index].respawn_remaining =
                                LAUNCHER_RESPAWN_SECONDS;
                            self.score += LAUNCHER_SCORE;
                            self.launchers_destroyed += 1;
                            self.spawn_explosion(launcher_position, COLOR_LAUNCHER, 28, 300.0);
                        } else {
                            self.spawn_hit_particles(launcher_position, COLOR_LAUNCHER);
                        }

                        bullet_consumed = true;
                        break;
                    }
                    launcher_index += 1;
                }
            }

            if bullet_consumed {
                self.bullets.swap_remove(bullet_index);
            } else {
                bullet_index += 1;
            }
        }
    }

    fn update_particles(&mut self, delta_seconds: f32) {
        for particle in &mut self.particles {
            particle.lifetime_remaining -= delta_seconds;
            particle.position += particle.velocity * delta_seconds;

            let drag = (1.0 - 3.2 * delta_seconds).clamp(0.0, 1.0);
            particle.velocity = particle.velocity * drag;
        }

        self.particles
            .retain(|particle| particle.lifetime_remaining > 0.0);
    }

    fn cleanup_entities(&mut self) {
        let cleanup_margin = 64.0;

        self.bullets.retain(|bullet| {
            bullet.lifetime_remaining > 0.0
                && position_is_inside_arena(bullet.position, cleanup_margin)
        });
        self.missiles.retain(|missile| {
            missile.lifetime_remaining > 0.0
                && position_is_inside_arena(missile.position, cleanup_margin)
        });
    }

    fn damage_player(&mut self, damage_amount: f32) {
        if self.game_over {
            return;
        }

        self.player.health = (self.player.health - damage_amount).max(0.0);

        if self.player.health <= 0.0 {
            self.game_over = true;
            self.spawn_explosion(self.player.position, COLOR_PLAYER, 42, 360.0);
            self.spawn_explosion(self.player.position, COLOR_MISSILE, 24, 230.0);
        }
    }

    fn spawn_muzzle_particles(&mut self, muzzle_position: Vec2, direction: Vec2) {
        let base_angle = direction.angle();

        for index in 0..6 {
            let angle_offset = (index as f32 - 2.5) * 0.11;
            let particle_angle = base_angle + angle_offset;
            let particle_speed = 110.0 + index as f32 * 18.0;
            let particle_lifetime = 0.16 + index as f32 * 0.012;

            self.particles.push(Particle::new(
                muzzle_position,
                Vec2::from_angle(particle_angle, particle_speed),
                2.2,
                particle_lifetime,
                COLOR_BULLET,
            ));
        }
    }

    fn spawn_launch_particles(&mut self, launch_position: Vec2) {
        for index in 0..8 {
            let particle_angle = TAU * index as f32 / 8.0 + self.effect_rotation;
            self.particles.push(Particle::new(
                launch_position,
                Vec2::from_angle(particle_angle, 80.0 + index as f32 * 9.0),
                2.4,
                0.22,
                COLOR_LAUNCHER,
            ));
        }
        self.effect_rotation = (self.effect_rotation + 0.55) % TAU;
    }

    fn spawn_hit_particles(&mut self, hit_position: Vec2, color: Rgb) {
        for index in 0..5 {
            let particle_angle = TAU * index as f32 / 5.0 + self.effect_rotation;
            self.particles.push(Particle::new(
                hit_position,
                Vec2::from_angle(particle_angle, 120.0 + index as f32 * 16.0),
                2.0,
                0.24,
                color,
            ));
        }
        self.effect_rotation = (self.effect_rotation + 0.7) % TAU;
    }

    fn spawn_explosion(
        &mut self,
        explosion_position: Vec2,
        color: Rgb,
        particle_count: usize,
        base_speed: f32,
    ) {
        let particle_count = particle_count.max(1);

        for index in 0..particle_count {
            let particle_angle = self.effect_rotation + TAU * index as f32 / particle_count as f32;
            let speed_jitter = 0.45 + ((index * 37) % 100) as f32 / 100.0;
            let lifetime_ratio = ((index * 17) % 100) as f32 / 100.0;
            let particle_lifetime = PARTICLE_MIN_LIFETIME_SECONDS
                + (PARTICLE_MAX_LIFETIME_SECONDS - PARTICLE_MIN_LIFETIME_SECONDS) * lifetime_ratio;
            let particle_radius = 2.0 + ((index * 29) % 100) as f32 / 100.0 * 2.5;
            let particle_color = if index % 4 == 0 {
                COLOR_BULLET_CORE
            } else {
                color
            };

            self.particles.push(Particle::new(
                explosion_position,
                Vec2::from_angle(particle_angle, base_speed * speed_jitter),
                particle_radius,
                particle_lifetime,
                particle_color,
            ));
        }

        self.effect_rotation = (self.effect_rotation + 0.83) % TAU;
    }
}

fn position_is_inside_arena(position: Vec2, margin: f32) -> bool {
    position.horizontal >= -margin
        && position.horizontal <= ARENA_WIDTH + margin
        && position.vertical >= -margin
        && position.vertical <= ARENA_HEIGHT + margin
}
