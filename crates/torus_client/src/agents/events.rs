// use std::collections::HashMap;
// use std::ops::Deref;

use bevy::prelude::*;
// use torus_core::flow::GameTick;
// use torus_core::network::data::ClientId;
use torus_core::network::messages::AgentUpdateMessage;
//use torus_core::{agents::Agent, network::Remote, physics::Body};

#[derive(Clone)]
pub enum AgentEvent {
    Update(AgentUpdateMessage),
}

pub fn handle_events(
    mut server_events: EventReader<AgentEvent>,
    // mut commands: Commands,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // mut query: Query<(&mut Body<Remote>, &Agent)>,
    // client_id: Res<ClientId>,
    // game_tick: Res<GameTick>,
    // assets: Res<AssetServer>,
) {
    for event in server_events.iter() {
        match event {
            AgentEvent::Update(_agent_update) => {
                // let mut to_spawn: HashMap<u32, Vec2> = HashMap::new();
                // for (mut remote, agent) in query.iter_mut() {
                //     if let Some(index) = agent_update
                //         .agents
                //         .iter()
                //         .position(|&update| update.0 == agent.owner)
                //     {
                //         let (_id, translation) = agent_update.agents.remove(index);
                //         remote.translation = Vec2::new(translation.x, translation.y);
                //         remote.set_last_update(game_tick.frame());
                //     }
                // }

                // // These should be unspawned players.
                // for (id, translation) in agent_update.agents.drain(..) {
                //     if !to_spawn.contains_key(&id) {
                //         to_spawn.insert(id, translation);
                //     }
                // }

                // for (id, translation) in to_spawn {
                //     //super::spawn_agent(commands, materials, client_id, id, translation, assets);
                // }
            }
        }
    }
}
