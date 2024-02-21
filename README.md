# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the first entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/vroussea/daily-bevy/branches).
Original ones can be found [here](https://github.com/awwsmm/daily-bevy/branches).

### Disclaimer
The goal of this repository will be to follow the tracks of [awwsmm's repo](https://github.com/awwsmm/daily-bevy/blob/master/README.md) that focuses on working on one example from Bevy repo per day.2

## Today's example
Today's focus is going to be on the [`reflection` example](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/reflection/reflection.rs) from Bevy's repository.

This example is all about the `Reflect` trait that allows us to do a lot of dynamic stuff rust usually doesn't let us do.
Firstly we derive Reflect on a struct called Foo, that allows use to retrieve fields from it using a string. We can also retrieve mutable fields.

Secondly we use a DynamicStruct that let's us create a struct that have dynamic fields, we can add new fields to that struct on the go. and then apply that struct to another struct, here Foo, so that all matching fields are applied to it.

Then we use Reflect to serialize the struct (into a RON serialized data). Serde Serialize and Deserialize traits are usable automatically by any type that derives Reflect.

Finally we deserialize the data using a similar way into a boxed dyn Reflect type.

We can also apply that dyn Reflect data to our Foo structure.