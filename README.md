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
Today's focus is going to be on the [`button` example](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/ui/button.rs) from Bevy's repository.

This example focuses on nexted entities (parent and child), on Ui nodes and also on a new resource called WinitSettings::desktop_app() that let's us set the windows to update only every 60 seconds instead of as fast as possible or when the user interact with it, as opposed as a game that would require constant update.

We also have a new filter ( `Changed<Interaction>` ) on the Query that let's us know when the entities returned by the query has been interacted with (so the `Interaction` component has been created or updated).

The last noteworthy piece of new information we got today is the `text_query.get_mut(children[0]).unwrap();` part. It uses get_mut on the query to retrieve a child entity from it.