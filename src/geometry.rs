use std::ops::{Add, AddAssign, Mul, Sub};

/// A two-dimensional vector used for positions, velocities and directions.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub horizontal: f32,
    pub vertical: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self {
        horizontal: 0.0,
        vertical: 0.0,
    };

    pub const fn new(horizontal: f32, vertical: f32) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    pub fn length_squared(self) -> f32 {
        self.horizontal * self.horizontal + self.vertical * self.vertical
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns a unit vector in the same direction, or `fallback` for a zero vector.
    pub fn normalized_or(self, fallback: Self) -> Self {
        let current_length = self.length();
        if current_length > f32::EPSILON {
            Self::new(
                self.horizontal / current_length,
                self.vertical / current_length,
            )
        } else {
            fallback
        }
    }

    pub fn distance_to(self, other: Self) -> f32 {
        (other - self).length()
    }

    pub fn angle(self) -> f32 {
        self.vertical.atan2(self.horizontal)
    }

    pub fn from_angle(angle: f32, length: f32) -> Self {
        Self::new(angle.cos() * length, angle.sin() * length)
    }

    pub fn clamped(self, minimum: Self, maximum: Self) -> Self {
        Self::new(
            self.horizontal
                .clamp(minimum.horizontal, maximum.horizontal),
            self.vertical.clamp(minimum.vertical, maximum.vertical),
        )
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.horizontal + other.horizontal,
            self.vertical + other.vertical,
        )
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.horizontal - other.horizontal,
            self.vertical - other.vertical,
        )
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self::new(self.horizontal * scalar, self.vertical * scalar)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.horizontal += other.horizontal;
        self.vertical += other.vertical;
    }
}

/// Returns true when two circular bodies intersect.
pub fn circles_overlap(
    first_center: Vec2,
    first_radius: f32,
    second_center: Vec2,
    second_radius: f32,
) -> bool {
    let combined_radius = first_radius + second_radius;
    first_center.distance_to(second_center) <= combined_radius
}

/// Returns the signed smallest rotation from `from_angle` to `to_angle`,
/// in the range `-PI..PI`.
pub fn shortest_angle_difference(from_angle: f32, to_angle: f32) -> f32 {
    let raw_difference = to_angle - from_angle;
    (raw_difference + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
}
