# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the first entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/vroussea/daily-bevy/branches).
Original ones can be found [here](https://github.com/awwsmm/daily-bevy/branches).

### Disclaimer
The goal of this repository will be to follow the tracks of [awwsmm's repo](https://github.com/awwsmm/daily-bevy/blob/master/README.md) that focuses on working on one example from Bevy repo per day.2

## Today's example
Today's focus is going to be on the [asset_loading example](https://github.com/bevyengine/bevy/blob/release-0.12.1/examples/asset/asset_loading.rs) from Bevy's repository.

It's main focus is on loading assets from the assets folder into the usable assets the app has access to. We use an AssetServer Resource that we use to load (.load()) assets files with. We can also load folders (.load_folder()) directly and get handlers on specific assets within by just loading the given asset (it won't be loaded again, you just get a strong Handle on it). When we start loading the asset we don't have access to it right away. The loading is non blocking and the asset will be accessible later on. For now we can just use the Handles on them to create the entities by giving them to the bundles, once the asset will be loaded the entity will have access to it.

We then spawn the three 3d shapes of which we loaded their mesh with a new material asset we created. We throw in one light, one 3d camera and that's it.

And that's it, nothing new besides that for today.