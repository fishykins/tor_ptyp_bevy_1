use bevy::prelude::*;

use super::{components::Controller, maths, WORLD_SIZE_X, WORLD_SIZE_Y};

// ===============================================================
// ======================== CORE PLAYER ==========================
// ===============================================================

#[derive(Default)]
/// A plugin that handles basic universal network events.
pub struct CorePlayerPlugin;

impl Plugin for CorePlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_to_stage(
            CoreStage::Update,
            update_player_local
                .system()
                .label("update_player_local")
                .before("update_player_transforms"),
        )
        .add_system_to_stage(
            CoreStage::Update,
            wrap_around
                .system()
                .label("wrap_around")
                .after("update_player_transforms"),
        );
    }
}

/// Updates local gbody positions based on controller input. For a client, this is only speculative- the server has final say.
fn update_player_local(time: Res<Time>, mut query: Query<(&Controller, &mut GBodyLocal, &Biped)>) {
    for (controller, mut local, biped) in query.iter_mut() {
        if let Some(forward) = controller.target_direction {
            if (forward - local.body.direction).abs() > 0.01 {
                local.body.direction = maths::lerp_angle(
                    local.body.direction,
                    forward,
                    biped.turn_speed * time.delta_seconds(),
                );
            } else {
                local.body.direction = forward;
            }
        }
        let velocity = Vec3::new(controller.lateral, controller.forward, 0.0).normalize().length() * time.delta_seconds() * biped.speed;
        if velocity > 0.0 {
            let rotation = Quat::from_rotation_z(local.body.direction);
            let move_dir = rotation * velocity * Vec3::Y;
            local.body.translation += move_dir;
        }
    }
}

fn wrap_around(mut query: Query<&mut GBodyLocal>) {
    for mut local in query.iter_mut() {
        if local.body.translation.x > WORLD_SIZE_X {
            local.body.translation.x = 0.1;
        } else if local.body.translation.x < 0.0 {
            local.body.translation.x = WORLD_SIZE_X;
        }

        if local.body.translation.y > WORLD_SIZE_Y {
            local.body.translation.y = 0.1;
        } else if local.body.translation.y < 0.0 {
            local.body.translation.y = WORLD_SIZE_Y;
        }

        if local.body.direction > 2.0 * std::f32::consts::PI {
            local.body.direction -= 2.0 * std::f32::consts::PI;
        } else if local.body.direction < 0.0 {
            local.body.direction += 2.0 * std::f32::consts::PI;
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GBody {
    pub translation: Vec3,
    pub translation_previous: Vec3,
    pub target: Vec2,
    pub direction: f32,
    pub velocity: Vec2,
}

/// A local representation of a goon's body and where we think it is. No one else cares about this.
#[derive(Debug, Default, Clone)]
pub struct GBodyLocal {
    pub body: GBody,
}

impl GBodyLocal {
    pub fn from_translation(translation: Vec3) -> Self {
        GBodyLocal {
            body: GBody {
                translation,
                ..Default::default()
            },
        }
    }
}

/// This is the authoritative representation of the goon's body, and represents exactly where the server thinks it is.
#[derive(Debug, Default, Clone)]
pub struct GBodyRemote {
    pub tick: u64,
    pub body: GBody,
}

#[derive(Debug, Clone, Copy)]
pub struct Biped {
    pub speed: f32,
    pub run_multiplier: f32,
    pub acceleration: f32,
    pub stopping_distance: f32,
    pub turn_speed: f32,
    pub turn_limmit: f32,
}

impl Default for Biped {
    fn default() -> Self {
        Self {
            speed: 200.0,
            run_multiplier: 1.5,
            acceleration: 0.1,
            stopping_distance: 200.0,
            turn_speed: 4.0,
            turn_limmit: 0.75,
        }
    }
}
