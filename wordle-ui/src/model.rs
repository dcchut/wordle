use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TileMode {
    Correct,
    Absent,
    Present,
}

impl TileMode {
    /// Toggles the mode of the current tile
    pub fn toggle(self) -> Self {
        match self {
            TileMode::Present => TileMode::Correct,
            TileMode::Correct => TileMode::Absent,
            TileMode::Absent => TileMode::Present,
        }
    }
}

impl std::fmt::Display for TileMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileMode::Correct => "correct",
                TileMode::Absent => "absent",
                TileMode::Present => "present",
            }
        )
    }
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct TileState {
    pub mode: TileMode,
    pub char: Option<char>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub tiles: Vec<TileState>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            tiles: vec![
                TileState {
                    mode: TileMode::Absent,
                    char: None,
                };
                5
            ],
        }
    }
}
