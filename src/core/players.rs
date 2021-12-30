use bevy::prelude::*;

use super::{components::Controller, WORLD_SIZE_X, WORLD_SIZE_Y};

pub const PLAYER_SPEED: f32 = 200.0;

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
            update_player_local.system().label("update_player_local").before("update_player_transforms"),
        );
    }
}

/// Updates local gbody positions based on controller input. For a client, this is only speculative- the server has final say.
fn update_player_local(
    time: Res<Time>,
    mut query: Query<(&Controller, &mut GBodyLocal)>,
) {
    for (controller, mut gbody_local) in query.iter_mut() {
        if controller.forward != 0.0 || controller.lateral != 0.0 {
            let step_move = Vec3::new(controller.lateral, controller.forward, 0.0).normalize()
                * time.delta_seconds()
                * PLAYER_SPEED;
            gbody_local.translation += step_move;
        }

        if gbody_local.translation.x > WORLD_SIZE_X {
            gbody_local.translation.x = 0.1;
        } else if gbody_local.translation.x < 0.0 {
            gbody_local.translation.x = WORLD_SIZE_X;
        }

        if gbody_local.translation.y > WORLD_SIZE_Y {
            gbody_local.translation.y = 0.1;
        } else if gbody_local.translation.y < 0.0 {
            gbody_local.translation.y = WORLD_SIZE_Y;
        }
    }
}

/// A local representation of a goon's body and where we think it is. No one else cares about this.
#[derive(Debug, Default, Clone)]
pub struct GBodyLocal {
    pub translation: Vec3,
}

impl GBodyLocal {
    pub fn from_translation(translation: Vec3) -> Self {
        GBodyLocal { translation }
    }
}

/// This is the authoritative representation of the goon's body, and represents exactly where the server thinks it is.
#[derive(Debug, Default, Clone)]
pub struct GBodyRemote {
    pub tick: u64,
    pub translation: Vec3,
}