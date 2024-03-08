use glam::Vec2;
use hecs::World;
use rapier2d::prelude::*;
use raylib::prelude::Color;

use crate::{
    components::{Ball, BallEater, Block, CTransform, Health, Physics, Shape, StrongBlock, Wall},
    physics_engine::m2p,
    render_commands::RenderCommand,
    state::State,
    DIMS,
};

pub fn render(ecs: &World, state: &mut State) {
    // render_physics(state);

    let mut cursor = Vec2::new(20.0, 20.0);
    for (_, physics) in ecs.query::<&Physics>().with::<&Ball>().iter() {
        state.render_command_buffer.push(RenderCommand::Text {
            pos: cursor,
            text: format!("vel: {}", physics.vel),
            size: 1,
            color: Color::new(255, 255, 255, 10),
        });
        cursor.y += 10.0;
    }

    // render walls
    for (entity, (ctransform, shape)) in ecs.query::<(&CTransform, &Shape)>().with::<&Wall>().iter()
    {
        // white if not a ball eater, red if it is
        let mut color: Color = Color::WHITE;
        if let Ok(mut r) = ecs.query_one::<&BallEater>(entity) {
            if let Some(_) = r.get() {
                color = Color::RED;
            }
        }
        state
            .render_command_buffer
            .push(RenderCommand::SolidRectangle {
                pos: ctransform.pos,
                dims: shape.dims,
                color,
            });
    }

    // render the level in the top right
    let cursor = Vec2::new(DIMS.x as f32 - 50.0, DIMS.y as f32 - 20.0);
    let size = 1;
    state.render_command_buffer.push(RenderCommand::Text {
        pos: cursor,
        text: format!("Level: {}", state.level),
        size,
        color: Color::WHITE,
    });
}
