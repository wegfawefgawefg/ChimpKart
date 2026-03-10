use glam::Vec2;
use hecs::World;
use nalgebra::vector;
use raylib::color::Color;

use crate::{
    components::{CTransform, Car, Physics},
    physics_engine::m2p,
    render_commands::RenderCommand,
    state::State,
};

pub const MAX_SPEED: f32 = 3.0;
pub const MAX_REVERSE_SPEED: f32 = -1.0;
pub const TIRE_RESISTANCE: f32 = 0.1;
pub const VEL_NOT_FACING_PENALTY: f32 = 0.05;
pub const VEL_FACING_BIAS: f32 = 0.1;
// pub const VEL_FACING_BIAS: f32 = 0.02;
pub fn step_physics(ecs: &World, state: &mut State) {
    for (entity, _) in ecs.query::<&Car>().into_iter() {
        if let Some(rigid_body_handle) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(rigid_body_handle) {
                let vel = rigid_body.linvel();
                let velmag = vel.magnitude();
                if velmag <= 0.0 {
                    continue;
                }

                // velocity penalty based on direction of facing

                let angle = rigid_body.position().rotation.angle();
                let dir = Vec2::from_angle(angle);

                let vel_dir = vel.normalize();
                let vel_dir = Vec2::new(vel_dir.x, vel_dir.y);

                let dot = vel_dir.dot(dir);
                let dot = dot.abs();
                let agreement = 1.0 - dot;
                let anti_vel = -agreement * VEL_NOT_FACING_PENALTY * vel;

                // draw this line
                let pos = rigid_body.position().translation.vector;
                let pos = Vec2::new(m2p(pos.x), m2p(pos.y));
                let start = Vec2::new(pos.x, pos.y);
                let av_scaled = anti_vel * 10000.0;
                let end = Vec2::new(pos.x + av_scaled.x, pos.y + av_scaled.y);
                let yellow = Color::new(255, 255, 0, 255);
                state.render_command_buffer.push(RenderCommand::Line {
                    start,
                    end,
                    color: yellow,
                });
                // println!("anti_vel: {:?}", anti_vel);
                // print pos
                println!("pos: {:?}", pos);

                rigid_body.apply_impulse(anti_vel, true);

                // but also convert some of the velocity to the direction of the car
                let vel_facing_bias = dot * VEL_FACING_BIAS * velmag;
                let facing_bias_force = dir * vel_facing_bias;
                // now reduce the old velocity by this amount also
                let facing_bias_resistance = -vel_dir * vel_facing_bias;
                let facing_bias_resistance =
                    vector![facing_bias_resistance.x, facing_bias_resistance.y];
                rigid_body.apply_impulse(facing_bias_resistance, true);

                // draw this line in green
                let fb_scaled = facing_bias_force * 500.0;
                let end = Vec2::new(pos.x + fb_scaled.x, pos.y + fb_scaled.y);
                let green = Color::new(0, 255, 0, 255);
                state.render_command_buffer.push(RenderCommand::Line {
                    start,
                    end,
                    color: green,
                });

                let facing_bias_force = vector![facing_bias_force.x, facing_bias_force.y];
                rigid_body.apply_impulse(facing_bias_force, true);

                // if state.level_design_inputs.gas {
                //     let force = dir * ACC_FORCE;
                //     let nalg_force = vector![force.x, force.y];
                //     rigid_body.apply_impulse(nalg_force, true);
                // }
                // if state.level_design_inputs.right {
                //     rigid_body.apply_torque_impulse(ROT_FORCE, true);
                // }
            }
        }
    }

    // // step
    // for (_, (ctransform, physics)) in ecs.query::<(&mut CTransform, &mut Physics)>().iter() {
    //     ctransform.pos += physics.vel;
    //     ctransform.dir += physics.rot_vel;
    // }

    // // speed clamp
    // for (_, physics) in ecs.query::<&mut Physics>().iter() {
    //     let velmag = physics.vel.length();
    //     if velmag > MAX_SPEED {
    //         physics.vel = physics.vel.normalize() * MAX_SPEED;
    //     }
    //     if velmag < MAX_REVERSE_SPEED {
    //         physics.vel = physics.vel.normalize() * MAX_REVERSE_SPEED;
    //     }
    // }
}
