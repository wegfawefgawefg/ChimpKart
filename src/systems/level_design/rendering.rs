use glam::Vec2;
use hecs::World;
use rapier2d::prelude::*;
use raylib::prelude::Color;

use crate::{
    components::{
        Ball, BallEater, Block, CTransform, Car, Health, Paddle, Physics, Shape, StrongBlock, Wall,
    },
    physics_engine::m2p,
    render_commands::RenderCommand,
    state::State,
    DIMS,
};

pub fn render(ecs: &World, state: &mut State) {
    // render map
    let map = &state.level_design_state.map;
    // render starting line as white
    state.render_command_buffer.push(RenderCommand::Line {
        start: map.starting_line.a,
        end: map.starting_line.b,
        color: Color::WHITE,
    });
    // render finish line as green
    state.render_command_buffer.push(RenderCommand::Line {
        start: map.finish_line.a,
        end: map.finish_line.b,
        color: Color::GREEN,
    });
    // render checkpoints as white
    for checkpoint in &map.checkpoints {
        state.render_command_buffer.push(RenderCommand::Line {
            start: checkpoint.a,
            end: checkpoint.b,
            color: Color::WHITE,
        });
    }
    // render walls as grey
    for wall in &map.walls {
        state.render_command_buffer.push(RenderCommand::Line {
            start: wall.a,
            end: wall.b,
            color: Color::GRAY,
        });
    }

    // render cars
    // for (_, (ctransform, shape)) in ecs.query::<(&CTransform, &Shape)>().with::<&Car>().iter() {
    //     state.render_command_buffer.push(RenderCommand::Rect {
    //         pos: ctransform.pos,
    //         dims: shape.dims,
    //         dir: ctransform.dir,
    //         color: Color::RED,
    //     });
    // }

    // render_physics(state);

    // let mut cursor = Vec2::new(20.0, 20.0);
    // for (_, physics) in ecs.query::<&Physics>().with::<&Ball>().iter() {
    //     state.render_command_buffer.push(RenderCommand::Text {
    //         pos: cursor,
    //         text: format!("vel: {}", physics.vel),
    //         size: 1,
    //         color: Color::new(255, 255, 255, 10),
    //     });
    //     cursor.y += 10.0;
    // }

    // // render walls
    // for (entity, (ctransform, shape)) in ecs.query::<(&CTransform, &Shape)>().with::<&Wall>().iter()
    // {
    //     // white if not a ball eater, red if it is
    //     let mut color: Color = Color::WHITE;
    //     if let Ok(mut r) = ecs.query_one::<&BallEater>(entity) {
    //         if let Some(_) = r.get() {
    //             color = Color::RED;
    //         }
    //     }
    //     state
    //         .render_command_buffer
    //         .push(RenderCommand::SolidRectangle {
    //             pos: ctransform.pos,
    //             dims: shape.dims,
    //             color,
    //         });
    // }

    // // render every player as a paddle
    // for (_, (_, ctransform, shape)) in ecs.query::<(&Paddle, &CTransform, &Shape)>().iter() {
    //     state.render_command_buffer.push(RenderCommand::Paddle {
    //         pos: ctransform.pos,
    //         dims: shape.dims,
    //         color: Color::RAYWHITE,
    //     })
    // }

    // // render every block
    // for (entity, (block, ctransform, shape, health)) in
    //     ecs.query::<(&Block, &CTransform, &Shape, &Health)>().iter()
    // {
    //     let ball_unbreakable = ecs.satisfies::<&StrongBlock>(entity).unwrap_or(false);
    //     state.render_command_buffer.push(RenderCommand::Block {
    //         pos: ctransform.pos,
    //         dims: shape.dims,
    //         color: block.color,
    //         hp: health.hp,
    //         ball_unbreakable,
    //     })
    // }

    // // render ball
    // for (_, (_, ctransform, shape)) in ecs.query::<(&Ball, &CTransform, &Shape)>().iter() {
    //     state.render_command_buffer.push(RenderCommand::Ball {
    //         pos: ctransform.pos,
    //         dims: shape.dims,
    //     })
    // }

    // // render the level in the top right
    // let cursor = Vec2::new(DIMS.x as f32 - 50.0, DIMS.y as f32 - 20.0);
    // let size = 1;
    // state.render_command_buffer.push(RenderCommand::Text {
    //     pos: cursor,
    //     text: format!("Level: {}", state.level),
    //     size,
    //     color: Color::WHITE,
    // });
}
