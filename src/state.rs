use macroquad::prelude::*;
use crate::tetromino::Tetromino;
use crate::display::{DisplayAction, DisplayBlock};

pub const ROW_COUNT: usize = 20;
pub const COLUMN_COUNT: usize = 10;

pub type Board = [DisplayBlock; ROW_COUNT * COLUMN_COUNT];
pub type Position = (i32, i32);

pub enum Direction {
    Left,
    Right,
    Down
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub board: Board,
    pub position: Position,
    pub current_piece: Tetromino,
    pub screen_size: (usize, usize),
}

impl Default for State {
    fn default() -> State {
        State {
            board: [DisplayBlock::default(); 10 * 20],
            position: (5, 1),
            current_piece: Tetromino::random(),
            screen_size: (500, 900),
        }
    }
}

impl State {
    fn cell_width(&self) -> usize {
        let (width, _) = self.screen_size;
        width / COLUMN_COUNT
    }

    fn cell_height(&self) -> usize {
        let (_, height) = self.screen_size;
        height / ROW_COUNT
    }

    pub fn move_current_piece(&mut self, direction: Direction) {
        let (x, y) = self.position;
        let new_position = match direction {
            Direction::Left =>  (x - 1, y    ),
            Direction::Right => (x + 1, y    ),
            Direction::Down =>  (x,     y + 1),
        };
        if self.current_piece.fits(&self.board, new_position) {
            self.position = new_position;
        };
    }

    pub fn manipulate_current_piece(&mut self, action: DisplayAction) {
        let (cx, cy) = self.position;
        if self.current_piece.fits(&self.board, self.position) {
            for [x, y] in self.current_piece.shape {
                let new_x = (x + cx) as usize;
                let new_y = (y + cy) as usize;
                self.board[new_y * COLUMN_COUNT + new_x] = DisplayBlock {
                    color: Some(self.current_piece.color),
                    action
                };
            };
        };
    }

    pub fn can_piece_move_down(&self) -> bool {
        self.current_piece.fits(&self.board, (self.position.0, self.position.1 + 1))
    }

    fn remove_full_lines(&mut self) {
        let mut new_board = [DisplayBlock::default(); ROW_COUNT * COLUMN_COUNT];
        let mut next_row_to_fill = ROW_COUNT;

        let is_persisted = |cell: &DisplayBlock| cell.action == DisplayAction::Persist;

        for row in self.board.chunks_exact(COLUMN_COUNT).rev() {
            if !row.iter().all(is_persisted) {
                next_row_to_fill -= 1;
                let target_slice = &mut new_board[next_row_to_fill * COLUMN_COUNT..(next_row_to_fill + 1) * COLUMN_COUNT];
                target_slice.copy_from_slice(row);
            }
        }

        let empty_row = [DisplayBlock::default(); COLUMN_COUNT];
        for row in new_board.chunks_exact_mut(COLUMN_COUNT).take(next_row_to_fill) {
            row.copy_from_slice(&empty_row);
        }

        self.board.copy_from_slice(&new_board);
    }

    pub fn clean_board(&mut self) {
        for cell in &mut self.board {
            if cell.action == DisplayAction::MustClean {
                *cell = DisplayBlock::default();
            }
        }
        self.remove_full_lines();
    }

    pub fn draw_board(&self) {
        let cell_height = self.cell_height() as f32;
        let cell_width = self.cell_width() as f32;

        for (index, cell) in self.board.iter().enumerate() {
            let i = index / COLUMN_COUNT;
            let j = index % COLUMN_COUNT;
            let (x, y) = (j as f32 * cell_width, i as f32 * cell_height);

            match cell.action {
                DisplayAction::Persist | DisplayAction::MustClean => {
                    draw_rectangle(x, y, cell_width, cell_height, cell.color.unwrap_or(BLACK))
                },
                _ => draw_rectangle_lines(x, y, cell_width, cell_height, 1.0, WHITE)
            }
        }
    }

    pub fn lock_piece(&mut self) {
        self.manipulate_current_piece(DisplayAction::Persist);
        self.position = (5, 1);
        self.current_piece = Tetromino::random();
    }
}
