use glam::Vec2;
use hecs::Entity;
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    audio_playing::AudioCommandBuffer,
    input_processing::{LevelDesignInputs, TitleInputs},
    message_stream::ExpiringMessages,
    physics_engine::PhysicsEngine,
    render_commands::RenderCommandBuffer,
};

pub const FRAMES_PER_SECOND: u32 = 120;

#[derive(Clone, Copy)]
pub enum GameMode {
    Title,
    JoiningServer,
    Lobby,
    RacePreStart,
    Playing,
    RaceDone,
    LevelDesign,
    TestLevel,
}

pub struct State {
    pub fps: f32,
    pub running: bool,
    pub time_since_last_update: f32,
    pub t: f32,
    pub rng: StdRng,

    pub game_mode: GameMode,

    pub level_design_state: Box<LevelDesignState>,

    pub expiring_messages: ExpiringMessages,

    pub audio_command_buffer: AudioCommandBuffer,
    pub render_command_buffer: RenderCommandBuffer,

    pub title_inputs: TitleInputs,
    pub level_design_inputs: LevelDesignInputs,
    pub mouse_screen_pos: Vec2,

    // pub collision_events: Vec<Collision>,
    pub level: u32,
    pub level_change_delay: u32,

    pub physics: PhysicsEngine,

    pub deletion_events: Vec<DeletionEvent>,
}

impl State {
    pub fn new() -> Self {
        let rng: StdRng = StdRng::from_entropy();

        let level_design_state = Box::new(LevelDesignState {
            mode: LevelDesignMode::PlaceLines,
            map: Box::new(Map::new()),
        });

        Self {
            fps: 0.0,
            running: true,
            time_since_last_update: 0.0,

            rng,

            t: 0.0,

            game_mode: GameMode::Title,

            level_design_state,

            expiring_messages: ExpiringMessages::new(),

            audio_command_buffer: AudioCommandBuffer::new(),
            render_command_buffer: RenderCommandBuffer::new(),

            title_inputs: TitleInputs::new(),
            level_design_inputs: LevelDesignInputs::new(),
            mouse_screen_pos: Vec2::ZERO,

            // collision_events: Vec::new(),
            level: 1,
            level_change_delay: 0,

            physics: PhysicsEngine::new(),

            deletion_events: Vec::new(),
        }
    }
}

pub enum DeletionEvent {
    Entity { entity: Entity },
    Physics { entity: Entity },
}

pub enum LevelDesignMode {
    PlaceLines,
    Drive,
    Pause,
}

impl ToString for LevelDesignMode {
    fn to_string(&self) -> String {
        match self {
            LevelDesignMode::PlaceLines => "PlaceLines".to_string(),
            LevelDesignMode::Drive => "Drive".to_string(),
            LevelDesignMode::Pause => "Pause".to_string(),
        }
    }
}

pub struct Line {
    pub a: Vec2,
    pub b: Vec2,
}

pub struct Map {
    pub starting_line: Line,
    pub checkpoints: Vec<Line>,
    pub finish_line: Line,
}

impl Map {
    pub fn new() -> Self {
        Self {
            starting_line: Line {
                a: Vec2::new(117.0, 19.0),
                b: Vec2::new(117.0, 55.0),
            },
            checkpoints: vec![
                // 33,87 to 86, 87
                Line {
                    a: Vec2::new(33.0, 87.0),
                    b: Vec2::new(86.0, 87.0),
                },
                // starting line, but down 100 pixels
                Line {
                    a: Vec2::new(117.0, 119.0),
                    b: Vec2::new(117.0, 155.0),
                },
                // 136,86 to 209, 86
                Line {
                    a: Vec2::new(136.0, 86.0),
                    b: Vec2::new(209.0, 86.0),
                },
            ],
            finish_line: Line {
                a: Vec2::new(130.0, 19.0),
                b: Vec2::new(130.0, 55.0),
            },
        }
    }
}

pub struct LevelDesignState {
    pub mode: LevelDesignMode,
    pub map: Box<Map>,
}
