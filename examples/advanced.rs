extern crate ears;

use ears::{ReverbEffect, ReverbPreset, Sound, AudioController};
use std::time::Duration;
use std::thread::sleep;

// Demonstrates setting up a ReverbEffect and playing multiple
// connected Sounds with different positions and various other settings.
fn main() {
    let reverb_properties = ReverbPreset::Forest.properties();
    let reverb_effect = ReverbEffect::preset(reverb_properties).ok();

    // stereo ambience
    let mut wind = Sound::new("res/wind.ogg").unwrap();
    wind.set_volume(0.35);
    wind.connect(&reverb_effect);
    wind.play();

    // mono positioned ambience
    let mut water = Sound::new("res/water.ogg").unwrap();
    water.connect(&reverb_effect);
    water.set_air_absorption_factor(1.0);
    water.set_position([-1.0, -0.4, 4.0]);
    water.play();

    sleep(Duration::from_millis(3000));

    // mono moving artillery shot
    let mut sound = Sound::new("res/artillery.ogg").unwrap();
    sound.connect(&reverb_effect);
    sound.set_air_absorption_factor(1.0);
    sound.set_position([3.0, 3.0, 3.0]);
    sound.play();

    for i in 0..600 {
        let z = i as f32 / 200.0;

        sound.set_position([3.0 -z, (3.0 - z).max(0.0), 3.0 - z]);
        sleep(Duration::from_millis(1));
    }

    // direct channel stereo car exploding
    let mut sound = Sound::new("res/explosion.wav").unwrap();
    sound.set_direct_channel(true);
    sound.play();

    sleep(Duration::from_millis(750));

    // mono human yelling
    let mut sound = Sound::new("res/yell.ogg").unwrap();
    sound.connect(&reverb_effect);
    sound.set_reference_distance(50.0);
    sound.set_air_absorption_factor(5.0);
    sound.set_position([100.0, 0.0, -50.0]);
    sound.play();

    sleep(Duration::from_millis(600));

    // mono distant sniper shot
    let mut sound = Sound::new("res/sniper.ogg").unwrap();
    sound.connect(&reverb_effect);
    sound.set_reference_distance(1500.0);
    sound.set_pitch(1.1);
    sound.set_air_absorption_factor(10.0);
    sound.set_position([-200.0, 75.0, 1500.0]);
    sound.play();

    sleep(Duration::from_millis(200));

    // mono distant gunshots
    let mut sound = Sound::new("res/shots2.ogg").unwrap();
    sound.connect(&reverb_effect);
    sound.set_reference_distance(250.0);
    sound.set_pitch(0.8);
    sound.set_air_absorption_factor(5.0);
    sound.set_position([-100.0, 0.0, 330.0]);
    sound.play();

    sleep(Duration::from_millis(1000));

    // mono nearby gunshots
    let mut sound = Sound::new("res/shots2.ogg").unwrap();
    sound.connect(&reverb_effect);
    sound.set_reference_distance(100.0);
    sound.set_air_absorption_factor(5.0);
    sound.set_position([100.0, 0.0, -50.0]);
    sound.play();

    sleep(Duration::from_millis(1200));

    // mono distant sniper shot
    let mut sound = Sound::new("res/sniper.ogg").unwrap();
    sound.connect(&reverb_effect);
    sound.set_reference_distance(1500.0);
    sound.set_pitch(0.9);
    sound.set_air_absorption_factor(10.0);
    sound.set_position([-200.0, 75.0, 1500.0]);
    sound.play();

    // fade out
    for i in 0..3000 {
        let v = 1.0 - (i as f32 / 3000.0);

        wind.set_volume(v);
        water.set_volume(v);
        sleep(Duration::from_millis(1));
    }
}
