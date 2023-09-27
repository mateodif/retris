#![allow(dead_code)]
#![allow(unused_variables)]

use macroquad::{prelude::*, rand};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const SCREEN_WIDTH: usize = 500;
const SCREEN_HEIGHT: usize = 900;

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

pub enum Direction {
    Left,
    Right,
    Down
}

type Shape = [[i32; 2]; 4];

#[derive(Debug, Clone, Copy)]
pub struct Tetromino {
    shape: Shape,
    color: Color,
    t_type: TetrominoType,
}

impl Tetromino {
    pub fn new(t_type: TetrominoType) -> Self {
        let (shape, color) = match t_type {
            TetrominoType::I => (
                [[-1,  0], [ 0,  0], [ 1, 0], [2, 0]],
                SKYBLUE
            ),
            TetrominoType::T => (
                [[ 0, -1], [-1,  0], [ 0, 0], [1, 0]],
                PURPLE
            ),
            TetrominoType::O => (
                [[ 0, -1], [ 1, -1], [ 0, 0], [1, 0]],
                YELLOW
            ),
            TetrominoType::J => (
                [[-1, -1], [-1,  0], [ 0, 0], [1, 0]],
                BLUE
            ),
            TetrominoType::L => (
                [[ 1, -1], [-1,  0], [ 0, 0], [1, 0]],
                ORANGE
            ),
            TetrominoType::S => (
                [[ 0, -1], [ 1, -1], [-1, 0], [0, 0]],
                GREEN
            ),
            TetrominoType::Z => (
                [[-1, -1], [ 0, -1], [ 0, 0], [1, 0]],
                RED
            ),
        };

        Tetromino { shape, color, t_type }
    }

    pub fn random() -> Self {
        let max = TetrominoType::iter().count();
        let num = rand::gen_range(0, max);
        Self::new(TetrominoType::iter().get(num).unwrap_or(TetrominoType::I))
    }

    pub fn fits(&self, board: Board, position: Position) -> bool {
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

    pub fn fits_after_rotate(&self, board: Board, position: Position) -> bool {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayAction { // tengo que cambiar este nombre
    Empty,
    MustClean,
    Persist,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DisplayBlock {
    color: Option<Color>,
    action: DisplayAction
}

impl DisplayBlock {
    pub fn new(color: Color, action: DisplayAction) -> Self {
        Self {
            color: Some(color),
            action
        }
    }
}

impl Default for DisplayBlock {
    fn default() -> DisplayBlock {
        DisplayBlock::new(BLACK, DisplayAction::Empty)
    }
}

type Board = [[DisplayBlock; 10]; 20];
type Position = (i32, i32);

#[derive(Debug, Clone, Copy)]
pub struct Field {
    board: Board,
    position: Position,
    piece: Tetromino,
}

impl Default for Field {
    fn default() -> Field {
        Field {
            board: [[DisplayBlock::default(); 10]; 20],
            position: (5, 1),
            piece: Tetromino::random()
        }
    }
}

impl Field {
    fn cell_width(&self) -> usize {
        SCREEN_WIDTH / self.board[0].len()
    }

    fn cell_height(&self) -> usize {
        SCREEN_HEIGHT / self.board.len()
    }

    pub fn move_current_piece(&mut self, direction: Direction) {
        let (x, y) = self.position;
        let new_position = match direction {
            Direction::Left =>  (x - 1, y    ),
            Direction::Right => (x + 1, y    ),
            Direction::Down =>  (x,     y + 1),
        };
        if self.piece.fits(self.board, new_position) {
            self.position = new_position;
        };
    }

    pub fn manipulate_current_piece(&mut self, action: DisplayAction) {
        let (cx, cy) = self.position;
        if self.piece.fits(self.board, self.position) {
            for [x, y] in self.piece.shape {
                let new_x = x + cx;
                let new_y = y + cy;
                self.board[new_y as usize][new_x as usize] = DisplayBlock::new(self.piece.color, action);
            };
        };
    }

    fn can_piece_move_down(&self) -> bool {
        self.piece.fits(self.board, (self.position.0, self.position.1 + 1))
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
        self.piece = Tetromino::random();
    }
}

#[macroquad::main("Retris")]
async fn main() {
    let mut field = Field::default();
    let mut start_time = get_time();

    loop {
        request_new_screen_size(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);
        field.clean_board();

        clear_background(BLACK);


        if get_time() > (start_time + 1.2) {
            field.move_current_piece(Direction::Down);
            start_time = get_time();
        }

        if !field.can_piece_move_down() {
            field.lock_piece();
        }

        if is_key_pressed(KeyCode::Space) {
            while field.can_piece_move_down() {
                field.move_current_piece(Direction::Down);
            }
        }

        if is_key_pressed(KeyCode::Up) {
            if field.piece.fits_after_rotate(field.board, field.position) {
                field.piece.rotate();
            }
        }

        if is_key_pressed(KeyCode::Right) {
            field.move_current_piece(Direction::Right);
        }

        if is_key_pressed(KeyCode::Left) {
            field.move_current_piece(Direction::Left);
        }

        if is_key_pressed(KeyCode::Down) {
            field.move_current_piece(Direction::Down);
        }

        field.manipulate_current_piece(DisplayAction::MustClean);

        field.draw_board();

        let minimum_frame_time = 1. / 160.; // 160 FPS
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        next_frame().await
    }
}
