#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Paused,
    Playing,
}

pub struct GameSpeed(pub u64);
