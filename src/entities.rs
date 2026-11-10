use crate::geometry::Vec2;
use crate::theme::{
    Rgb, BULLET_LIFETIME_SECONDS, BULLET_RADIUS, LAUNCHER_MAX_HEALTH, LAUNCHER_RADIUS,
    MISSILE_LIFETIME_SECONDS, MISSILE_RADIUS, PLAYER_MAX_HEALTH, PLAYER_RADIUS,
};

pub struct Player {
    pub position: Vec2,
    pub radius: f32,
    pub health: f32,
    pub max_health: f32,
    pub aim_direction: Vec2,
    pub fire_cooldown_remaining: f32,
    pub invulnerability_remaining: f32,
    pub hit_flash_remaining: f32,
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            radius: PLAYER_RADIUS,
            health: PLAYER_MAX_HEALTH,
            max_health: PLAYER_MAX_HEALTH,
            aim_direction: Vec2::new(1.0, 0.0),
            fire_cooldown_remaining: 0.0,
            invulnerability_remaining: 0.0,
            hit_flash_remaining: 0.0,
        }
    }

    pub fn health_ratio(&self) -> f32 {
        (self.health / self.max_health).clamp(0.0, 1.0)
    }
}

pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub lifetime_remaining: f32,
}

impl Bullet {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self {
            position,
            velocity,
            radius: BULLET_RADIUS,
            lifetime_remaining: BULLET_LIFETIME_SECONDS,
        }
    }
}

pub struct Launcher {
    pub position: Vec2,
    pub radius: f32,
    pub health: f32,
    pub max_health: f32,
    pub fire_cooldown_remaining: f32,
    pub aim_angle: f32,
    pub respawn_remaining: f32,
    pub alive: bool,
}

impl Launcher {
    pub fn new(position: Vec2, initial_fire_delay: f32) -> Self {
        Self {
            position,
            radius: LAUNCHER_RADIUS,
            health: LAUNCHER_MAX_HEALTH,
            max_health: LAUNCHER_MAX_HEALTH,
            fire_cooldown_remaining: initial_fire_delay,
            aim_angle: 0.0,
            respawn_remaining: 0.0,
            alive: true,
        }
    }

    pub fn health_ratio(&self) -> f32 {
        (self.health / self.max_health).clamp(0.0, 1.0)
    }
}

pub struct Missile {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub lifetime_remaining: f32,
    pub trail: Vec<Vec2>,
    pub trail_timer: f32,
}

impl Missile {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self {
            position,
            velocity,
            radius: MISSILE_RADIUS,
            lifetime_remaining: MISSILE_LIFETIME_SECONDS,
            trail: Vec::new(),
            trail_timer: 0.0,
        }
    }
}

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub lifetime_remaining: f32,
    pub total_lifetime: f32,
    pub color: Rgb,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, radius: f32, lifetime: f32, color: Rgb) -> Self {
        Self {
            position,
            velocity,
            radius,
            lifetime_remaining: lifetime,
            total_lifetime: lifetime,
            color,
        }
    }

    pub fn life_ratio(&self) -> f32 {
        (self.lifetime_remaining / self.total_lifetime).clamp(0.0, 1.0)
    }
}
