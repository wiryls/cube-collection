mod adaption;
mod marker;
mod state;
mod style;
mod translate;

pub mod bundle;

pub mod system {
    use super::*;
    pub use adaption::self_adaption_system as self_adaption;
    pub use state::state_system as state;
    pub use translate::{
        position_system as position, realpha_system as realpha, recolor_system as recolor,
        reshape_system as reshape,
    };
}

pub mod component {
    use super::*;
    pub use marker::Earthbound;
    pub use translate::{TranslateAlpha, TranslateColor, TranslatePosition, TranslateShape};
}
