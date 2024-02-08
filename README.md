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
Let's continue working onto Camera2dBundle.
Focus was more into trying stuff onto the Camera and using a new Entity: the `Text`.
In this example, I mapped the keyboard arrows onto camera movements, the mouse wheel onto camera zoom (since camera is orthogonal I'm scaling x and y) and left +  right mouse clicks onto camera rotation.

The text that is printed is using the Text entity that is created using the Text2DBundle. In that bundle the only component that we change is the Text one by adding a font (loaded from assets folder) and the selections (the strings that will be visible).

Same as yesterday, we are using a tuple when spawning the entities that let's use add a label onto them that we then use in our systems.

The systems use the Transform components from the 2dCamera to do the translations/rotations.