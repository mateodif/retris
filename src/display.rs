use macroquad::prelude::*;
use tokio::runtime::Runtime;

lazy_static! {
    static ref BLACK_TEXTURE: Texture2D = {
        Runtime::new().unwrap().block_on(async {
            load_texture("textures/black.png").await.unwrap()
        })
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayAction {
    Empty,
    MustClean,
    Persist,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayBlock {
    pub color: Option<Color>,
    pub texture: Texture2D,
    pub action: DisplayAction
}

impl DisplayBlock {
    pub fn new(color: Color, action: DisplayAction, texture: Texture2D) -> Self {
        Self { color: Some(color), action, texture }
    }
}

impl Default for DisplayBlock {
    fn default() -> DisplayBlock {
        DisplayBlock::new(BLACK, DisplayAction::Empty, BLACK_TEXTURE.to_owned())
    }
}
