mod local_io;
mod utils;
mod types;


fn main() {
    let m = types::player::Player::new(1);
    println!("Hello player {}!", m.id);
}
