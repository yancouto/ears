# ears [![Build Status](https://travis-ci.org/nickbrowne/ears.svg?branch=master)](https://travis-ci.org/nickbrowne/ears) [![Build status](https://ci.appveyor.com/api/projects/status/0dhp10u9y2ivrieo/branch/master?svg=true)](https://ci.appveyor.com/project/nickbrowne/ears/branch/master) [![](http://meritbadge.herokuapp.com/ears)](https://crates.io/crates/ears)

Ears aims to be a convenient and easy to understand Rust interface over OpenAL.

It's designed first and foremost for game development, giving you easy access to
complex functionality like HRTF, spatialization, and environmental effects with
almost no configuration required.

**[View documentation](https://docs.rs/ears/)**

Supports a wide variety of audio formats, including:

* Ogg Vorbis
* Microsoft WAV
* RAW PCM
* Lossless FLAC
* AIFF

For a full list please see the documentation for libsndfile here: http://www.mega-nerd.com/libsndfile/

## Before you start

You need to install OpenAL and libsndfile on your system.

#### Linux (Debian and Ubuntu):

```
sudo apt install libopenal-dev libsndfile1-dev
```

#### Linux (Fedora):

```
sudo dnf install openal-soft-devel libsndfile-devel
```

#### Mac:

```
brew install openal-soft libsndfile
```

#### Windows:

Install [MSYS2](http://www.msys2.org/) according to the instructions. Be sure to
use the default installation folder (i.e. `C:\msys32` or `C:\msys64`), otherwise
compiling won't work. Then, run the following in the MSYS2 shell:

```
pacman -S mingw-w64-x86_64-libsndfile mingw-w64-x86_64-openal
```

## Usage

Include `ears` in your `Cargo.toml` dependencies.

```toml
[dependencies]
ears = "0.8.0"
```

Playing a sound effect while simultaneously streaming music off disk is as simple as it gets.

```rust
extern crate ears;
use ears::{Music, Sound, AudioController};

fn main() {
    let mut music = Music::new("your-music.ogg").unwrap();
    music.play();

    let mut sound = Sound::new("your-sound-effect.wav").unwrap();
    sound.play();

    while music.is_playing() || sound.is_playing() {};
}
```

## Running examples

```
cargo run --example basic
cargo run --example advanced
cargo run --example music
cargo run --example record
cargo run --example simple_player
cargo run --example threads
cargo run --example direct_channel
```
