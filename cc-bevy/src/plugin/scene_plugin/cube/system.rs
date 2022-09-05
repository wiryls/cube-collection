use iyes_loopless::{condition::ConditionSystemSet, prelude::*};

use super::world::World;

mod state;
mod translate;

pub fn calculate(set: ConditionSet) -> ConditionSystemSet {
    set.run_if_resource_exists::<World>()
        .with_system(state::state_system)
}

pub fn execute(set: ConditionSet) -> ConditionSystemSet {
    set.run_if_resource_exists::<World>()
        .with_system(translate::position_system)
}
