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

impl Default for DisplayBlock {
    fn default() -> DisplayBlock {
        DisplayBlock {
            color: Some(BLACK),
            action: DisplayAction::Empty
        }
    }
}
