use hecs::World;
use rand::Rng;
use raylib::RaylibHandle;

use crate::{
    components::InputControlled,
    entity_archetypes::{spawn_car, spawn_wall},
    state::{GameMode, LevelDesignState, State},
    utils::vec2_lerp,
};

pub fn transition_to_level_design(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {
    {
        state.game_mode = GameMode::LevelDesign;
        state.level_design_state = Box::new(LevelDesignState::new());

        // get a random point on the starting line
        let starting_line = &state.level_design_state.map.starting_line;
        let mut rng = rand::thread_rng();
        // random number between 0 and 1
        let random = rng.gen::<f32>();
        let starting_position = vec2_lerp(starting_line.a, starting_line.b, random);

        let starting_direction = &state.level_design_state.map.starting_direction;
        let car_entity = spawn_car(ecs, state, starting_position, *starting_direction);
        // add the InputControlled component to car_entity
        let _ = ecs.insert_one(car_entity, InputControlled);
    }

    // spawn wall for every wall
    {
        for wall in state.level_design_state.map.walls.clone().into_iter() {
            spawn_wall(ecs, state, wall.a, wall.b);
        }
    }
}
