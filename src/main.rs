#![allow(dead_code)]
#![allow(unused_variables)]

use macroquad::prelude::*;
use crate::display::DisplayAction;
use crate::state::Direction;

mod state;
mod tetromino;
mod display;

const TIME_INTERVAL: f64 = 1.2;
const MIN_FRAME_TIME: f32 = 1. / 160.;

#[macroquad::main("Retris")]
async fn main() {
    let mut state = state::State::default();
    let mut start_time = get_time();
    let mut pause = false;

    loop {
        let (width, height) = state.screen_size;
        request_new_screen_size(width as f32, height as f32);

        state.clean_board();
        clear_background(BLACK);
        let has_to_move = get_time() > (start_time + TIME_INTERVAL);

        if pause {
            draw_text("PAUSE", ((width / 2) - 98) as f32, (height / 2) as f32, 90.0, RED);
        }

        if !state.can_piece_move_down() && has_to_move && !pause {
            state.lock_piece();
        }

        if has_to_move && !pause {
            state.move_current_piece(Direction::Down);
            start_time = get_time();
        }

        if is_key_pressed(KeyCode::Space) && !pause {
            while state.can_piece_move_down() {
                state.move_current_piece(Direction::Down);
            }
            state.lock_piece();
        }

        if is_key_pressed(KeyCode::Up) && !pause {
            if state.current_piece.fits_after_rotate(&state.board, state.position) {
                state.current_piece.rotate();
            }
        }

        if is_key_pressed(KeyCode::Right) && !pause {
            state.move_current_piece(Direction::Right);
        }

        if is_key_pressed(KeyCode::Left) && !pause {
            state.move_current_piece(Direction::Left);
        }

        if is_key_pressed(KeyCode::Down) && !pause {
            state.move_current_piece(Direction::Down);
        }

        if is_key_pressed(KeyCode::P) {
            pause = !pause;
        }

        state.manipulate_current_piece(DisplayAction::MustClean);

        state.draw_board();

        let frame_time = get_frame_time();
        if frame_time < MIN_FRAME_TIME {
            let time_to_sleep = (MIN_FRAME_TIME - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        next_frame().await
    }
}
