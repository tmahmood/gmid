use sdl3::pixels::{Color, FColor};
use sdl3::render::{BlendMode, Canvas, FPoint, FRect, Vertex, VertexIndices};
use sdl3::video::Window;

use crate::entities::{Bullet, Launcher, Missile, Particle, Player};
use crate::game::GameState;
use crate::geometry::Vec2;
use crate::theme::*;

pub fn render_world(canvas: &mut Canvas<Window>, game: &GameState) -> Result<(), sdl3::Error> {
    canvas.set_blend_mode(BlendMode::Blend);

    draw_background(canvas)?;
    draw_arena_border(canvas)?;

    for launcher in &game.launchers {
        draw_launcher(canvas, launcher)?;
    }
    for missile in &game.missiles {
        draw_missile(canvas, missile)?;
    }
    for bullet in &game.bullets {
        draw_bullet(canvas, bullet)?;
    }
    draw_player(canvas, &game.player)?;
    for particle in &game.particles {
        draw_particle(canvas, particle)?;
    }

    if game.game_over {
        draw_game_over_dim(canvas)?;
    }

    Ok(())
}

fn draw_background(canvas: &mut Canvas<Window>) -> Result<(), sdl3::Error> {
    let grid_color = solid_color(COLOR_GRID, 150);
    canvas.set_draw_color(grid_color);

    let grid_spacing = 64.0;
    let mut horizontal_position = 0.0;
    while horizontal_position <= ARENA_WIDTH {
        canvas.draw_line(
            FPoint::new(horizontal_position, 0.0),
            FPoint::new(horizontal_position, ARENA_HEIGHT),
        )?;
        horizontal_position += grid_spacing;
    }

    let mut vertical_position = 0.0;
    while vertical_position <= ARENA_HEIGHT {
        canvas.draw_line(
            FPoint::new(0.0, vertical_position),
            FPoint::new(ARENA_WIDTH, vertical_position),
        )?;
        vertical_position += grid_spacing;
    }

    draw_filled_circle(
        canvas,
        Vec2::new(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5),
        150.0,
        COLOR_ARENA_BORDER,
        16,
    )
}

fn draw_arena_border(canvas: &mut Canvas<Window>) -> Result<(), sdl3::Error> {
    canvas.set_draw_color(solid_color(COLOR_ARENA_BORDER, 210));
    canvas.draw_rect(FRect::new(
        10.0,
        10.0,
        ARENA_WIDTH - 20.0,
        ARENA_HEIGHT - 20.0,
    ))?;

    canvas.set_draw_color(solid_color(COLOR_ARENA_BORDER, 80));
    canvas.draw_rect(FRect::new(
        18.0,
        18.0,
        ARENA_WIDTH - 36.0,
        ARENA_HEIGHT - 36.0,
    ))?;

    Ok(())
}

fn draw_player(canvas: &mut Canvas<Window>, player: &Player) -> Result<(), sdl3::Error> {
    let blinking = player.invulnerability_remaining > 0.0
        && ((player.invulnerability_remaining * 22.0) as i32) % 2 == 0;
    let body_alpha = if blinking { 110 } else { 255 };
    let body_color = if player.hit_flash_remaining > 0.0 {
        [255, 246, 246]
    } else {
        COLOR_PLAYER
    };

    draw_glow(
        canvas,
        player.position,
        player.radius + 14.0,
        COLOR_PLAYER,
        38,
    )?;
    draw_filled_circle(
        canvas,
        player.position,
        player.radius,
        body_color,
        body_alpha,
    )?;
    draw_circle_outline(
        canvas,
        player.position,
        player.radius,
        COLOR_PLAYER_CORE,
        body_alpha,
    )?;
    draw_filled_circle(
        canvas,
        player.position,
        player.radius * 0.44,
        COLOR_PLAYER_CORE,
        body_alpha,
    )?;

    let aim_direction = player.aim_direction.normalized_or(Vec2::new(1.0, 0.0));
    let barrel_start = player.position + aim_direction * (player.radius * 0.35);
    let barrel_end = player.position + aim_direction * (player.radius + 18.0);

    draw_thick_line(
        canvas,
        barrel_start,
        barrel_end,
        7.0,
        COLOR_PLAYER_CORE,
        body_alpha,
    )?;
    draw_filled_circle(canvas, barrel_end, 3.4, COLOR_BULLET, body_alpha)?;

    Ok(())
}

fn draw_launcher(canvas: &mut Canvas<Window>, launcher: &Launcher) -> Result<(), sdl3::Error> {
    if !launcher.alive {
        draw_circle_outline(
            canvas,
            launcher.position,
            launcher.radius,
            COLOR_LAUNCHER_DARK,
            130,
        )?;
        draw_circle_outline(
            canvas,
            launcher.position,
            launcher.radius * 0.55,
            COLOR_LAUNCHER_DARK,
            80,
        )?;

        let respawn_ratio =
            (1.0 - launcher.respawn_remaining / LAUNCHER_RESPAWN_SECONDS).clamp(0.0, 1.0);
        draw_bar(
            canvas,
            launcher.position.horizontal - 27.0,
            launcher.position.vertical - launcher.radius - 16.0,
            54.0,
            5.0,
            respawn_ratio,
            COLOR_LAUNCHER,
        )?;

        return Ok(());
    }

    draw_glow(
        canvas,
        launcher.position,
        launcher.radius + 16.0,
        COLOR_LAUNCHER,
        34,
    )?;
    draw_filled_circle(
        canvas,
        launcher.position,
        launcher.radius,
        COLOR_LAUNCHER_DARK,
        255,
    )?;
    draw_circle_outline(
        canvas,
        launcher.position,
        launcher.radius,
        COLOR_LAUNCHER,
        255,
    )?;
    draw_filled_circle(
        canvas,
        launcher.position,
        launcher.radius * 0.62,
        [46, 22, 20],
        255,
    )?;

    let aim_direction = Vec2::from_angle(launcher.aim_angle, 1.0);
    let barrel_start = launcher.position + aim_direction * (launcher.radius * 0.45);
    let barrel_end = launcher.position + aim_direction * (launcher.radius + 16.0);
    draw_thick_line(canvas, barrel_start, barrel_end, 8.0, COLOR_LAUNCHER, 255)?;
    draw_filled_circle(canvas, launcher.position, 5.0, COLOR_LAUNCHER_CORE, 255)?;

    draw_bar(
        canvas,
        launcher.position.horizontal - 29.0,
        launcher.position.vertical - launcher.radius - 17.0,
        58.0,
        6.0,
        launcher.health_ratio(),
        health_color(launcher.health_ratio()),
    )?;

    Ok(())
}

fn draw_missile(canvas: &mut Canvas<Window>, missile: &Missile) -> Result<(), sdl3::Error> {
    let trail_length = missile.trail.len() as f32;

    for (index, trail_position) in missile.trail.iter().enumerate() {
        let age_ratio = (index as f32 + 1.0) / trail_length;
        let trail_alpha = (age_ratio * 105.0).clamp(0.0, 255.0) as u8;
        let trail_radius = 1.6 + age_ratio * 3.4;

        draw_filled_circle(
            canvas,
            *trail_position,
            trail_radius,
            COLOR_MISSILE,
            trail_alpha,
        )?;
    }

    draw_glow(
        canvas,
        missile.position,
        missile.radius + 8.0,
        COLOR_MISSILE,
        62,
    )?;

    let direction = missile.velocity.normalized_or(Vec2::new(1.0, 0.0));
    let tail_position = missile.position - direction * 12.0;
    draw_thick_line(
        canvas,
        tail_position,
        missile.position,
        4.0,
        COLOR_LAUNCHER_CORE,
        170,
    )?;

    draw_filled_circle(canvas, missile.position, missile.radius, COLOR_MISSILE, 255)?;
    draw_circle_outline(
        canvas,
        missile.position,
        missile.radius,
        COLOR_MISSILE_CORE,
        220,
    )?;
    draw_filled_circle(canvas, missile.position, 2.6, COLOR_MISSILE_CORE, 255)?;

    Ok(())
}

fn draw_bullet(canvas: &mut Canvas<Window>, bullet: &Bullet) -> Result<(), sdl3::Error> {
    draw_glow(
        canvas,
        bullet.position,
        bullet.radius + 5.0,
        COLOR_BULLET,
        80,
    )?;
    draw_filled_circle(
        canvas,
        bullet.position,
        bullet.radius,
        COLOR_BULLET_CORE,
        255,
    )?;
    draw_filled_circle(
        canvas,
        bullet.position,
        bullet.radius * 0.55,
        [255, 255, 255],
        255,
    )
}

fn draw_particle(canvas: &mut Canvas<Window>, particle: &Particle) -> Result<(), sdl3::Error> {
    let life_ratio = particle.life_ratio();
    let alpha = (life_ratio * 255.0).clamp(0.0, 255.0) as u8;
    let radius = (particle.radius * (0.45 + life_ratio * 0.75)).max(0.6);

    draw_filled_circle(canvas, particle.position, radius, particle.color, alpha)
}

fn draw_game_over_dim(canvas: &mut Canvas<Window>) -> Result<(), sdl3::Error> {
    canvas.set_blend_mode(BlendMode::Blend);
    canvas.set_draw_color(solid_color([4, 6, 12], 178));
    canvas.fill_rect(FRect::new(0.0, 0.0, ARENA_WIDTH, ARENA_HEIGHT))
}

fn draw_glow(
    canvas: &mut Canvas<Window>,
    center: Vec2,
    radius: f32,
    color: Rgb,
    alpha: u8,
) -> Result<(), sdl3::Error> {
    draw_filled_circle(canvas, center, radius, color, alpha / 3)?;
    draw_filled_circle(canvas, center, radius * 0.66, color, alpha / 2)
}

fn draw_filled_circle(
    canvas: &mut Canvas<Window>,
    center: Vec2,
    radius: f32,
    color: Rgb,
    alpha: u8,
) -> Result<(), sdl3::Error> {
    if radius <= 0.0 {
        return Ok(());
    }

    let segment_count = ((radius * 1.6) as usize).clamp(16, 64);
    let vertex_color = float_color(color, alpha);
    let mut vertices = Vec::with_capacity(segment_count * 3);

    for index in 0..segment_count {
        let start_angle = std::f32::consts::TAU * index as f32 / segment_count as f32;
        let end_angle = std::f32::consts::TAU * (index + 1) as f32 / segment_count as f32;
        let start_offset = Vec2::from_angle(start_angle, radius);
        let end_offset = Vec2::from_angle(end_angle, radius);

        vertices.push(Vertex {
            position: FPoint::new(center.horizontal, center.vertical),
            color: vertex_color,
            tex_coord: FPoint::new(0.5, 0.5),
        });
        vertices.push(Vertex {
            position: FPoint::new(
                center.horizontal + start_offset.horizontal,
                center.vertical + start_offset.vertical,
            ),
            color: vertex_color,
            tex_coord: FPoint::new(0.5, 0.5),
        });
        vertices.push(Vertex {
            position: FPoint::new(
                center.horizontal + end_offset.horizontal,
                center.vertical + end_offset.vertical,
            ),
            color: vertex_color,
            tex_coord: FPoint::new(0.5, 0.5),
        });
    }

    canvas.render_geometry(&vertices, None, VertexIndices::Sequential)
}

fn draw_circle_outline(
    canvas: &mut Canvas<Window>,
    center: Vec2,
    radius: f32,
    color: Rgb,
    alpha: u8,
) -> Result<(), sdl3::Error> {
    let segment_count = 48;
    let mut points = Vec::with_capacity(segment_count + 1);

    for index in 0..=segment_count {
        let angle = std::f32::consts::TAU * index as f32 / segment_count as f32;
        let offset = Vec2::from_angle(angle, radius);
        points.push(FPoint::new(
            center.horizontal + offset.horizontal,
            center.vertical + offset.vertical,
        ));
    }

    canvas.set_draw_color(solid_color(color, alpha));
    canvas.draw_lines(points.as_slice())
}

fn draw_thick_line(
    canvas: &mut Canvas<Window>,
    start: Vec2,
    end: Vec2,
    thickness: f32,
    color: Rgb,
    alpha: u8,
) -> Result<(), sdl3::Error> {
    let direction = (end - start).normalized_or(Vec2::new(1.0, 0.0));
    let normal = Vec2::new(-direction.vertical, direction.horizontal);
    let line_count = thickness.max(1.0).ceil() as i32;

    canvas.set_draw_color(solid_color(color, alpha));

    for line_index in 0..line_count {
        let offset = line_index as f32 - (line_count as f32 - 1.0) * 0.5;
        let shift = normal * offset;

        canvas.draw_line(
            FPoint::new(
                start.horizontal + shift.horizontal,
                start.vertical + shift.vertical,
            ),
            FPoint::new(
                end.horizontal + shift.horizontal,
                end.vertical + shift.vertical,
            ),
        )?;
    }

    Ok(())
}

fn draw_bar(
    canvas: &mut Canvas<Window>,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    ratio: f32,
    fill_color: Rgb,
) -> Result<(), sdl3::Error> {
    let clamped_ratio = ratio.clamp(0.0, 1.0);

    canvas.set_draw_color(solid_color([6, 9, 16], 220));
    canvas.fill_rect(FRect::new(left - 1.0, top - 1.0, width + 2.0, height + 2.0))?;

    if clamped_ratio > 0.0 {
        canvas.set_draw_color(solid_color(fill_color, 255));
        canvas.fill_rect(FRect::new(left, top, width * clamped_ratio, height))?;
    }

    canvas.set_draw_color(solid_color(fill_color, 150));
    canvas.draw_rect(FRect::new(left - 1.0, top - 1.0, width + 2.0, height + 2.0))?;

    Ok(())
}

fn health_color(health_ratio: f32) -> Rgb {
    if health_ratio > 0.55 {
        COLOR_HEALTH_FULL
    } else if health_ratio > 0.25 {
        COLOR_HEALTH_MEDIUM
    } else {
        COLOR_HEALTH_LOW
    }
}

fn solid_color(color: Rgb, alpha: u8) -> Color {
    Color::RGBA(color[0], color[1], color[2], alpha)
}

fn float_color(color: Rgb, alpha: u8) -> FColor {
    FColor::RGBA(
        color[0] as f32 / 255.0,
        color[1] as f32 / 255.0,
        color[2] as f32 / 255.0,
        alpha as f32 / 255.0,
    )
}
