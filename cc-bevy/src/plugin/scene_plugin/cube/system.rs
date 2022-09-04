use super::world::World;
use iyes_loopless::{condition::ConditionSystemSet, prelude::*};

mod state;
mod translate;

pub fn calculate(set: ConditionSet) -> ConditionSystemSet {
    set.run_if_resource_exists::<World>()
        .with_system(state::system)
}

pub fn execute(set: ConditionSet) -> ConditionSystemSet {
    set.run_if_resource_exists::<World>()
        .with_system(translate::system)
}
