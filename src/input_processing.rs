use glam::Vec2;
use raylib::RaylibHandle;

use crate::state::{GameMode, State};

pub fn process_input(rl: &mut RaylibHandle, state: &mut State) {
    match state.game_mode {
        GameMode::Title => {
            title_process_input(rl, state);
        }
        GameMode::LevelDesign => {
            level_design_process_input(rl, state);
        }
        _ => {}
    }
}

////////////////////////    PER GAME MODE INPUT PROCESSING     ////////////////////////
pub fn title_process_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    let mouse_pos_rl = rl.get_mouse_position();
    let mouse_pos = Vec2::new(mouse_pos_rl.x, mouse_pos_rl.y);
    state.mouse_screen_pos = mouse_pos;

    let mut title_inputs = TitleInputs { confirm: false };
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        title_inputs.confirm = true;
    }
    if title_inputs.confirm {
        state.title_inputs = title_inputs;
        state.level = 1;
        state.game_mode = GameMode::LevelDesign;
    }
}

pub fn level_design_process_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    let mouse_pos_rl = rl.get_mouse_position();
    let mouse_pos = Vec2::new(mouse_pos_rl.x, mouse_pos_rl.y);
    state.mouse_screen_pos = mouse_pos;

    let mut inputs = LevelDesignInputs::new();
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
        inputs.left = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_D) {
        inputs.right = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
        inputs.gas = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
        inputs.brake = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
        inputs.drift_brake = true;
    }

    state.level_design_inputs = inputs;
}

////////////////////////    INPUT DEFS    ////////////////////////
pub struct TitleInputs {
    pub confirm: bool,
}
impl TitleInputs {
    pub fn new() -> TitleInputs {
        TitleInputs { confirm: false }
    }
}

pub struct LevelDesignInputs {
    pub left: bool,
    pub right: bool,
    pub gas: bool,
    pub brake: bool,
    pub drift_brake: bool,

    pub confirm: bool,
}
impl LevelDesignInputs {
    pub fn new() -> LevelDesignInputs {
        LevelDesignInputs {
            left: false,
            right: false,
            gas: false,
            brake: false,
            drift_brake: false,

            confirm: false,
        }
    }
}
