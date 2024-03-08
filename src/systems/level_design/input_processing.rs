use glam::Vec2;
use hecs::{With, World};
use nalgebra::vector;

use crate::{
    components::{CTransform, Car, InputControlled, Paddle, Physics, Shape},
    state::State,
    utils::vec2_lerp,
};

pub const ACC_FORCE: f32 = 0.01;
pub const REVERSE_FORCE: f32 = 0.01;
pub const BREAK_FORCE: f32 = 0.01;
pub const ROT_FORCE: f32 = 0.0001;
pub const ROT_SPEED_LOSS_FRAC: f32 = 0.99;
pub const HANDLING: f32 = 0.3;
pub fn process_inputs(ecs: &mut World, state: &mut State) {
    for (entity, _) in ecs.query::<&Car>().with::<&InputControlled>().into_iter() {
        // fetch the rigid body associated with the car entity
        // get direction
        // add force in that direction

        if let Some(rigid_body_handle) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(rigid_body_handle) {
                if state.level_design_inputs.gas {
                    let angle = rigid_body.position().rotation.angle();
                    let dir = Vec2::from_angle(angle);
                    let force = dir * ACC_FORCE;
                    let nalg_force = vector![force.x, force.y];
                    rigid_body.apply_impulse(nalg_force, true);
                }
                if state.level_design_inputs.reverse {
                    let angle = rigid_body.position().rotation.angle();
                    let dir = Vec2::from_angle(angle);
                    let force = -dir * REVERSE_FORCE;
                    let nalg_force = vector![force.x, force.y];
                    rigid_body.apply_impulse(nalg_force, true);
                }
                if state.level_design_inputs.left {
                    rigid_body.apply_torque_impulse(-ROT_FORCE, true);
                }
                if state.level_design_inputs.right {
                    rigid_body.apply_torque_impulse(ROT_FORCE, true);
                }
            }
        }
    }

    for (_, (ctransform, physics)) in ecs
        .query::<(&mut CTransform, &mut Physics)>()
        .with::<&Car>()
        .iter()
    {
        // gas
        // if state.level_design_inputs.gas {
        //     let dir = ctransform.dir;
        //     physics.vel += dir * ACC_FORCE;
        // }

        // let velmag = physics.vel.length();
        // let veldir = physics.vel.normalize();
        // // brake
        // if state.level_design_inputs.brake {
        //     // do nothing if car not moving
        //     if velmag > 0.0 {
        //         println!("brake");
        //         let veldir = physics.vel.normalize();
        //         let break_vector = -veldir * BREAK_FORCE;
        //         let break_vector_mag = break_vector.length();

        //         // print the velmag and the breakmag
        //         // println!("velmag: {}, breakmag: {}", velmag, break_vector_mag);

        //         if break_vector_mag < velmag {
        //             physics.vel += break_vector;
        //         } else {
        //             // physics.vel = Vec2::ZERO;
        //         }
        //     }
        // }

        // rotate
        // if state.level_design_inputs.left {
        //     let new_angle = ctransform.dir.to_angle() - ROT_SPEED;
        //     ctransform.dir = Vec2::from_angle(new_angle);

        //     let vel_mag = physics.vel.length();
        //     let new_vel = ctransform.dir * vel_mag * ROT_SPEED_LOSS_FRAC;
        //     physics.vel = vec2_lerp(physics.vel, new_vel, HANDLING);
        // }
        // if state.level_design_inputs.right {
        //     let new_angle = ctransform.dir.to_angle() + ROT_SPEED;
        //     ctransform.dir = Vec2::from_angle(new_angle);

        //     let vel_mag = physics.vel.length();
        //     let new_vel = ctransform.dir * vel_mag * ROT_SPEED_LOSS_FRAC;
        //     physics.vel = vec2_lerp(physics.vel, new_vel, HANDLING);
        // }

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
