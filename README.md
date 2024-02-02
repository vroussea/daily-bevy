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
Today's focus is going to be on the [hello world](https://github.com/bevyengine/bevy/blob/main/examples/hello_world.rs) example from Bevy's repository.

So to say, today's goal was simply to decipher basic app creation and running. That can be simplified by first adding a resource that let's Bevy translate type into literals (reflection), then add a few schedulers (Main and FixedUpdateLoop), add an AppExit listener in Main and add one simple system (a function called by a scheduler) before running the App.