mod entities;
mod game;
mod geometry;
mod hud;
mod render;
mod theme;

use std::time::{Duration, Instant};

use egui_sdl3::{egui, sdl3};
use sdl3::event::Event;
use sdl3::keyboard::Scancode;

use game::{FrameInput, GameState};
use geometry::Vec2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("gmid - arena shooter", 1280, 720)
        .position_centered()
        .build()?;

    let mut egui_canvas = egui_sdl3::canvas::EguiCanvas::new(window);
    egui_canvas.ctx.set_visuals(egui::Visuals::dark());

    let mut event_pump = sdl_context.event_pump()?;
    let mut game = GameState::new();
    let mut pointer_position =
        Vec2::new(theme::ARENA_WIDTH * 0.5 + 180.0, theme::ARENA_HEIGHT * 0.5);

    let mut running = true;
    let mut previous_frame_instant = Instant::now();
    let target_frame_duration = Duration::from_secs_f32(1.0 / 120.0);

    while running {
        let frame_start_instant = Instant::now();
        let delta_seconds = (frame_start_instant - previous_frame_instant)
            .as_secs_f32()
            .min(0.05);
        previous_frame_instant = frame_start_instant;

        for event in event_pump.poll_iter() {
            let event_response = egui_canvas.on_event(&event);

            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::KeyDown {
                    scancode: Some(scancode),
                    repeat: false,
                    ..
                } => match scancode {
                    Scancode::Escape => running = false,
                    Scancode::R => game.reset(),
                    _ => {}
                },
                Event::MouseMotion {
                    x: pointer_horizontal,
                    y: pointer_vertical,
                    ..
                } if !event_response.consumed => {
                    pointer_position = scale_window_pointer_to_canvas(
                        &egui_canvas,
                        pointer_horizontal,
                        pointer_vertical,
                    );
                }
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        let input = FrameInput {
            movement: read_movement_input(&keyboard_state),
            firing: keyboard_state.is_scancode_pressed(Scancode::Space),
            aim_target: pointer_position,
        };

        game.update(delta_seconds, input);

        let background = theme::COLOR_BACKGROUND;
        egui_canvas.clear([background[0], background[1], background[2], 255]);

        if let Err(render_error) = render::render_world(&mut egui_canvas.painter.canvas, &game) {
            eprintln!("Render error: {render_error}");
            break;
        }

        let mut restart_requested = false;
        egui_canvas.run(|egui_context| {
            restart_requested = hud::draw_hud(egui_context, &game);
        });

        if restart_requested {
            game.reset();
        }

        egui_canvas.paint();
        egui_canvas.present();

        let frame_elapsed = frame_start_instant.elapsed();
        if frame_elapsed < target_frame_duration {
            std::thread::sleep(target_frame_duration - frame_elapsed);
        }
    }

    egui_canvas.destroy();
    Ok(())
}

fn read_movement_input(keyboard_state: &sdl3::keyboard::KeyboardState) -> Vec2 {
    let mut horizontal_input = 0.0;
    let mut vertical_input = 0.0;

    if keyboard_state.is_scancode_pressed(Scancode::A)
        || keyboard_state.is_scancode_pressed(Scancode::Left)
    {
        horizontal_input -= 1.0;
    }
    if keyboard_state.is_scancode_pressed(Scancode::D)
        || keyboard_state.is_scancode_pressed(Scancode::Right)
    {
        horizontal_input += 1.0;
    }
    if keyboard_state.is_scancode_pressed(Scancode::W)
        || keyboard_state.is_scancode_pressed(Scancode::Up)
    {
        vertical_input -= 1.0;
    }
    if keyboard_state.is_scancode_pressed(Scancode::S)
        || keyboard_state.is_scancode_pressed(Scancode::Down)
    {
        vertical_input += 1.0;
    }

    Vec2::new(horizontal_input, vertical_input)
}

fn scale_window_pointer_to_canvas(
    egui_canvas: &egui_sdl3::canvas::EguiCanvas,
    pointer_horizontal: f32,
    pointer_vertical: f32,
) -> Vec2 {
    let (window_width, window_height) = egui_canvas.painter.canvas.window().size();
    let (output_width, output_height) = egui_canvas
        .painter
        .canvas
        .output_size()
        .unwrap_or((window_width, window_height));

    let horizontal_scale = output_width as f32 / window_width.max(1) as f32;
    let vertical_scale = output_height as f32 / window_height.max(1) as f32;

    Vec2::new(
        pointer_horizontal * horizontal_scale,
        pointer_vertical * vertical_scale,
    )
}
