use glam::Vec2;
use rapier2d::prelude::*;
use raylib::prelude::Color;

use crate::{
    physics_engine::m2p, render_commands::RenderCommand, state::State, utils::degrees_to_radians,
};

pub fn render_physics(state: &mut State) {
    // Render colliders
    for (_, collider) in state.physics.collider_set.iter() {
        let center = collider.position().translation.vector;
        let shape = collider.shape();
        let shape_type = shape.shape_type();

        if let ShapeType::Cuboid = shape_type {
            let cuboid = shape.as_cuboid().unwrap();
            let tl = center + -cuboid.half_extents;
            let size = cuboid.half_extents * 2.0;

            let ppos = Vec2::new(m2p(tl.x), m2p(tl.y));
            let psize = Vec2::new(m2p(size.x), m2p(size.y));
            // state.render_command_buffer.push(RenderCommand::SolidRectangle { pos: (), dims: (), color: () } {
            //     pos: ppos,
            //     dims: psize,
            //     color: Color::RED, // or any color you prefer for debug
            //     hp: 1,
            //     ball_unbreakable: false,
            // });
        }
    }

    // Render rigid bodies (Optional, if you need to distinguish them)
    for (_, rigid_body) in state.physics.rigid_body_set.iter() {
        // draw the directions
        let pos = rigid_body.position().translation.vector;
        let rot = rigid_body.position().rotation.angle();
        let rot = Vec2::from_angle(rot);

        let ppos = Vec2::new(m2p(pos.x), m2p(pos.y));
        state.render_command_buffer.push(RenderCommand::Line {
            start: ppos,
            end: ppos + rot * 10.0,
            color: Color::GREEN, // or any color you prefer for debug
        });

        for collider_handle in rigid_body.colliders().iter() {
            if let Some(collider) = state.physics.collider_set.get_mut(*collider_handle) {
                let center = collider.position().translation.vector;
                let shape = collider.shape();
                let shape_type = shape.shape_type();

                match shape_type {
                    ShapeType::Cuboid => {
                        let pcenter = Vec2::new(m2p(center.x), m2p(center.y));
                        let cuboid = shape.as_cuboid().unwrap();

                        let size = cuboid.half_extents * 2.0;
                        let pdims = Vec2::new(m2p(size.x), m2p(size.y));

                        let angle = collider.position().rotation.angle();
                        let dir = Vec2::from_angle(angle);

                        state.render_command_buffer.push(RenderCommand::LinesRect {
                            center: pcenter,
                            dims: pdims,
                            color: Color::WHITE,
                            dir,
                        })
                    }
                    ShapeType::Segment => {
                        let segment = shape.as_segment().unwrap();
                        let pstart = Vec2::new(m2p(segment.a.x), m2p(segment.a.y));
                        let pend = Vec2::new(m2p(segment.b.x), m2p(segment.b.y));
                        state.render_command_buffer.push(RenderCommand::Line {
                            start: pstart,
                            end: pend,
                            color: Color::WHITE,
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}
