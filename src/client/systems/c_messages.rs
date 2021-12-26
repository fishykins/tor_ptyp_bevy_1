use std::collections::HashMap;

use bevy::{prelude::*, log};
use bevy_networking_turbulence::NetworkResource;

use crate::{core::components::Agent, network::{ClientMessage, GameStateMessage}};
use super::ServerIds;

pub(crate) fn handle_messages(
    mut commands: Commands,
    mut net: ResMut<NetworkResource>,
    mut server_ids: ResMut<ServerIds>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut agents: Query<(Entity, &mut Transform), With<Agent>>,
) {
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(_client_message) = channels.recv::<ClientMessage>() {
            log::error!("ClientMessage received on [{}]", handle);
        }

        // it is possible that many state updates came at the same time - spawn once
        let mut to_spawn: HashMap<u32, (u32, Vec3)> = HashMap::new();

        while let Some(mut state_message) = channels.recv::<GameStateMessage>() {
            let message_frame = state_message.frame;
            log::debug!(
                "GameStateMessage received on [{}]: {:?}",
                handle,
                state_message
            );

            // update all agents
            for (entity, mut transform) in agents.iter_mut() {
                let server_id_entry = server_ids.get_mut(&entity.id()).unwrap();
                let (server_id, update_frame) = *server_id_entry;

                if let Some(index) = state_message
                    .agents
                    .iter()
                    .position(|&update| update.0 == server_id)
                {
                    let (_id, translation) = state_message.agents.remove(index);

                    if update_frame > message_frame {
                        continue;
                    }
                    server_id_entry.1 = message_frame;
                    transform.translation = translation;
                } else {
                    // TODO: despawn disconnected agents
                }
            }
            // create new agents
            for (id, translation) in state_message.agents.drain(..) {
                if let Some((frame, _translation)) = to_spawn.get(&id) {
                    if *frame > message_frame {
                        continue;
                    }
                };
                to_spawn.insert(id, (message_frame, translation));
            }
        }

        for (id, (frame, translation)) in to_spawn.iter() {
            log::info!("Spawning {} @{}", id, frame);
            let entity = commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(
                        Color::rgb(0.8 - (*id as f32 / 5.0), 0.2, 0.2 + (*id as f32 / 5.0)).into(),
                    ),
                    transform: Transform::from_translation(*translation),
                    sprite: Sprite::new(Vec2::new(30.0, 30.0)),
                    ..Default::default()
                })
                .insert(Agent { controller: *id })
                .id();
            server_ids.insert(entity.id(), (*id, *frame));
        }
    }
}