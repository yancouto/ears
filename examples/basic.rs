extern crate ears;

use ears::{AudioController, Sound};

fn play_file(file: &str) {
    // Create a new Sound.
    let mut snd = Sound::new(file).unwrap();

    // Play the Sound
    snd.play();

    // Wait until the end of the sound
    while snd.is_playing() {}
}

fn main() {
    play_file("res/shots2.ogg");
    play_file("res/explosion.wav");
}
