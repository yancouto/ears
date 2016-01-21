# ears [![Build Status](https://travis-ci.org/jhasse/ears.svg?branch=master)](https://travis-ci.org/jhasse/ears) [![License](https://img.shields.io/badge/license-MIT-blue.svg)]()


__ears__ is a simple library to play sounds and music in Rust and is build on the top of OpenAL and
libsndfile.

* Provides an access to the OpenAL spatialization functionality in a simple way.
* Accepts a lot of audio formats, thanks to libsndfile.

You need to install OpenAL and libsndfile on your system:

## Linux

```
sudo apt install libopenal-dev libsndfile1-dev
```

## Mac

```
brew install openal-soft libsndfile
```

## Examples

```
cargo run --example basic
cargo run --example many_sounds
cargo run --example music
cargo run --example record
cargo run --example simple_player
```

## Functionality

__ears__ provides two ways to play audio files:

* The Sound class, which represents light sounds who can share a buffer of samples with another
  Sound.
* The Music class, which represents bigger sound and can't share sample buffers.
