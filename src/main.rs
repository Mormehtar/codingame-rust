extern crate coding_game;
use coding_game::local_io::ReadBuffer;
use std::io;
use coding_game::utils::init_board;

fn main() {
    let mut buffer = Box::new(std::io::stdin());
    let mut reader = ReadBuffer::new(buffer);

    let mut board = init_board(&mut reader);
}
