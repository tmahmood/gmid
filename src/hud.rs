use egui_sdl3::egui::{self, Align2, Color32, RichText, Stroke};

use crate::game::GameState;
use crate::theme::PLAYER_MAX_HEALTH;

const TEXT_PRIMARY: Color32 = Color32::from_rgb(222, 238, 255);
const TEXT_MUTED: Color32 = Color32::from_rgb(126, 156, 190);
const ACCENT: Color32 = Color32::from_rgb(72, 225, 255);
const DANGER: Color32 = Color32::from_rgb(255, 92, 116);
const HEALTH_FULL: Color32 = Color32::from_rgb(110, 230, 130);
const HEALTH_MEDIUM: Color32 = Color32::from_rgb(255, 200, 90);
const HEALTH_LOW: Color32 = Color32::from_rgb(255, 90, 90);
const PANEL_BORDER: Color32 = Color32::from_rgb(48, 76, 110);

/// Draws the whole HUD. Returns true when the player asked for a restart.
pub fn draw_hud(egui_context: &egui::Context, game: &GameState) -> bool {
    draw_status_panel(egui_context, game);
    draw_threat_panel(egui_context, game);
    draw_controls_hint(egui_context);

    if game.game_over {
        draw_game_over_panel(egui_context, game)
    } else {
        false
    }
}

fn draw_status_panel(egui_context: &egui::Context, game: &GameState) {
    egui::Area::new(egui::Id::new("status_panel"))
        .anchor(Align2::LEFT_TOP, egui::vec2(26.0, 22.0))
        .show(egui_context, |area_ui| {
            egui::Frame::new()
                .fill(panel_background())
                .stroke(Stroke::new(1.0_f32, PANEL_BORDER))
                .corner_radius(12)
                .inner_margin(16)
                .show(area_ui, |panel_ui| {
                    panel_ui.set_min_width(238.0);

                    panel_ui.label(RichText::new("HULL INTEGRITY").small().color(TEXT_MUTED));
                    let health_ratio = game.player.health_ratio();
                    let health_color = health_color_for_ratio(health_ratio);

                    panel_ui.add(
                        egui::ProgressBar::new(health_ratio)
                            .desired_width(238.0)
                            .fill(health_color)
                            .text(
                                RichText::new(format!(
                                    "{:.0} / {:.0}",
                                    game.player.health, PLAYER_MAX_HEALTH
                                ))
                                .color(Color32::from_rgb(8, 12, 20))
                                .strong(),
                            ),
                    );

                    panel_ui.add_space(10.0);
                    egui::Grid::new("score_grid")
                        .num_columns(2)
                        .spacing([22.0, 4.0])
                        .show(panel_ui, |grid_ui| {
                            grid_ui.label(RichText::new("SCORE").small().color(TEXT_MUTED));
                            grid_ui.label(
                                RichText::new(format!("{:05}", game.score))
                                    .size(20.0)
                                    .strong()
                                    .color(ACCENT),
                            );
                            grid_ui.end_row();

                            grid_ui.label(
                                RichText::new("LAUNCHERS DESTROYED")
                                    .small()
                                    .color(TEXT_MUTED),
                            );
                            grid_ui.label(
                                RichText::new(format!("{}", game.launchers_destroyed))
                                    .strong()
                                    .color(TEXT_PRIMARY),
                            );
                            grid_ui.end_row();
                        });
                });
        });
}

fn draw_threat_panel(egui_context: &egui::Context, game: &GameState) {
    egui::Area::new(egui::Id::new("threat_panel"))
        .anchor(Align2::RIGHT_TOP, egui::vec2(-26.0, 22.0))
        .show(egui_context, |area_ui| {
            egui::Frame::new()
                .fill(panel_background())
                .stroke(Stroke::new(1.0_f32, PANEL_BORDER))
                .corner_radius(12)
                .inner_margin(14)
                .show(area_ui, |panel_ui| {
                    panel_ui.set_min_width(176.0);
                    panel_ui.label(
                        RichText::new("THREAT BOARD")
                            .small()
                            .strong()
                            .color(TEXT_MUTED),
                    );
                    panel_ui.add_space(6.0);

                    egui::Grid::new("threat_grid")
                        .num_columns(2)
                        .spacing([20.0, 5.0])
                        .show(panel_ui, |grid_ui| {
                            grid_ui
                                .label(RichText::new("MISSILES INBOUND").small().color(TEXT_MUTED));
                            grid_ui.label(
                                RichText::new(format!("{:02}", game.missiles.len()))
                                    .strong()
                                    .color(DANGER),
                            );
                            grid_ui.end_row();

                            grid_ui
                                .label(RichText::new("LAUNCHERS ONLINE").small().color(TEXT_MUTED));
                            grid_ui.label(
                                RichText::new(format!(
                                    "{} / {}",
                                    game.active_launcher_count(),
                                    game.launchers.len()
                                ))
                                .strong()
                                .color(TEXT_PRIMARY),
                            );
                            grid_ui.end_row();

                            grid_ui.label(RichText::new("SURVIVED").small().color(TEXT_MUTED));
                            grid_ui.label(
                                RichText::new(format_survival_time(game.survived_seconds))
                                    .strong()
                                    .color(TEXT_PRIMARY),
                            );
                            grid_ui.end_row();
                        });
                });
        });
}

fn draw_controls_hint(egui_context: &egui::Context) {
    egui::Area::new(egui::Id::new("controls_hint"))
        .anchor(Align2::CENTER_BOTTOM, egui::vec2(0.0, -16.0))
        .show(egui_context, |area_ui| {
            egui::Frame::new()
                .fill(panel_background())
                .stroke(Stroke::new(1.0_f32, PANEL_BORDER))
                .corner_radius(9)
                .inner_margin(egui::Margin::symmetric(16, 8))
                .show(area_ui, |panel_ui| {
                    panel_ui.set_min_width(700.0);
                    panel_ui.add(
                        egui::Label::new(
                            RichText::new(
                                "WASD / ARROWS  move     MOUSE  aim     SPACE  fire     R  restart     ESC  quit",
                            )
                            .monospace()
                            .size(12.0)
                            .color(TEXT_MUTED),
                        )
                        .wrap_mode(egui::TextWrapMode::Extend),
                    );
                });
        });
}

fn draw_game_over_panel(egui_context: &egui::Context, game: &GameState) -> bool {
    let mut restart_requested = false;

    egui::Area::new(egui::Id::new("game_over_panel"))
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(egui_context, |area_ui| {
            egui::Frame::new()
                .fill(panel_background())
                .stroke(Stroke::new(1.0_f32, DANGER))
                .corner_radius(18)
                .inner_margin(30)
                .show(area_ui, |panel_ui| {
                    panel_ui.set_min_width(320.0);
                    panel_ui.vertical_centered(|centered_ui| {
                        centered_ui.label(
                            RichText::new("SYSTEM DOWN")
                                .size(34.0)
                                .strong()
                                .color(DANGER),
                        );
                        centered_ui.add_space(8.0);
                        centered_ui.label(
                            RichText::new(format!("FINAL SCORE   {:05}", game.score))
                                .size(20.0)
                                .strong()
                                .color(TEXT_PRIMARY),
                        );
                        centered_ui.label(
                            RichText::new(format!(
                                "SURVIVED   {}",
                                format_survival_time(game.survived_seconds)
                            ))
                            .color(TEXT_MUTED),
                        );
                        centered_ui.add_space(18.0);

                        let restart_response = centered_ui.add(
                            egui::Button::new(
                                RichText::new("RESTART")
                                    .size(18.0)
                                    .strong()
                                    .color(Color32::from_rgb(6, 12, 20)),
                            )
                            .fill(ACCENT)
                            .corner_radius(9)
                            .min_size(egui::vec2(170.0, 42.0)),
                        );

                        if restart_response.clicked() {
                            restart_requested = true;
                        }

                        centered_ui.add_space(8.0);
                        centered_ui.label(RichText::new("or press R").small().color(TEXT_MUTED));
                    });
                });
        });

    restart_requested
}

fn health_color_for_ratio(health_ratio: f32) -> Color32 {
    if health_ratio > 0.55 {
        HEALTH_FULL
    } else if health_ratio > 0.25 {
        HEALTH_MEDIUM
    } else {
        HEALTH_LOW
    }
}

fn format_survival_time(survived_seconds: f32) -> String {
    let total_seconds = survived_seconds.max(0.0) as u32;
    format!("{:02}:{:02}", total_seconds / 60, total_seconds % 60)
}

fn panel_background() -> Color32 {
    Color32::from_rgba_unmultiplied(11, 17, 29, 225)
}
