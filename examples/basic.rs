extern crate ears;

use ears::*;

fn main() {
    // Create a new Sound.
    let mut snd = Sound::new("examples/assets/test.ogg").unwrap();

    // Play the Sound
    snd.play();

    // Wait until the end of the sound
    while snd.is_playing() {}
}
