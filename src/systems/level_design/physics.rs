use hecs::World;

use crate::{
    components::{CTransform, Physics},
    state::State,
};

pub const MAX_SPEED: f32 = 3.0;
pub const MAX_REVERSE_SPEED: f32 = -1.0;
pub fn step_physics(ecs: &World, state: &mut State) {
    // step
    for (_, (ctransform, physics)) in ecs.query::<(&mut CTransform, &mut Physics)>().iter() {
        ctransform.pos += physics.vel;
        ctransform.rot += physics.rot_vel;
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
