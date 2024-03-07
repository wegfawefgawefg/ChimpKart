use glam::Vec2;
use hecs::World;

use crate::{
    components::{CTransform, Car, Paddle, Physics, Shape},
    state::State,
    utils::vec2_lerp,
};

pub const ACC_FORCE: f32 = 0.01;
pub const REVERSE_FORCE: f32 = 0.01;
pub const BREAK_FORCE: f32 = 0.01;
pub const ROT_SPEED: f32 = 0.03;
pub const ROT_SPEED_LOSS_FRAC: f32 = 0.99;
pub const HANDLING: f32 = 0.1;
pub fn process_inputs(ecs: &mut World, state: &mut State) {
    for (_, (ctransform, physics)) in ecs
        .query::<(&mut CTransform, &mut Physics)>()
        .with::<&Car>()
        .iter()
    {
        // gas
        if state.level_design_inputs.gas {
            let dir = ctransform.dir;
            physics.vel += dir * ACC_FORCE;
        }

        let velmag = physics.vel.length();
        let veldir = physics.vel.normalize();
        println!("vel: {}, dir: {}", velmag, veldir);
        // brake
        if state.level_design_inputs.brake {
            // do nothing if car not moving
            if velmag > 0.0 {
                println!("brake");
                let veldir = physics.vel.normalize();
                let break_vector = -veldir * BREAK_FORCE;
                let break_vector_mag = break_vector.length();

                // print the velmag and the breakmag
                // println!("velmag: {}, breakmag: {}", velmag, break_vector_mag);

                if break_vector_mag < velmag {
                    physics.vel += break_vector;
                } else {
                    // physics.vel = Vec2::ZERO;
                }
            }
        }

        // rotate
        if state.level_design_inputs.left {
            let new_angle = ctransform.dir.to_angle() - ROT_SPEED;
            ctransform.dir = Vec2::from_angle(new_angle);

            let vel_mag = physics.vel.length();
            let new_vel = ctransform.dir * vel_mag * ROT_SPEED_LOSS_FRAC;
            physics.vel = vec2_lerp(physics.vel, new_vel, HANDLING);
        }
        if state.level_design_inputs.right {
            let new_angle = ctransform.dir.to_angle() + ROT_SPEED;
            ctransform.dir = Vec2::from_angle(new_angle);

            let vel_mag = physics.vel.length();
            let new_vel = ctransform.dir * vel_mag * ROT_SPEED_LOSS_FRAC;
            physics.vel = vec2_lerp(physics.vel, new_vel, HANDLING);
        }

        // print rotation
        // // reverse
        // if state.level_design_inputs.brake {
        //     let vel = physics.vel.length();
        //     let dir = physics.physics.vel = -dir * REVERSE_FORCE;
        //     // if speed under threshold, become zero
        //     if velmag < 0.1 {
        //         physics.vel = Vec2::ZERO;
        //     }
        // }
    }

    // if state.level_change_delay > 0 {
    //     return;
    // }
    // if state.playing_inputs.next_level {
    //     state.level += 1;
    //     state.level_change_delay = LEVEL_CHANGE_DELAY_DEFAULT;
    //     spawn_level(ecs, state, state.level);
    // } else if state.playing_inputs.previous_level {
    //     if state.level == 1 {
    //         return;
    //     }
    //     state.level -= 1;
    //     state.level_change_delay = LEVEL_CHANGE_DELAY_DEFAULT;
    //     spawn_level(ecs, state, state.level);
    // }

    // if state.playing_inputs.restart_level {
    //     state.level_change_delay = LEVEL_CHANGE_DELAY_DEFAULT;
    //     state.next_game_mode = Some(crate::state::GameMode::PrepareLevel);
    // }
}
