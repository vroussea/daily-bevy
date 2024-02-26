# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the first entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/vroussea/daily-bevy/branches).
Original ones can be found [here](https://github.com/awwsmm/daily-bevy/branches).

### Disclaimer
The goal of this repository will be to follow the tracks of [awwsmm's repo](https://github.com/awwsmm/daily-bevy/blob/master/README.md) that focuses on working on one example from Bevy repo per day.2

## Today's example
Today, we'll be going back into the [`3d_shapes` example](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/3d/3d_shapes.rs) found in the Bevy repo.

This time, the goal would be to change bevy's version from `0.12.1` to the latest `0.13.0` and check what needs to be changed. I decided to go only for this example as it is among the ones with most changes because of the new release.

The major changes would be that the shapes we previously used are now deprecated and we should now use simpler versions of them (that now have 2d or 3d in their name depending in the use).
One new thing about it is that there is now Meshable impl on those shapes which is a trait called inside the From<> impl for Meshes, which let us remove the unecessary `.into()` when adding the meshes to the assets.

Unlike Awwsmm I'm not going to review every small changes in other examples that we did before and we just use Bevy `0.13.0` from now on.