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
Today's focus will be on the [2d/text2d](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/2d/text2d.rs) example.

This one shpows us a lot of stuff about 2d text bundles and also adds a few twists using systems to rotate/translate/scale the text. Before showing us how to bound text inside a sprite and then how to use anchors to choose where the text should be located given a transform.

Awwsmm also checks a bit more onto the plugins to see how they are built, what the finish() and cleanup() methods do etc.

We also have a look at children entities.

I decided to change the scaling system to change the font size as to not make it so pixelated.