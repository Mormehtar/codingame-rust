use std::io;
use std::io::prelude::*;

//mod local_io;
//mod types;

//pub fn main_fn(input: Box<io::BufRead>, output: Box<Write>, comment: Box<Write>) {
//    let mut reader = local_io::ReadBuffer::new(input);
//
//    let first_line = reader.read_line();
//    let player_count: usize = first_line[0].parse().unwarp(); // the amount of players (2 to 4)
//    let my_id: usize = first_line[1].parse().unwarp(); // my player ID (0, 1, 2 or 3)
//    let zone_count: usize = first_line[2].parse().unwarp(); // the amount of zones on the map
//    let link_count: usize = first_line[3].parse().unwarp(); // the amount of links between all zones
//}

//pub fn init_map(reader: &mut local_io::ReadBuffer) -> types::Map {}