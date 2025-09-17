pub mod actions;
pub mod cards;
pub mod game_state;
pub mod models;
pub mod scoring;

// Re-export commonly used types
pub use game_state::{
    GamePhase, Location, SpecialCondition, SpecialConditionType, StateChange, TurnPhase,
};
pub use scoring::{Multiplier, PlayerScore};

/// 액션 실행 결과
#[derive(Debug, Clone)]
pub enum ActionResult {
    Success {
        next_action: Option<actions::Action>,
        state_change: StateChange,
    },
    Invalid {
        reason: String,
    },
}
