use glam::Vec2;
use hecs::World;

use crate::{
    components::{CTransform, Car, Physics},
    state::State,
};

pub const MAX_SPEED: f32 = 3.0;
pub const MAX_REVERSE_SPEED: f32 = -1.0;
pub const TIRE_RESISTANCE: f32 = 0.1;
pub fn step_physics(ecs: &World, state: &mut State) {
    // bias movement in direction of facing
    for (entity, _) in ecs.query::<&Car>().into_iter() {
        if let Some(rigid_body_handle) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(rigid_body_handle) {
                let angle = rigid_body.position().rotation.angle();
                let dir = Vec2::from_angle(angle);

                let vel = rigid_body.linvel();
                let vel_dir = vel.normalize();
                let vel_dir = Vec2::new(vel_dir.x, vel_dir.y);

                let dot = vel_dir.dot(dir);
                let dot = dot.abs();

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

    // step
    for (_, (ctransform, physics)) in ecs.query::<(&mut CTransform, &mut Physics)>().iter() {
        ctransform.pos += physics.vel;
        ctransform.dir += physics.rot_vel;
    }

    // speed clamp
    for (_, physics) in ecs.query::<&mut Physics>().iter() {
        let velmag = physics.vel.length();
        if velmag > MAX_SPEED {
            physics.vel = physics.vel.normalize() * MAX_SPEED;
        }
        if velmag < MAX_REVERSE_SPEED {
            physics.vel = physics.vel.normalize() * MAX_REVERSE_SPEED;
        }
    }
}
