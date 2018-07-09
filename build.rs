extern crate bundler;
use std::fs::File;
use std::io::prelude::*;

fn write_down(path: &str, data: String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn main() {
    write_down("output/singlefile.rs", bundler::bundle(".")).unwrap();
}