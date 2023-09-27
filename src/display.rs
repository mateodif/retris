use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayAction { // tengo que cambiar este nombre
    Empty,
    MustClean,
    Persist,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DisplayBlock {
    pub color: Option<Color>,
    pub action: DisplayAction
}

impl DisplayBlock {
    pub fn new(color: Color, action: DisplayAction) -> Self {
        Self { color: Some(color), action }
    }
}

impl Default for DisplayBlock {
    fn default() -> DisplayBlock {
        DisplayBlock::new(BLACK, DisplayAction::Empty)
    }
}
