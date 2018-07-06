pub mod types;
pub mod local_io;
pub mod utils;

fn main() {
    let m = types::player::Player::new(1);
    println!("Hello player {}!", m.id);
}
