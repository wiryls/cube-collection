use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::plugin::ShapePlugin;

mod cube;
mod input;
mod scene_loading;
mod scene_running;
mod view;

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        const MARK: CoreStage = CoreStage::Update;

        app.add_plugin(ShapePlugin)
            .add_loopless_state(SceneState::default())
            .add_stage_before(MARK, CustomStage::INPUT, SystemStage::parallel())
            .add_stage_before(MARK, CustomStage::CHECK, SystemStage::parallel())
            .add_stage_before(MARK, CustomStage::WORLD, SystemStage::parallel());

        view::setup(app, CustomStage::INPUT);
        input::setup(app, CustomStage::INPUT);
        scene_loading::setup(app);
        scene_running::setup(app, CustomStage::CHECK, CustomStage::WORLD);
    }
}

struct CustomStage;
impl CustomStage {
    pub const INPUT: &'static str = "custom_input";
    pub const CHECK: &'static str = "custom_check";
    pub const WORLD: &'static str = "custom_world";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SceneState {
    Loading,
    Running,
}

impl Default for SceneState {
    fn default() -> Self {
        Self::Loading
    }
}
