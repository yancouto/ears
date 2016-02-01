extern crate ears;

use ears::{Music, AudioController};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut music = Music::new("res/music.ogg").unwrap();
    music.play();
    while music.is_playing() {
        sleep(Duration::from_millis(1000));
    }
}
