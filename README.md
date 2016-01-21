# ears [![Build Status](https://travis-ci.org/jhasse/ears.svg?branch=master)](https://travis-ci.org/jhasse/ears)


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

## Usage

Run the basic example using:

```
cargo run --example basic
```

## Functionality

__ears__ provides two ways to play audio files:

* The Sound class, which represents light sounds who can share a buffer of samples with another
  Sound.
* The Music class, which represents bigger sound and can't share sample buffers.
