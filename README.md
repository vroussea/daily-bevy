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
Today's focus is going to be on the [`game_menu`](https://github.com/bevyengine/bevy/blob/release-0.12.1/examples/games/game_menu.rs) example example from Bevy's repository.

This example is about resources (that holds game settings), game state (splash screen, menu screen, game screen) in which we can either change the settings or run the game depending on it. And finally about plugins that let us split the code in a lot more modular parts.

I first decided to change the example by adding a new GameState PluginGroup that holds all other plugins, since that's how a game is going to be split, with different groups of plugins holding different logics for different parts of the game. Some could say that the most basics plugins are supposed to be re-usable for other games in some ways ?

Now let's attack the first plugin :

### splash
This plugin only adds 3 different systems, the first difference compared to previous days'examples is with the scheduling of those systems.
We get `OnEnter(GameState::Splash)`, `OnExit(GameState::Splash)` and a normal Update.
OnEnter seems to wait for the given state to be set before calling the system while OnExit waits for exiting that state.
We also have a `.run_if()` on the Update system that checks if we are `in_state(GameSTate::Splash)`. run_if() takes a `Condition`, what is it ?
Here is the definition:
``` rust
/// A system that determines if one or more scheduled systems should run.
///
/// Implemented for functions and closures that convert into [`System<Out=bool>`](crate::system::System)
/// with [read-only](crate::system::ReadOnlySystemParam) parameters.
///
/// # Examples
/// A condition that returns true every other time it's called.
/// ```
/// # use bevy_ecs::prelude::*;
/// fn every_other_time() -> impl Condition<()> {
///     IntoSystem::into_system(|mut flag: Local<bool>| {
///         *flag = !*flag;
///         *flag
///     })
/// }
///
/// # #[derive(Resource)] struct DidRun(bool);
/// # fn my_system(mut did_run: ResMut<DidRun>) { did_run.0 = true; }
/// # let mut schedule = Schedule::default();
/// schedule.add_systems(my_system.run_if(every_other_time()));
/// # let mut world = World::new();
/// # world.insert_resource(DidRun(false));
/// # schedule.run(&mut world);
/// # assert!(world.resource::<DidRun>().0);
/// # world.insert_resource(DidRun(false));
/// # schedule.run(&mut world);
/// # assert!(!world.resource::<DidRun>().0);
/// ```
///
/// A condition that takes a bool as an input and returns it unchanged.
///
/// ```
/// # use bevy_ecs::prelude::*;
/// fn identity() -> impl Condition<(), bool> {
///     IntoSystem::into_system(|In(x)| x)
/// }
///
/// # fn always_true() -> bool { true }
/// # let mut app = Schedule::default();
/// # #[derive(Resource)] struct DidRun(bool);
/// # fn my_system(mut did_run: ResMut<DidRun>) { did_run.0 = true; }
/// app.add_systems(my_system.run_if(always_true.pipe(identity())));
/// # let mut world = World::new();
/// # world.insert_resource(DidRun(false));
/// # app.run(&mut world);
/// # assert!(world.resource::<DidRun>().0);
```