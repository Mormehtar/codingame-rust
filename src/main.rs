extern crate coding_game;
use coding_game::local_io::ReadBuffer;
use std::io;
use coding_game::utils::init_board;

fn main() {
    let stdin = io::stdin();
    let mut buffer = Box::new(stdin.lock());
    let mut reader = ReadBuffer::new(buffer);

    let mut board = init_board(&mut reader);
}
