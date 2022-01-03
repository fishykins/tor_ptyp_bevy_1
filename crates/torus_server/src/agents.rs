use bevy::prelude::*;
use rand::Rng;
use torus_core::{
    agents::{biped::Biped, Agent},
    control::Controller,
    network::Local,
    physics::Body, WORLD_SIZE_X, WORLD_SIZE_Y,
};

use crate::events::ClientEvent;

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub agent: Agent,
    pub transform: Transform,
    pub body: Body<Local>,
    pub controller: Controller,
    pub biped: Biped,
}

impl PlayerBundle {
    pub fn new(handle: u32) -> Self {
        let mut rng = rand::thread_rng();
        let pos_x = rng.gen_range(0.0..WORLD_SIZE_X) as f32;
        let pos_y = rng.gen_range(0.0..WORLD_SIZE_Y) as f32;

        Self {
            agent: Agent::new(handle),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 1.0)),
            body: Body::<Local>::from_translation(Vec2::new(pos_x, pos_y)),
            controller: Controller::default(),
            biped: Biped::default(),
        }
    }
}

pub fn spawn_players(mut commands: Commands, mut spawn_events: EventReader<ClientEvent>) {
    for event in spawn_events.iter() {
        match event {
            ClientEvent::Spawn(handle) => {
                let player = PlayerBundle::new(*handle);
                let pos = player.transform.translation;
                commands.spawn_bundle(player);
                bevy::log::info!("Spawned player [{}] at {}", handle, pos);
            }
        }
    }
}