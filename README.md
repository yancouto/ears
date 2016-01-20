# ears [![Build Status](https://travis-ci.org/jeremyletang/ears.png?branch=master)](https://travis-ci.org/jeremyletang/ears)


__ears__ is a simple library to play Sounds and Musics in Rust.

__ears__ is build on the top of OpenAL and libsndfile.

* Provides an access to the OpenAL spatialization functionality in a simple way.
* Accepts a lot of audio formats, thanks to libsndfile.

# A simple example

```Rust
extern crate ears;
use ears::Sound;

fn main() {
	// Create a new Sound.
	let snd = Sound::new("path/to/my/sound.ogg").unwrap();

	// Play the Sound
	snd.play();

	// Wait until the end of the sound
	while snd.is_playing() {}
}
```

# Functionalities

__ears__ provides two ways to play audio files.

* The Sound class, which represents light sounds who can share a buffer of samples with another Sound.
* The Music class, which is a bigger sound and who can't share sample buffer.

# Use ears

Like previously said, __ears__ requires OpenAL and libsndfile. You need to install these two
libraries on your system. You can run the basic example using:

```
cargo run --example basic
```
