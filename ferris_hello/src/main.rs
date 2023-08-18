#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
use ferris_says::say;

use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let text = String::from("hello rust, here I come at you with my zigness");
    let width = text.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(text.as_bytes(), width, &mut writer).unwrap();
}
