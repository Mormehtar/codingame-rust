mod lib;

fn main() {
    let m = lib::types::Player::new(1);
    println!("Hello player {}!", m.id);
}
