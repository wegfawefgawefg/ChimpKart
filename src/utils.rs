use glam::Vec2;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn vec2_lerp(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t))
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * (180.0 / std::f32::consts::PI)
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (std::f32::consts::PI / 180.0)
}

pub fn rotate_a_around_b(a: Vec2, b: Vec2, angle_in_radians: f32) -> Vec2 {
    let c = angle_in_radians.cos();
    let s = angle_in_radians.sin();
    let x = a.x - b.x;
    let y = a.y - b.y;
    Vec2::new(b.x + x * c - y * s, b.y + x * s + y * c)
}
