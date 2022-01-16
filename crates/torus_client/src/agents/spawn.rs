use std::ops::Deref;

use bevy::prelude::*;
use torus_core::{
    agents::{Agent, Biped, Controller, AgentEvent},
    network::{
        data::ClientId,
        {Local, Remote},
    },
    physics::Rigidbody,
};

use crate::agents::Player;
use crate::assets::Images;

pub fn spawn_agents(
    mut commands: Commands,
    mut events: EventReader<AgentEvent>,
    texture_assets: Res<Images>,
    client_id: Res<ClientId>,
    agents: Query<&Agent>,
) {
    if !client_id.deref().allocated() {
        return;
    }
    for event in events.iter() {
        match event {
            AgentEvent::Spawn(handle, data) => {
                if agents.iter().any(|agent| agent.owner == *handle) {
                    continue;
                }

                bevy::log::info!("Spawning player {} at {}", handle, data.position);
                let mut transform =
                    Transform::from_translation(Vec3::new(data.position.x, data.position.y, 1.0));
                transform.scale = Vec3::new(0.5, 0.5, 1.0);
                let mut entity = commands.spawn_bundle(SpriteBundle {
                    texture: texture_assets.player.clone(),
                    transform,
                    ..Default::default()
                });
                entity
                    .insert(Agent::new(*handle))
                    .insert(Rigidbody::<Remote>::from_translation(data.position))
                    .insert(Biped::default());
                if client_id.is_equal(*handle) {
                    entity
                        .insert(Player::default())
                        .insert(Controller::default())
                        .insert(Rigidbody::<Local>::from_translation(data.position));
                }
            }
        }
    }
}
