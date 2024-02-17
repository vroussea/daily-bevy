# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the first entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/vroussea/daily-bevy/branches).
Original ones can be found [here](https://github.com/awwsmm/daily-bevy/branches).

### Disclaimer
The goal of this repository will be to follow the tracks of [awwsmm's repo](https://github.com/awwsmm/daily-bevy/blob/master/README.md) that focuses on working on one example from Bevy repo per day.2

## Today's example
Today we are just compiling yesterday's app (also adding a 2dText) in web assembly.
You'll first need to add the target using rustup:
```
    rustup target add wasm32-unknown-unknown
```
Install the wasm bindgen cli:
```
    cargo install wasm-bindgen-cli
```
Now build the project for the wasm target:
```
    cargo build --release --target wasm32-unknown-unknown
```
And finally generate the js file that's going to be used in the example.html file. (files names are arbitrary, daily-bevy is the repo name):
```
    wasm-bindgen --out-name wasm_example --out-dir target --target web target/wasm32-unknown-unknown/release/daily-bevy.wasm
```
Serve the file using a server (here using python):
```
    python -m http.server
```
and open the html file from the server in your browser (here on windows):
```
    http://localhost:8000/example.html
```
