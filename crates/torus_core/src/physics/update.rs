use crate::physics::Rigidbody;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::{Collider, RigidbodyType};

pub fn body_query_update<T>(
    rigidbodies: Query<(Entity, &Rigidbody<T>, &Children)>,
    colliders: Query<&Collider>,
) where
    T: 'static + Sync + Send + Default + Inspectable + Reflect,
{
    let mut dynamic_rigidbodies = Vec::new();
    let mut static_rigidbodies = Vec::new();

    for (entity, body, children) in &mut rigidbodies.iter() {
        match body.r#type {
            RigidbodyType::Dynamic => {
                let mut has_collider = false;
                for &child in children.iter() {
                    if let Ok(collider) = colliders.get(child) {
                        dynamic_rigidbodies.push((entity, body, collider));
                        has_collider = true;
                    }
                }
                if !has_collider {
                    static_rigidbodies.push((entity, body));
                }
            },
            _ => {
                static_rigidbodies.push((entity, body));
            },
        }
    }
}


/// A Bevy system to update all rigidbodies.
pub fn physics_update<T>(
    _time: Res<Time>,
    _commands: Commands,
) where
    T: 'static + Sync + Send + Default + Inspectable + Reflect,
{

}
