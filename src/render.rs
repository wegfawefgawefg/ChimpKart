use glam::Vec2;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

use crate::{
    render_commands::execute_render_command_buffer,
    state::{GameMode, State},
    DIMS,
};

pub fn draw(state: &State, low_res_draw_handle: &mut RaylibTextureMode<RaylibDrawHandle>) {
    match state.game_mode {
        GameMode::Title => {
            title_render(state, low_res_draw_handle);
        }
        GameMode::LevelDesign => {
            level_design_render(state, low_res_draw_handle);
        }
        _ => {}
    }
    execute_render_command_buffer(low_res_draw_handle, &state.render_command_buffer);
}

////////////////////////    PER GAME MODE DRAW FUNCTIONS     ////////////////////////
pub fn title_render(_state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let mut cursor = Vec2::new(DIMS.x as f32 * 0.15, DIMS.y as f32 * 0.4);
    let title = "Chimp Kart!";
    let size = 20;
    d.draw_text(title, cursor.x as i32, cursor.y as i32, size, Color::WHITE);
    cursor.y += size as f32 * 1.5;

    let subtitle = "press space to start";
    let size = 1;
    d.draw_text(
        subtitle,
        cursor.x as i32,
        cursor.y as i32,
        size,
        Color::WHITE,
    );
}

pub fn level_design_render(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    let mut cursor = Vec2::new(DIMS.x as f32 * 0.0, DIMS.y as f32 * 0.0);
    let title = "level design mode";
    let size = 10;
    d.draw_text(title, cursor.x as i32, cursor.y as i32, size, Color::WHITE);
    // cursor.y += size as f32 * 1.5;
}

const MESSAGES_OF_ENCOURAGEMENT: [&str; 5] = ["clean", "savage", "aww yeah", "sweet", "radical"];
