use std::path::Path;
extern crate rustsourcebundler;
use rustsourcebundler::Bundler;

fn main() {
    let mut bundler: Bundler = Bundler::new(Path::new("src/main.rs"),
                                            Path::new("output/singlefile.rs"));
    bundler.crate_name("platinum-rift");
    bundler.run();
}
