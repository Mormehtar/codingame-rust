extern crate coding_game;
use coding_game::local_io::ReadBuffer;
use std::io;
use coding_game::utils;
use coding_game::strategy::base_strategy::BaseStrategy;
use coding_game::strategy::Strategy;

fn main() {
    let stdin = io::stdin();
    let mut buffer = Box::new(stdin.lock());
    let mut reader = ReadBuffer::new(buffer);
    let stdout = io::stdout();

    utils::main::<BaseStrategy, io::StdoutLock>(reader, &mut stdout.lock());
}
