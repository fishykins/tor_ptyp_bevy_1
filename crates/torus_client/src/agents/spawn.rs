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

use crate::agents::Player;

pub fn spawn_agent(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    client_id: ClientId,
    id: u32,
    translation: Vec2,
    assets: Res<AssetServer>,
) {
    bevy::log::info!("Spawning player {}", id);
    //let sprite_handle = assets.g
    let mut transform = Transform::from_translation(Vec3::new(translation.x, translation.y, 0.0));
    transform.scale = Vec3::new(0.5, 0.5, 1.0);
    let mut entity = commands.spawn_bundle(SpriteBundle {
        material: materials.add(sprite_handle.into()),
        transform,
        ..Default::default()
    });
    entity
        .insert(Agent::new(id))
        .insert(Body::<Remote>::from_translation(translation))
        .insert(Biped::default());
    if client_id.is_equal(id) {
        entity
            .insert(Player::default())
            .insert(Controller::default())
            .insert(Body::<Local>::from_translation(translation));
    }
}
