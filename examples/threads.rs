extern crate ears;
use ears::{Sound, AudioController};
use std::thread;

fn main() {
    let mut handles = Vec::new();
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut snd = Sound::new("res/shot.wav").unwrap();
            snd.play();
            while snd.is_playing() {}
        });
        handles.push(handle);
    }

    for h in handles {
        match h.join() {
            Ok(_)  => println!("Thread exited successfully"),
            Err(_) => println!("Thread died"),
        };
    }
}
