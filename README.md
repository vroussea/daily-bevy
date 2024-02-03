# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the first entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/vroussea/daily-bevy/branches).
Original ones can be found [here](https://github.com/awwsmm/daily-bevy/branches).

### Disclaimer
The goal of this repository will be to follow the tracks of [awwsmm's repo](https://github.com/awwsmm/daily-bevy/blob/master/README.md) that focuses on working on one example from Bevy repo per day.2

## Hello, Bevy!

Today is the first day of Daily Bevy.

## Today's example
Today's focus is going to be on the [keyboard input](https://github.com/bevyengine/bevy/blob/main/examples/input/keyboard_input.rs) example from Bevy's repository.

Same as yesterday, the default plugins are added to the app. The difference is in the argument given to the system. This time we use a `Res<Input<KeyCode>>`, Res (or ResMut if need mutability) allowing us to access a Resource (a type in the world inserted as a singleton as the documentation says). The Res holds an Input that can be anything "press-able" that in that case is a KeyCode (keyboard input).

Today's example only introduce a few new types.

Got to read a bit the doc about Worlds, InputPlugin and Ticks as well.

PS: awwsmm seems to have had some struggle with the inputs, but since I tried, for the first time, today's example on Windows I seem to having been exampted of any trouble whatsoever. But looks more like I used the right Bevy version tag from the get go.