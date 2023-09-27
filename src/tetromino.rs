use macroquad::{prelude::*, rand};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use tokio::runtime::Runtime;
use std::collections::HashMap;

use crate::display::DisplayAction;
use crate::state::{Board, Position};

lazy_static! {
    pub static ref TETROMINO_TEXTURES: HashMap<TetrominoType, Texture2D> = {
        let mut textures = HashMap::new();
        Runtime::new().unwrap().block_on(async {
            textures.insert(TetrominoType::I, load_texture("textures/skyblue.png").await.unwrap());
            textures.insert(TetrominoType::T, load_texture("textures/purple.png").await.unwrap());
            textures.insert(TetrominoType::O, load_texture("textures/orange.png").await.unwrap());
            textures.insert(TetrominoType::J, load_texture("textures/blue.png").await.unwrap());
            textures.insert(TetrominoType::L, load_texture("textures/orange.png").await.unwrap());
            textures.insert(TetrominoType::S, load_texture("textures/green.png").await.unwrap());
            textures.insert(TetrominoType::Z, load_texture("textures/red.png").await.unwrap());
        });
        textures
    };
}

#[derive(EnumIter, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

type Shape = [[i32; 2]; 4];

#[derive(Debug, Clone)]
pub struct Tetromino <'a>{
    pub shape: Shape,
    pub color: Color,
    pub texture: &'a Texture2D,
    pub t_type: TetrominoType,
}

impl Tetromino <'_> {
    pub fn new(t_type: TetrominoType) -> Self {
        let shape = match t_type {
            TetrominoType::I => [[-1,  0], [ 0,  0], [ 1, 0], [2, 0]],
            TetrominoType::T => [[ 0, -1], [-1,  0], [ 0, 0], [1, 0]],
            TetrominoType::O => [[ 0, -1], [ 1, -1], [ 0, 0], [1, 0]],
            TetrominoType::J => [[-1, -1], [-1,  0], [ 0, 0], [1, 0]],
            TetrominoType::L => [[ 1, -1], [-1,  0], [ 0, 0], [1, 0]],
            TetrominoType::S => [[ 0, -1], [ 1, -1], [-1, 0], [0, 0]],
            TetrominoType::Z => [[-1, -1], [ 0, -1], [ 0, 0], [1, 0]],
        };

        let texture = TETROMINO_TEXTURES.get(&t_type).unwrap();
        let color = WHITE;

        Tetromino { shape, color, texture, t_type }
    }

    pub fn random() -> Self {
        let max = TetrominoType::iter().count();
        let num = rand::gen_range(0, max);
        Self::new(TetrominoType::iter().get(num).unwrap_or(TetrominoType::I))
    }

    pub fn fits(&self, board: &Board, position: Position) -> bool {
        let (cx, cy) = position;

        for [x, y] in self.shape {
            let new_x = (x + cx) as usize;
            let new_y = (y + cy) as usize;

            if new_y < board.len() && new_x < board[0].len() {
                if board[new_y][new_x].action != DisplayAction::Empty {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn fits_after_rotate(&self, board: &Board, position: Position) -> bool {
        let (cx, cy) = position;

        for [x, y] in self.rotated_shape() {
            let new_x = (x + cx) as usize;
            let new_y = (y + cy) as usize;

            if new_y < board.len() && new_x < board[0].len() {
                if board[new_y][new_x].action != DisplayAction::Empty {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }


    pub fn rotated_shape(&self) -> Shape {
       self.shape.map(|coord| {
            let [x, y] = coord;
            [(- y), x]
        })
    }

    pub fn rotate(&mut self) {
        self.shape = self.rotated_shape();
    }
}
