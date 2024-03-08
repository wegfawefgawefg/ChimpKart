use hecs::World;
use raylib::RaylibHandle;

use crate::{
    state::{GameMode, State},
    systems::{self},
};

pub fn step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {
    match state.game_mode {
        GameMode::Title => {
            title_step(state, ecs);
        }
        GameMode::LevelDesign => {
            level_design_step(state, ecs);
        }
        _ => {}
    }
}

////////////////////////    PER GAME MODE STEPPING     ////////////////////////
pub fn title_step(state: &mut State, ecs: &mut World) {}

pub fn playing_step(state: &mut State, ecs: &mut World) {
    if state.level_change_delay > 0 {
        state.level_change_delay -= 1;
    }

    // systems::playing::physics::constantly_resize_paddle(ecs, state);

    systems::playing::input_processing::process_inputs(ecs, state);
    // systems::playing::physics::boundary_checking(ecs, state);

    // all reshaping needs to happen before the ecs is synced to physics

    systems::playing::physics::set_ball_to_angle(ecs, state);
    systems::playing::physics::sync_ecs_to_physics(ecs, state);
    systems::playing::physics::step_physics(ecs, state);
    systems::playing::physics::respond_to_collisions(ecs, state);
    systems::playing::cleanup::process_deletion_events(ecs, state);
    systems::playing::state_changing::check_for_level_complete(ecs, state);
    systems::playing::state_changing::check_for_level_lost(ecs, state);
    systems::playing::rendering::render(ecs, state);
}

pub fn level_design_step(state: &mut State, ecs: &mut World) {
    systems::level_design::input_processing::process_inputs(ecs, state);
    // systems::level_design::physics::step_physics(ecs, state);
    systems::playing::physics::step_physics(ecs, state);
    systems::level_design::rendering::render(ecs, state);
    systems::common::rendering::render_physics(state);
}
