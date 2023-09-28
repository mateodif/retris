use macroquad::{prelude::*, rand};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::display::DisplayAction;
use crate::state::{Board, Position};


#[derive(EnumIter, Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct Tetromino {
    pub shape: Shape,
    pub color: Color,
    pub t_type: TetrominoType,
}

impl Tetromino {
    pub fn new(t_type: TetrominoType) -> Self {
        let (shape, color) = match t_type {
            TetrominoType::I => ([[-1,  0], [ 0,  0], [ 1, 0], [2, 0]], SKYBLUE),
            TetrominoType::T => ([[ 0, -1], [-1,  0], [ 0, 0], [1, 0]], PURPLE),
            TetrominoType::O => ([[ 0, -1], [ 1, -1], [ 0, 0], [1, 0]], YELLOW),
            TetrominoType::J => ([[-1, -1], [-1,  0], [ 0, 0], [1, 0]], BLUE),
            TetrominoType::L => ([[ 1, -1], [-1,  0], [ 0, 0], [1, 0]], ORANGE),
            TetrominoType::S => ([[ 0, -1], [ 1, -1], [-1, 0], [0, 0]], GREEN),
            TetrominoType::Z => ([[-1, -1], [ 0, -1], [ 0, 0], [1, 0]], RED),
        };

        Tetromino { shape, color, t_type }
    }

    pub fn random() -> Self {
        let max = TetrominoType::iter().count();
        let num = rand::gen_range(0, max);
        Self::new(TetrominoType::iter().get(num).unwrap_or(TetrominoType::I))
    }

    pub fn fits(&self, board: &Board, position: Position) -> bool {
        self.check_fit(board, position, &self.shape)
    }

    pub fn fits_after_rotate(&self, board: &Board, position: Position) -> bool {
        self.check_fit(board, position, &self.rotated_shape())
    }

    fn check_fit(&self, board: &Board, position: Position, shape: &Shape) -> bool {
        let (cx, cy) = position;

        shape.iter().all(|[x, y]| {
            let new_x = (x + cx) as usize;
            let new_y = (y + cy) as usize;

            if let Some(row) = board.get(new_y) {
                if let Some(cell) = row.get(new_x) {
                    return cell.action == DisplayAction::Empty;
                }
            }
            false
        })
    }

    pub fn rotated_shape(self) -> Shape {
       self.shape.map(|coord| {
            let [x, y] = coord;
            [(- y), x]
        })
    }

    pub fn rotate(&mut self) {
        self.shape = self.rotated_shape();
    }
}
