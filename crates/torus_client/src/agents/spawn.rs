use bevy::prelude::*;
use torus_core::{
    agents::{biped::Biped, Agent},
    control::Controller,
    network::{
        data::ClientId,
        {Local, Remote},
    },
    physics::Body,
};

use crate::{agents::Player, TextureAssets};

use super::AgentEvent;

pub fn spawn_agents(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<AgentEvent>,
    texture_assets: Res<TextureAssets>,
    client_id: Res<ClientId>,
) {
    for event in events.iter() {
        match event {
            AgentEvent::Spawn(handle, data) => {
                bevy::log::info!("Spawning player {} at {}", handle, data.position);
                let mut transform =
                    Transform::from_translation(Vec3::new(data.position.x, data.position.y, 1.0));
                transform.scale = Vec3::new(0.5, 0.5, 1.0);
                let mut entity = commands.spawn_bundle(SpriteBundle {
                    material: materials.add(texture_assets.doddy.clone().into()),
                    transform,
                    ..Default::default()
                });
                entity
                    .insert(Agent::new(*handle))
                    .insert(Body::<Remote>::from_translation(data.position))
                    .insert(Biped::default());
                if client_id.is_equal(*handle) {
                    entity
                        .insert(Player::default())
                        .insert(Controller::default())
                        .insert(Body::<Local>::from_translation(data.position));
                }
            }
        }
    }
}