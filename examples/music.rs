extern crate ears;

use ears::{AudioController, Music};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut music = Music::new("res/music.ogg").unwrap();
    music.play();
    while music.is_playing() {
        sleep(Duration::from_millis(1000));
    }
}
