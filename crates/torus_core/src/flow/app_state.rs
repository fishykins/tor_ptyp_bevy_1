use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, StageLabel)]
pub enum AppState {
    Starting,
    InGame,
    Loading,
}

impl AppState {
    /// Adds [`AppState`] to the [`AppBuilder`] and all its associated stages.
    pub fn insert(app: &mut AppBuilder, state: AppState) {
        app.insert_resource(State::new(state))
            .add_system_set_to_stage(CoreStage::First, State::<AppState>::get_driver())
            .add_system_set_to_stage(CoreStage::PreUpdate, State::<AppState>::get_driver())
            .add_system_set_to_stage(CoreStage::Update, State::<AppState>::get_driver())
            .add_system_set_to_stage(CoreStage::PostUpdate, State::<AppState>::get_driver())
            .add_system_set_to_stage(CoreStage::Last, State::<AppState>::get_driver());
    }
}
