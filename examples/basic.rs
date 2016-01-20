extern crate ears;

use ears::{Sound, AudioController};

fn play_file(file: &str) {
    // Create a new Sound.
    let mut snd = Sound::new(file).unwrap();

    // Play the Sound
    snd.play();

    // Wait until the end of the sound
    while snd.is_playing() {}
}

fn main() {
    play_file("res/shot.wav");
    play_file("res/explosion.ogg");
}
