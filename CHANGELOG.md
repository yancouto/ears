# v0.8.0
  - Added ability to get and set offset of Sound and Music (at sample/frame level accuracy)
  - Prevent a panic that could occur when reading a file that had non-UTF-8 tags
  - Deprecated audio_tags::empty()

# v0.7.0
  - Added ability to get duration of Sound and Music

# v0.6.0
  - Added basic support for creating and attaching reverb effects to both Sound and Music
  - Added ability to set the Air Absorption of a Sound
  - Added ability to set the velocity of a Sound
  - Added ability to set the velocity of the listener

# v0.5.1
  - Improved documentation
  - Added CI badges to Cargo.toml

# v0.5.0
  - Made building on Windows easier [#12](https://github.com/jhasse/ears/pull/12)
  - Improved error handling [#13](https://github.com/jhasse/ears/pull/13)
  - Added ARM support [#16](https://github.com/jhasse/ears/pull/16)
  - Fixed bug that could cause looped music to stop unexpectedly [#18](https://github.com/jhasse/ears/pull/18)
  - Added direct channel support [#20](https://github.com/jhasse/ears/pull/20)
  - Added changelog!
