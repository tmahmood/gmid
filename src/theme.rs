/// RGB color used by the SDL renderer.
pub type Rgb = [u8; 3];

pub const ARENA_WIDTH: f32 = 1280.0;
pub const ARENA_HEIGHT: f32 = 720.0;

// Player tuning.
pub const PLAYER_RADIUS: f32 = 16.0;
pub const PLAYER_SPEED: f32 = 285.0;
pub const PLAYER_MAX_HEALTH: f32 = 100.0;
pub const PLAYER_FIRE_COOLDOWN: f32 = 0.11;
pub const PLAYER_INVULNERABILITY_SECONDS: f32 = 0.9;
pub const PLAYER_HIT_FLASH_SECONDS: f32 = 0.22;

// Bullet tuning.
pub const BULLET_RADIUS: f32 = 4.0;
pub const BULLET_SPEED: f32 = 980.0;
pub const BULLET_LIFETIME_SECONDS: f32 = 1.5;
pub const BULLET_DAMAGE: f32 = 34.0;

// Launcher tuning.
pub const LAUNCHER_RADIUS: f32 = 26.0;
pub const LAUNCHER_MAX_HEALTH: f32 = 100.0;
pub const LAUNCHER_FIRE_INTERVAL_SECONDS: f32 = 3.0;
pub const LAUNCHER_RESPAWN_SECONDS: f32 = 7.0;

// Missile tuning.
pub const MISSILE_RADIUS: f32 = 7.0;
pub const MISSILE_INITIAL_SPEED: f32 = 165.0;
pub const MISSILE_MAX_SPEED: f32 = 270.0;
pub const MISSILE_ACCELERATION: f32 = 62.0;
pub const MISSILE_TURN_RATE: f32 = 2.35;
pub const MISSILE_LIFETIME_SECONDS: f32 = 8.0;
pub const MISSILE_DAMAGE: f32 = 16.0;

// Scoring.
pub const MISSILE_SCORE: u32 = 25;
pub const LAUNCHER_SCORE: u32 = 100;

// Particle tuning.
pub const PARTICLE_MIN_LIFETIME_SECONDS: f32 = 0.25;
pub const PARTICLE_MAX_LIFETIME_SECONDS: f32 = 0.75;

// World colors.
pub const COLOR_BACKGROUND: Rgb = [9, 13, 22];
pub const COLOR_GRID: Rgb = [24, 34, 52];
pub const COLOR_ARENA_BORDER: Rgb = [52, 82, 120];
pub const COLOR_PLAYER: Rgb = [72, 225, 255];
pub const COLOR_PLAYER_CORE: Rgb = [226, 252, 255];
pub const COLOR_BULLET: Rgb = [255, 224, 120];
pub const COLOR_BULLET_CORE: Rgb = [255, 252, 220];
pub const COLOR_LAUNCHER: Rgb = [255, 124, 72];
pub const COLOR_LAUNCHER_DARK: Rgb = [104, 42, 32];
pub const COLOR_LAUNCHER_CORE: Rgb = [255, 206, 150];
pub const COLOR_MISSILE: Rgb = [255, 76, 112];
pub const COLOR_MISSILE_CORE: Rgb = [255, 226, 232];
pub const COLOR_HEALTH_FULL: Rgb = [110, 230, 130];
pub const COLOR_HEALTH_MEDIUM: Rgb = [255, 200, 90];
pub const COLOR_HEALTH_LOW: Rgb = [255, 90, 90];
