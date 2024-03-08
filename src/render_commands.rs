use glam::Vec2;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode, Vector2};

use crate::utils::{degrees_to_radians, radians_to_degrees};

pub type RenderCommandBuffer = Vec<RenderCommand>;

#[derive(Clone)]
pub enum RenderCommand {
    Rect {
        pos: Vec2,
        dims: Vec2,
        color: Color,
        dir: Vec2,
    },
    LinesRect {
        center: Vec2,
        dims: Vec2,
        color: Color,
        dir: Vec2,
    },
    ColoredSquare {
        pos: Vec2,
        color: Color,
    },
    Text {
        pos: Vec2,
        text: String,
        size: i32,
        color: Color,
    },
    Line {
        start: Vec2,
        end: Vec2,
        color: Color,
    },
    Circle {
        pos: Vec2,
        radius: f32,
        color: Color,
    },
    SolidRectangle {
        pos: Vec2,
        dims: Vec2,
        color: Color,
    },
}

const LINE_THICKNESS: i32 = 1;

pub fn execute_render_command_buffer(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    render_command_buffer: &RenderCommandBuffer,
) {
    for command in render_command_buffer.iter() {
        match command {
            RenderCommand::Rect {
                pos,
                dims,
                color,
                dir,
            } => {
                let center = Vector2::new(pos.x + dims.x / 2.0, pos.y + dims.y / 2.0);
                let rectangle = raylib::ffi::Rectangle {
                    x: pos.x,
                    y: pos.y,
                    width: dims.x,
                    height: dims.y,
                };
                d.draw_rectangle_pro(
                    rectangle,
                    Vector2::new(dims.x / 2.0, dims.y / 2.0),
                    radians_to_degrees(dir.to_angle()) - 90.0,
                    *color,
                );
            }
            RenderCommand::LinesRect {
                center,
                dims,
                color,
                dir,
            } => {
                // let dir = Vec2::from_angle(degrees_to_radians(90.0)).rotate(*dir);
                let half_dims = *dims / 2.0;
                let a = *center + Vec2::new(-half_dims.x, -half_dims.y);
                let b = *center + Vec2::new(half_dims.x, -half_dims.y);
                let c = *center + Vec2::new(half_dims.x, half_dims.y);
                let dd = *center + Vec2::new(-half_dims.x, half_dims.y);

                let to_a = a - *center;
                let to_b = b - *center;
                let to_c = c - *center;
                let to_d = dd - *center;

                let a = *center + dir.rotate(to_a);
                let b = *center + dir.rotate(to_b);
                let c = *center + dir.rotate(to_c);
                let dd = *center + dir.rotate(to_d);

                d.draw_line(a.x as i32, a.y as i32, b.x as i32, b.y as i32, color);
                d.draw_line(b.x as i32, b.y as i32, c.x as i32, c.y as i32, color);
                d.draw_line(c.x as i32, c.y as i32, dd.x as i32, dd.y as i32, color);
                d.draw_line(dd.x as i32, dd.y as i32, a.x as i32, a.y as i32, color);
            }
            RenderCommand::ColoredSquare { pos, color } => {
                d.draw_rectangle(
                    pos.x as i32,
                    pos.y as i32,
                    LINE_THICKNESS,
                    LINE_THICKNESS,
                    *color,
                );
            }
            RenderCommand::Text {
                pos,
                text,
                size,
                color,
            } => {
                d.draw_text(text, pos.x as i32, pos.y as i32, *size, *color);
            }
            RenderCommand::Line { start, end, color } => {
                d.draw_line_v(
                    Vector2::new(start.x, start.y),
                    Vector2::new(end.x, end.y),
                    *color,
                );
            }
            RenderCommand::Circle { pos, radius, color } => {
                d.draw_circle(pos.x as i32, pos.y as i32, *radius, *color);
            }
            RenderCommand::SolidRectangle { pos, dims, color } => {
                d.draw_rectangle(
                    pos.x as i32,
                    pos.y as i32,
                    dims.x as i32,
                    dims.y as i32,
                    *color,
                );
            }
        }
    }
}
