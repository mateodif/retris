#![allow(dead_code)]
#![allow(unused_variables)]

use macroquad::prelude::*;
use crate::display::DisplayAction;
use crate::state::Direction;

mod state;
mod tetromino;
mod display;

#[macroquad::main("Retris")]
async fn main() {
    let mut state = state::State::default();
    let mut start_time = get_time();

    loop {
        let (width, height) = state.screen_size;
        request_new_screen_size(width as f32, height as f32);
        state.clean_board();

        clear_background(BLACK);


        if get_time() > (start_time + 1.2) {
            state.move_current_piece(Direction::Down);
            start_time = get_time();
        }

        if !state.can_piece_move_down() {
            state.lock_piece();
        }

        if is_key_pressed(KeyCode::Space) {
            while state.can_piece_move_down() {
                state.move_current_piece(Direction::Down);
            }
        }

        if is_key_pressed(KeyCode::Up) {
            if state.current_piece.fits_after_rotate(&state.board, state.position) {
                state.current_piece.rotate();
            }
        }

        if is_key_pressed(KeyCode::Right) {
            state.move_current_piece(Direction::Right);
        }

        if is_key_pressed(KeyCode::Left) {
            state.move_current_piece(Direction::Left);
        }

        if is_key_pressed(KeyCode::Down) {
            state.move_current_piece(Direction::Down);
        }

        state.manipulate_current_piece(DisplayAction::MustClean);

        state.draw_board();

        let minimum_frame_time = 1. / 160.; // 160 FPS
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        next_frame().await
    }
}
