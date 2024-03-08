use glam::Vec2;
use hecs::{Entity, World};
use nalgebra::vector;
use rapier2d::{
    math::Point,
    prelude::{ActiveEvents, ColliderBuilder, InteractionGroups, RigidBodyBuilder},
};
use raylib::prelude::Color;

use crate::{
    components::{
        Ball, BallEater, Block, Bouncy, CTransform, Car, HasRigidBody, Health, InputControlled,
        OwnedBy, Paddle, Physics, Player, PositionManaged, Shape, StrongBlock, VelocityManaged,
        Wall,
    },
    physics_engine::p2m,
    state::State,
    utils::radians_to_degrees,
    DIMS,
};

pub fn spawn_walls(ecs: &mut World, state: &mut State) {
    println!("Spawning walls");
    let wall_color = Color::RAYWHITE;
    let wall_thickness = 20.0;
    // top wall
    let pos = Vec2::new(0.0, -wall_thickness + 1.0);
    let shape = Shape {
        dims: Vec2 {
            x: DIMS.x as f32,
            y: wall_thickness,
        },
    };
    let top_wall = ecs.spawn((
        CTransform {
            pos,
            dir: Vec2::new(0.0, 0.0),
        },
        shape,
        Wall,
        HasRigidBody,
    ));
    let center = pos + shape.dims / 2.0;
    let top_wall_collider =
        ColliderBuilder::cuboid(p2m(shape.dims.x) / 2.0, p2m(shape.dims.y) / 2.0)
            .restitution(1.0)
            .friction(0.0)
            .build();
    let top_wall_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![p2m(center.x), p2m(center.y)])
        .can_sleep(false)
        .build();
    let top_wall_body_handle = state.physics.rigid_body_set.insert(top_wall_rigid_body);
    state.physics.collider_set.insert_with_parent(
        top_wall_collider,
        top_wall_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(top_wall, top_wall_body_handle);

    // bottom wall
    let pos = Vec2::new(0.0, DIMS.y as f32 - 1.0);
    let shape = Shape {
        dims: Vec2 {
            x: DIMS.x as f32,
            y: wall_thickness,
        },
    };
    let bottom_wall = ecs.spawn((
        CTransform {
            pos,
            dir: Vec2::new(0.0, 0.0),
        },
        shape,
        Wall,
        HasRigidBody,
        BallEater,
    ));
    let center = pos + shape.dims / 2.0;
    let bottom_wall_collider =
        ColliderBuilder::cuboid(p2m(shape.dims.x) / 2.0, p2m(shape.dims.y) / 2.0)
            .restitution(1.0)
            .friction(0.0)
            .build();
    let bottom_wall_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![p2m(center.x), p2m(center.y)])
        .can_sleep(false)
        .build();
    let bottom_wall_body_handle = state.physics.rigid_body_set.insert(bottom_wall_rigid_body);
    state.physics.collider_set.insert_with_parent(
        bottom_wall_collider,
        bottom_wall_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(bottom_wall, bottom_wall_body_handle);

    // left wall
    let pos = Vec2::new(-wall_thickness + 1.0, 0.0);
    let shape = Shape {
        dims: Vec2 {
            x: wall_thickness,
            y: DIMS.y as f32,
        },
    };
    let left_wall = ecs.spawn((
        CTransform {
            pos,
            dir: Vec2::new(0.0, 0.0),
        },
        shape,
        Wall,
        HasRigidBody,
    ));
    let center = pos + shape.dims / 2.0;
    let left_wall_collider =
        ColliderBuilder::cuboid(p2m(shape.dims.x) / 2.0, p2m(shape.dims.y) / 2.0)
            .restitution(1.0)
            .friction(0.0)
            .build();
    let left_wall_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![p2m(center.x), p2m(center.y)])
        .can_sleep(false)
        .build();
    let left_wall_body_handle = state.physics.rigid_body_set.insert(left_wall_rigid_body);
    state.physics.collider_set.insert_with_parent(
        left_wall_collider,
        left_wall_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(left_wall, left_wall_body_handle);

    // right wall
    let pos = Vec2::new(DIMS.x as f32 - 1.0, 0.0);
    let shape = Shape {
        dims: Vec2 {
            x: wall_thickness,
            y: DIMS.y as f32,
        },
    };
    let right_wall = ecs.spawn((
        CTransform {
            pos,
            dir: Vec2::new(0.0, 0.0),
        },
        shape,
        Wall,
        HasRigidBody,
        // PositionManaged,
    ));
    let center = pos + shape.dims / 2.0;
    let right_wall_collider =
        ColliderBuilder::cuboid(p2m(shape.dims.x) / 2.0, p2m(shape.dims.y) / 2.0)
            .restitution(1.0)
            .friction(0.0)
            .build();
    let right_wall_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![p2m(center.x), p2m(center.y)])
        .can_sleep(false)
        .build();
    let right_wall_body_handle = state.physics.rigid_body_set.insert(right_wall_rigid_body);
    state.physics.collider_set.insert_with_parent(
        right_wall_collider,
        right_wall_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(right_wall, right_wall_body_handle);

    // middle wall
    // let right_wall = ecs.spawn((
    //     CTransform {
    //         pos: Vec2::new(DIMS.x as f32 / 2.0, DIMS.y as f32 / 2.0),
    //         rot: Vec2::new(0.0, 0.0),
    //     },
    //     Shape {
    //         dims: Vec2 {
    //             x: 5.0,
    //             y: DIMS.y as f32 / 10.0,
    //         },
    //     },
    //     Wall,
    //     HasRigidBody,
    //     // PositionManaged,
    // ));
    // let middle_wall_collider =
    //     ColliderBuilder::cuboid(p2m(wall_thickness) / 2.0, p2m(DIMS.y as f32) / 2.0)
    //         .restitution(1.0)
    //         .friction(0.0)
    //         .build();
    // let middle_wall_rigid_body = RigidBodyBuilder::fixed()
    //     .translation(vector![
    //         p2m(DIMS.x as f32 + wall_thickness / 2.0),
    //         p2m(DIMS.y as f32 / 2.0)
    //     ])
    //     .can_sleep(false)
    //     .build();
    // let right_wall_body_handle = state.physics.rigid_body_set.insert(right_wall_rigid_body);
    // state.physics.collider_set.insert_with_parent(
    //     right_wall_collider,
    //     right_wall_body_handle,
    //     &mut state.physics.rigid_body_set,
    // );
    // state
    //     .physics
    //     .set_rigid_body_mapping(right_wall, right_wall_body_handle);
}

// NOTE: the 0 angle is to the right, so width is the x dimension, and height is the y dimension
//  so if a car is facing left or right, the long part is the x dimension
//  but if it's facing up or down, the long part is the y dimension
pub const CAR_SHAPE: Vec2 = Vec2::new(12.0, 8.0);
pub fn spawn_car(ecs: &mut World, state: &mut State, pos: Vec2, dir: Vec2) {
    let car_entity = ecs.spawn((
        Car { tires_dir: dir },
        CTransform { pos, dir },
        Physics {
            vel: Vec2::ZERO,
            rot_vel: 0.0,
        },
        Shape { dims: CAR_SHAPE },
        HasRigidBody,
    ));
    // print original angle
    let angle = dir.to_angle();
    println!("angle: {}", angle);
    println!("dir: {:?}", dir);

    let car_collider = ColliderBuilder::cuboid(p2m(CAR_SHAPE.x) / 2.0, p2m(CAR_SHAPE.y) / 2.0)
        .restitution(1.0)
        .friction(0.0)
        .mass(0.0001)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .collision_groups(InteractionGroups::new(0b0001.into(), 0b0001.into()))
        .build();
    let car_rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![p2m(pos.x), p2m(pos.y)])
        // .rotation(angle)
        .linvel(vector![p2m(0.0), p2m(0.0)])
        // .lock_rotations()
        .linear_damping(0.0)
        .angular_damping(0.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();
    let car_body_handle = state.physics.rigid_body_set.insert(car_rigid_body);
    state.physics.collider_set.insert_with_parent(
        car_collider,
        car_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(car_entity, car_body_handle);
}

pub fn spawn_wall(ecs: &mut World, state: &mut State, start: Vec2, end: Vec2) {
    let length = (end - start).length();
    let wall_entity = ecs.spawn((
        Wall,
        CTransform {
            pos: start,
            dir: (end - start).normalize(),
        },
        Shape {
            dims: Vec2::new(length, 0.0),
        },
        HasRigidBody,
    ));

    let wall_collider = ColliderBuilder::segment(
        Point::new(p2m(start.x), p2m(start.y)),
        Point::new(p2m(end.x), p2m(end.y)),
    )
    .active_events(ActiveEvents::COLLISION_EVENTS)
    .collision_groups(InteractionGroups::new(0b0001.into(), 0b0001.into()))
    .build();
    let wall_rigid_body = RigidBodyBuilder::fixed().build();
    let wall_body_handle = state.physics.rigid_body_set.insert(wall_rigid_body);
    state.physics.collider_set.insert_with_parent(
        wall_collider,
        wall_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(wall_entity, wall_body_handle);
}

pub const BALL_SHAPE: Vec2 = Vec2::new(4.0, 4.0);
pub fn spawn_ball(ecs: &mut World, state: &mut State, pos: Vec2, vel: Vec2, owner: Entity) {
    let ball_entity = ecs.spawn((
        Ball,
        CTransform {
            pos,
            dir: Vec2::new(0.0, 0.0),
        },
        Physics { vel, rot_vel: 0.0 },
        OwnedBy { owner },
        Shape { dims: BALL_SHAPE },
        Bouncy,
        HasRigidBody,
        VelocityManaged,
    ));
    // let ball_collider = ColliderBuilder::ball(p2m(8.0) / 2.0)
    let ball_collider = ColliderBuilder::cuboid(p2m(BALL_SHAPE.x) / 2.0, p2m(BALL_SHAPE.y) / 2.0)
        // let ball_collider = ColliderBuilder::ball(p2m(BALL_SHAPE.x) / 2.0)
        .restitution(1.0)
        .friction(0.0)
        .mass(0.0001)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .collision_groups(InteractionGroups::new(0b0001.into(), 0b0001.into()))
        .build();
    let ball_rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![p2m(pos.x), p2m(pos.y)])
        .linvel(vector![p2m(vel.x), p2m(vel.y)])
        .lock_rotations()
        .linear_damping(0.0)
        .angular_damping(0.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();
    let ball_body_handle = state.physics.rigid_body_set.insert(ball_rigid_body);
    state.physics.collider_set.insert_with_parent(
        ball_collider,
        ball_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(ball_entity, ball_body_handle);
}

pub fn spawn_block(
    ecs: &mut World,
    state: &mut State,
    pos: Vec2,
    shape: Vec2,
    color: Color,
    hp: u32,
    ball_unbreakable: bool,
) {
    let block_entity = ecs.spawn((
        CTransform {
            pos,
            dir: Vec2::new(0.0, 1.0),
        },
        Shape { dims: shape },
        Block { color },
        Health { hp },
        HasRigidBody,
    ));
    if ball_unbreakable {
        ecs.insert_one(block_entity, StrongBlock).unwrap();
    }

    let block_collider = ColliderBuilder::cuboid(p2m(shape.x) / 2.0, p2m(shape.y) / 2.0)
        .restitution(1.0)
        .friction(0.0)
        .collision_groups(InteractionGroups::new(0b0101.into(), 0b0101.into()))
        .build();
    let block_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![
            p2m(pos.x + shape.x / 2.0),
            p2m(pos.y + shape.y / 2.0)
        ])
        .ccd_enabled(true)
        .can_sleep(false)
        .build();

    let block_body_handle = state.physics.rigid_body_set.insert(block_rigid_body);
    state.physics.collider_set.insert_with_parent(
        block_collider,
        block_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(block_entity, block_body_handle);
}

pub fn spawn_paddle(
    ecs: &mut World,
    state: &mut State,
    pos: Vec2,
    shape: Vec2,
    color: Color,
) -> Entity {
    let paddle_entity = ecs.spawn((
        CTransform {
            pos,
            dir: Vec2::new(0.0, 0.0),
        },
        Physics {
            vel: Vec2::ZERO,
            rot_vel: 0.0,
        },
        InputControlled,
        Player,
        Paddle { size: 1 },
        Shape { dims: shape },
        HasRigidBody,
        PositionManaged,
    ));

    let paddle_collider = ColliderBuilder::cuboid(p2m(shape.x) / 2.0, p2m(shape.y) / 2.0)
        .restitution(1.0)
        .collision_groups(InteractionGroups::new(0b0011.into(), 0b0011.into()))
        .build();
    let paddle_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![
            p2m(pos.x + shape.x / 2.0),
            p2m(pos.y + shape.y / 2.0)
        ])
        // .ccd_enabled(true)
        .can_sleep(false)
        .build();

    let paddle_body_handle = state.physics.rigid_body_set.insert(paddle_rigid_body);
    state.physics.collider_set.insert_with_parent(
        paddle_collider,
        paddle_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(paddle_entity, paddle_body_handle);
    paddle_entity
}

// pub fn spawn_powerup(
//     ecs: &mut World,
//     state: &mut State,
//     pos: Vec2,
//     shape: Vec2,
//     power_up_type: PowerUpType,
// ) -> Entity {
//     let power_up_entity = ecs.spawn((
//         CTransform {
//             pos,
//             rot: Vec2::new(0.0, 0.0),
//         },
//         Physics {
//             vel: Vec2::new(0.0, -2.0),
//             rot_vel: 0.0,
//         },
//         Shape { dims: shape },
//         PowerUp { power_up_type },
//     ));

//     let paddle_collider = ColliderBuilder::cuboid(p2m(shape.x) / 2.0, p2m(shape.y) / 2.0)
//         .restitution(1.0)
//         .collision_groups(InteractionGroups::new(0b0011.into(), 0b0011.into()))
//         .build();
//     let paddle_rigid_body = RigidBodyBuilder::kinematic_position_based()
//         .translation(vector![
//             p2m(pos.x + shape.x / 2.0),
//             p2m(pos.y + shape.y / 2.0)
//         ])
//         // .ccd_enabled(true)
//         .can_sleep(false)
//         .build();

//     let paddle_body_handle = state.physics.rigid_body_set.insert(paddle_rigid_body);
//     state.physics.collider_set.insert_with_parent(
//         paddle_collider,
//         paddle_body_handle,
//         &mut state.physics.rigid_body_set,
//     );
//     state
//         .physics
//         .set_rigid_body_mapping(paddle_entity, paddle_body_handle);
//     paddle_entity
// }
