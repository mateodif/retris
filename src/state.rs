use macroquad::prelude::*;
use crate::tetromino::Tetromino;
use crate::display::{DisplayAction, DisplayBlock};

pub type Board = [[DisplayBlock; 10]; 20];
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
            board: [[DisplayBlock::default(); 10]; 20],
            position: (5, 1),
            current_piece: Tetromino::random(),
            screen_size: (500, 900),
        }
    }
}

impl State {
    fn cell_width(&self) -> usize {
        let (width, _) = self.screen_size;
        width / self.board[0].len()
    }

    fn cell_height(&self) -> usize {
        let (_, height) = self.screen_size;
        height / self.board.len()
    }

    pub fn move_current_piece(&mut self, direction: Direction) {
        let (x, y) = self.position;
        let new_position = match direction {
            Direction::Left =>  (x - 1, y    ),
            Direction::Right => (x + 1, y    ),
            Direction::Down =>  (x,     y + 1),
        };
        if self.current_piece.fits(self.board, new_position) {
            self.position = new_position;
        };
    }

    pub fn manipulate_current_piece(&mut self, action: DisplayAction) {
        let (cx, cy) = self.position;
        if self.current_piece.fits(self.board, self.position) {
            for [x, y] in self.current_piece.shape {
                let new_x = x + cx;
                let new_y = y + cy;
                self.board[new_y as usize][new_x as usize] = DisplayBlock::new(self.current_piece.color, action);
            };
        };
    }

    pub fn can_piece_move_down(&self) -> bool {
        self.current_piece.fits(self.board, (self.position.0, self.position.1 + 1))
    }

    fn remove_full_lines(&mut self) {
        let mut non_zero_index = 0;

        for current_index in 0..self.board.len() {
            if !self.board[current_index].iter().all(|cell| cell.action == DisplayAction::Persist) {
                if current_index != non_zero_index {
                    self.board.swap(current_index, non_zero_index);
                }
                non_zero_index += 1;
            }
        }
        for i in non_zero_index..self.board.len() {
            self.board[i] = [DisplayBlock::default(); 10];
        }

        if non_zero_index > 0 {
            self.board.rotate_left(non_zero_index);
        }
    }

    pub fn clean_board(&mut self) {
        for row in &mut self.board {
            for cell in row.iter_mut() {
                if cell.action == DisplayAction::MustClean {
                    *cell = DisplayBlock::default();
                }
            }
        }
        self.remove_full_lines();
    }

    pub fn draw_board(&self) {
        let cell_height = self.cell_height() as f32;
        let cell_width = self.cell_width() as f32;

        for (i, row) in self.board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let (x, y) = (j as f32 * cell_width, i as f32 * cell_height);

                match cell.action {
                    DisplayAction::Persist | DisplayAction::MustClean => {
                        draw_rectangle(x, y, cell_width, cell_height, cell.color.unwrap_or(BLACK))
                    },
                    _ => draw_rectangle_lines(x, y, cell_width, cell_height, 1.0, WHITE)
                }
            }
        }
    }

    pub fn lock_piece(&mut self) {
        self.manipulate_current_piece(DisplayAction::Persist);
        self.position = (5, 1);
        self.current_piece = Tetromino::random();
    }
}
