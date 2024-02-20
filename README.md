# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the first entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/vroussea/daily-bevy/branches).
Original ones can be found [here](https://github.com/awwsmm/daily-bevy/branches).

### Disclaimer
The goal of this repository will be to follow the tracks of [awwsmm's repo](https://github.com/awwsmm/daily-bevy/blob/master/README.md) that focuses on working on one example from Bevy repo per day.2

## Today's example
Today's focus is going to be on the [`scene` example](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/scene/scene.rs) example from Bevy's repository.

This example is about scenes. A scene is somewhat like a snapshot of a world, by that it means that it holds all the resources/entities the world has. We then use this scene to create a copy of the world in a file will all entities/resources saved in a seralized [`RON`](https://github.com/ron-rs/ron) format. It looks like JSON but a bit more modern and rustlike. We can also load a scene file using the asset server loading method.

To create a scene you need to make sure the entities' components and the resources types that make the world are saved within the App (using .register_type() method) and that the components/resources also derive the Reflect trait that gives them dynamic serialization/deserialization/default (default can be manually implemented to allow choosing members of the structs to be ignored by implementing the FromWorld trait).
We then need to use the Reflect derive macro to add respectively the component and resource traits to also reflects their behaviors.

And that's it for today.