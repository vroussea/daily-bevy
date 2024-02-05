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
Today's focus is going to be on the [clear_color](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/window/clear_color.rs) example from Bevy's repository.

 This time we get two different systems to play with, and a new resource inserted into the App.
 The resource is straightforward, it's a ClearColor that is used to set a color between frames.
 (Read awwsmm post in today's example to see difference between init and insert resource, short reply is : init get or create a resource, insert overwrite, in both case it's a singleton)

 The first system gets a mut Commands variable as parameter that is used to queue changes to the world (spoiler: here it's used to add a camera as soon as possible).
 So surely enough we spawn a new command that will create an entity called a 2DCamera using a Bundle that helps us set the entity Components.
 The big difference we get from previous systems is that this system is going to run at Startup on the Main scheduler.

 Note from the doc:
 `Since each command requires exclusive access to the `World`, all queued commands are automatically applied in sequence when the [`apply_deferred`] system runs.`
 Should check in later examples when that system is run.

 The second system is set on Update and is partly similar to what we got yesterday. It also retrieves the ClearColor (inside a ResMut so a mutable resource) that was set as a resource and change it when space is pressed (here i changed the example to change the color only while space is pressed). It is not an event so we don't need a reader, we can just set it.