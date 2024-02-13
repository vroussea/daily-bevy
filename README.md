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
Today, we'll be digging into the [`3d_shapes` example](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/3d/3d_shapes.rs) found in the Bevy repo.

Today's focus is about CameraBundle again, but this time 3d one.

Small difference in today's default plugins by changing the image one to use nearest setup instead of the default linear (to make images more pixelated) by using the DefaultPlugins set method.

The setup system's goal is to create different shapes, a light and a plan to see those 3d shapes shadows casted from the light.
We see three new Resources, all are assets and they let us use Meshes, Images, and Standard Materials.
We use them to spawn all the shapes + light while also spawning the 3dCamera.

For the material used by the shapes we use a function called uv_debug_texture that set 4bytes colors (RGBA) in an array while rotating the color on each new line to make a colorful effect.

As for the the second system called rotate it simply rotates all the shapes at the same speed using the Time resource.