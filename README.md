# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the first entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/vroussea/daily-bevy/branches).
Original ones can be found [here](https://github.com/awwsmm/daily-bevy/branches).

### Disclaimer
The goal of this repository will be to follow the tracks of [awwsmm's repo](https://github.com/awwsmm/daily-bevy/blob/master/README.md) that focuses on working on one example from Bevy repo per day.2

## Today's example
Today we worked again on the WASM example by trying to save how many times the button was clicked, we are using bevy_pkv to do so, it uses the local Web Storage.

We have a State `Resource` that stores the amount of times the button was clicked, it is initialized inside the state_setup system that tries to retrieve the stored value (get) and if doesn't exist creates it (set).

in the button_system every time the button is pressed we modify the click counter inside the state `Resource` and save it (set).

If the button is not interacted with we simply show how many times it was pressed so far.

I changed Awwsmm example by first fixing the value not being set inside the storage. My guess is that he first tried to set it before retrieving it and didn't check if someone using his example would have an issue with this or not. Thus why i created this new setup system.

This way I also reduce the amount of time we use get (by 2) on the state we are retrieving, should improve performance. Might be possible to improve it furthermore.