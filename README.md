# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the first entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/vroussea/daily-bevy/branches).
Original ones can be found [here](https://github.com/awwsmm/daily-bevy/branches).

### Disclaimer
The goal of this repository will be to follow the tracks of [awwsmm's repo](https://github.com/awwsmm/daily-bevy/blob/master/README.md) that focuses on working on one example from Bevy repo per day.2


## Today's example
Today, we'll be digging into [the `2d_gizmos` example](https://github.com/bevyengine/bevy/blob/v0.13.0/examples/2d/2d_gizmos.rs) from the Bevy repo.

In this example we get a new method call added to the `App` : `.init_gizmo_group::<MyRoundGizmos>()`
It comes from the `AppGizmoBuilder` `impl`ementation in the `App`. Here it is :
```rust
fn init_gizmo_group<T: GizmoConfigGroup + Default>(&mut self) -> &mut Self {
        if self.world.contains_resource::<GizmoStorage<T>>() {
            return self;
        }

        self.init_resource::<GizmoStorage<T>>()
            .add_systems(Last, update_gizmo_meshes::<T>);

        self.world
            .get_resource_or_insert_with::<GizmoConfigStore>(Default::default)
            .register::<T>();

        let Ok(render_app) = self.get_sub_app_mut(RenderApp) else {
            return self;
        };

        render_app.add_systems(ExtractSchedule, extract_gizmo_data::<T>);

        self
    }

```
With this we can add a custom `GizmoConfigGroup` which seems to be used to hold several `GizmoConfig`. this type looks like this:

```rust
/// A struct that stores configuration for gizmos.
#[derive(Clone, Reflect)]
pub struct GizmoConfig {
    // -- snip --
    pub enabled: bool,
    // -- snip --
    pub line_width: f32,
    // -- snip --
    pub line_perspective: bool,
    // -- snip --
    pub depth_bias: f32,
    // -- snip --
    pub render_layers: RenderLayers,
}
```

I'll paste here the small definition Awwsmm gave :

> "The term is kinda fuzzy and means different things in different engines, but in Bevy it refers to lightweight 3D wireframe overlays that you can use for visual debugging." - [pcwalton](https://news.ycombinator.com/item?id=39413585), [Bevy contributor](https://github.com/pcwalton)

So debugging wireframes huh ?

There are different shapes we can draw in the `Gizmos` `impl`ementation :

```rust
impl<'w, 's, T: GizmoConfigGroup> Gizmos<'w, 's, T> {
    // -- snip --
    pub fn line(&mut self, start: Vec3, end: Vec3, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn line_gradient(&mut self, start: Vec3, end: Vec3, start_color: Color, end_color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn ray(&mut self, start: Vec3, vector: Vec3, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn ray_gradient(
        &mut self,
        start: Vec3,
        vector: Vec3,
        start_color: Color,
        end_color: Color,
    ) {
        // -- snip --
    }

    // -- snip --
    pub fn linestrip(&mut self, positions: impl IntoIterator<Item = Vec3>, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn linestrip_gradient(&mut self, points: impl IntoIterator<Item = (Vec3, Color)>) {
        // -- snip --
    }

    // -- snip --
    pub fn sphere(
        &mut self,
        position: Vec3,
        rotation: Quat,
        radius: f32,
        color: Color,
    ) -> SphereBuilder<'_, 'w, 's, T> {
        // -- snip --
    }

    // -- snip --
    pub fn rect(&mut self, position: Vec3, rotation: Quat, size: Vec2, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn cuboid(&mut self, transform: impl TransformPoint, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn line_2d(&mut self, start: Vec2, end: Vec2, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn line_gradient_2d(
        &mut self,
        start: Vec2,
        end: Vec2,
        start_color: Color,
        end_color: Color,
    ) {
        // -- snip --
    }

    // -- snip --
    pub fn linestrip_2d(&mut self, positions: impl IntoIterator<Item = Vec2>, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn linestrip_gradient_2d(&mut self, positions: impl IntoIterator<Item = (Vec2, Color)>) {
        // -- snip --
    }

    // -- snip --
    pub fn ray_2d(&mut self, start: Vec2, vector: Vec2, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn ray_gradient_2d(
        &mut self,
        start: Vec2,
        vector: Vec2,
        start_color: Color,
        end_color: Color,
    ) {
        // -- snip --
    }

    // -- snip --
    pub fn rect_2d(&mut self, position: Vec2, rotation: f32, size: Vec2, color: Color) {
        // -- snip --
    }

    // -- snip --
}
```

While `Gizmos are declared as follow:
```rust
/// A [`SystemParam`] for drawing gizmos.
///
/// They are drawn in immediate mode, which means they will be rendered only for
/// the frames in which they are spawned.
/// Gizmos should be spawned before the [`Last`](bevy_app::Last) schedule to ensure they are drawn.
pub struct Gizmos<'w, 's, T: GizmoConfigGroup = DefaultGizmoConfigGroup> {
    buffer: Deferred<'s, GizmoBuffer<T>>,
    pub(crate) enabled: bool,
    /// The currently used [`GizmoConfig`]
    pub config: &'w GizmoConfig,
    /// The currently used [`GizmoConfigGroup`]
    pub config_ext: &'w T,
}

This bit of
```

As the comment says, we should always try to draw them before the `Last` schedule.

Them being `SystemParam` is what allows us to retrieve them directly inside systems, like we'd do for queries, resources etc..

Here we are trying to init `MyRoundGizmos` `GizmoConfigGroup` inside the App. The type we create also needs `Default` and `Reflect` as stated in `.init_gismo_group()`.

The method checks if that config group already exists and returns self (the `App`) if that's the case. It then init a `GizmoStorage<T>` resource, which holds informations about the position and color of different Items (lists and strips) as well as a `PhantomData` marker of the T type. Here is how it looks:

```rust
#[derive(Resource, Default)]
pub(crate) struct GizmoStorage<T: GizmoConfigGroup> {
    pub list_positions: Vec<PositionItem>,
    pub list_colors: Vec<ColorItem>,
    pub strip_positions: Vec<PositionItem>,
    pub strip_colors: Vec<ColorItem>,
    marker: PhantomData<T>,
}
```

It then add an `Update` system to that type T called `update_gizmo_meshes::<T>` that looks like this :

```rust
fn update_gizmo_meshes<T: GizmoConfigGroup>(
    mut line_gizmos: ResMut<Assets<LineGizmo>>,
    mut handles: ResMut<LineGizmoHandles>,
    mut storage: ResMut<GizmoStorage<T>>,
) {
    if storage.list_positions.is_empty() {
        handles.list.remove(&TypeId::of::<T>());
    } else if let Some(handle) = handles.list.get(&TypeId::of::<T>()) {
        let list = line_gizmos.get_mut(handle).unwrap();

        list.positions = mem::take(&mut storage.list_positions);
        list.colors = mem::take(&mut storage.list_colors);
    } else {
        let mut list = LineGizmo {
            strip: false,
            ..Default::default()
        };

        list.positions = mem::take(&mut storage.list_positions);
        list.colors = mem::take(&mut storage.list_colors);

        handles
            .list
            .insert(TypeId::of::<T>(), line_gizmos.add(list));
    }

    if storage.strip_positions.is_empty() {
        handles.strip.remove(&TypeId::of::<T>());
    } else if let Some(handle) = handles.strip.get(&TypeId::of::<T>()) {
        let strip = line_gizmos.get_mut(handle).unwrap();

        strip.positions = mem::take(&mut storage.strip_positions);
        strip.colors = mem::take(&mut storage.strip_colors);
    } else {
        let mut strip = LineGizmo {
            strip: true,
            ..Default::default()
        };

        strip.positions = mem::take(&mut storage.strip_positions);
        strip.colors = mem::take(&mut storage.strip_colors);

        handles
            .strip
            .insert(TypeId::of::<T>(), line_gizmos.add(strip));
    }
}
```

It is split between the two Vec we saw earlier:
- the first part uses the `list` with a `LineGizmo` set on false
- the second part uses `strip` with `LineGizmo` set on true

`LineGizmo` definition:
```rust
#[derive(Asset, Debug, Default, Clone, TypePath)]
struct LineGizmo {
    positions: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    /// Whether this gizmo's topology is a line-strip or line-list
    strip: bool,
}
```

Lines seem to be made of a continuous segment while strips are made of different connected segments between points A and B. From what I understand we would use lines when drawning simples shapes while strips allows us to draw complex ones like circles.

In both lines and stripes case the function checks if the vecs of positions (3d positions) are empty in which case it removes it's `TypeId` from  the `mut handles: ResMut<LineGizmoHandles>` argument. This type is as follow:
```rust
#[derive(Resource, Default)]
struct LineGizmoHandles {
    list: TypeIdMap<Handle<LineGizmo>>,
    strip: TypeIdMap<Handle<LineGizmo>>,
}
```
It seems that removing it from that list to prevent spawning unnecessary entities in this function:
```rust
fn extract_gizmo_data<T: GizmoConfigGroup>(
    mut commands: Commands,
    handles: Extract<Res<LineGizmoHandles>>,
    config: Extract<Res<GizmoConfigStore>>,
) {
    let (config, _) = config.config::<T>();

    if !config.enabled {
        return;
    }

    for map in [&handles.list, &handles.strip].into_iter() {
        let Some(handle) = map.get(&TypeId::of::<T>()) else {
            continue;
        };
        commands.spawn((
            LineGizmoUniform {
                line_width: config.line_width,
                depth_bias: config.depth_bias,
                #[cfg(feature = "webgl")]
                _padding: Default::default(),
            },
            (*handle).clone_weak(),
            GizmoMeshConfig::from(config),
        ));
    }
}
```

In the case that there are positions, it tries to get the `Handle` on the `LineGizmo` and extract the positions and colors of the stored lines onto the `Handle` by using `mem::take` that will replace them by their default in the storage. So it means that if we didn't add new values until next time this system is called, then the Handle will be remove.

Finally the last else brings us to the case of a non-empty stored list_position but inexisting Handle, in which case we create it in  line mode (`strip: false`) before moving the values inside as well and then adding the Handle to the list of handles. with a `TypeId` of type T.

We have almost the same process on the strips except that this time we set `strip: true`.

###short answer
We need to add the gizmo positions to draw in every loop before the `Last` schedule in order to see the gizmo, otherwise the handle is dropped until the next time there is something to draw, in which case the handle would be created again.

Now back to the `init_gizmo_group` method, we added the resource and system. Now we get or insert the `GizmoConfigStore` resource. Which is defined as:
```rust
/// A [`Resource`] storing [`GizmoConfig`] and [`GizmoConfigGroup`] structs
///
/// Use `app.init_gizmo_group::<T>()` to register a custom config group.
#[derive(Resource, Default)]
pub struct GizmoConfigStore {
    // INVARIANT: must map TypeId::of::<T>() to correct type T
    store: TypeIdMap<(GizmoConfig, Box<dyn Reflect>)>,
}
```

It stores tuples of `GizmoConfig` and our newly created `GizmoConfigGroup`. Those tuples are registered this way:
```rust
    /// Inserts [`GizmoConfig`] and [`GizmoConfigGroup`] replacing old values
    pub fn insert<T: GizmoConfigGroup>(&mut self, config: GizmoConfig, ext_config: T) {
        // INVARIANT: hash map must correctly map TypeId::of::<T>() to &dyn Reflect of type T
        self.store
            .insert(TypeId::of::<T>(), (config, Box::new(ext_config)));
    }

    pub(crate) fn register<T: GizmoConfigGroup>(&mut self) {
        self.insert(GizmoConfig::default(), T::default());
    }
```
It creates a default `GizmoConfig` and default T type which is our `GizmoConfigGroup` type (here `MyRoundGizmos`). It then insert that tuple into the store with the `TypeId` of T being the key. So to say it links a config to our type. This is `GizmoConfig` definition:
```rust
/// A struct that stores configuration for gizmos.
#[derive(Clone, Reflect)]
pub struct GizmoConfig {
    /// Set to `false` to stop drawing gizmos.
    ///
    /// Defaults to `true`.
    pub enabled: bool,
    /// Line width specified in pixels.
    ///
    /// If `line_perspective` is `true` then this is the size in pixels at the camera's near plane.
    ///
    /// Defaults to `2.0`.
    pub line_width: f32,
    /// Apply perspective to gizmo lines.
    ///
    /// This setting only affects 3D, non-orthographic cameras.
    ///
    /// Defaults to `false`.
    pub line_perspective: bool,
    /// How closer to the camera than real geometry the line should be.
    ///
    /// In 2D this setting has no effect and is effectively always -1.
    ///
    /// Value between -1 and 1 (inclusive).
    /// * 0 means that there is no change to the line position when rendering
    /// * 1 means it is furthest away from camera as possible
    /// * -1 means that it will always render in front of other things.
    ///
    /// This is typically useful if you are drawing wireframes on top of polygons
    /// and your wireframe is z-fighting (flickering on/off) with your main model.
    /// You would set this value to a negative number close to 0.
    pub depth_bias: f32,
    /// Describes which rendering layers gizmos will be rendered to.
    ///
    /// Gizmos will only be rendered to cameras with intersecting layers.
    pub render_layers: RenderLayers,
}
```
It allows us to decide if we want to draw the gizmo, it's line width, if we want perspective, how we want to handle depth in 3d by choosing how far from the camera it is rendered and on which layer we want to draw the gizmo ? The layers are defined here:
```rust
/// Describes which rendering layers an entity belongs to.
///
/// Cameras with this component will only render entities with intersecting
/// layers.
///
/// There are 32 layers numbered `0` - [`TOTAL_LAYERS`](RenderLayers::TOTAL_LAYERS). Entities may
/// belong to one or more layers, or no layer at all.
///
/// The [`Default`] instance of `RenderLayers` contains layer `0`, the first layer.
///
/// An entity with this component without any layers is invisible.
///
/// Entities without this component belong to layer `0`.
#[derive(Component, Copy, Clone, Reflect, PartialEq, Eq, PartialOrd, Ord)]
#[reflect(Component, Default, PartialEq)]
pub struct RenderLayers(LayerMask);
```

Next `init_gizmo_group` gets a `RendereApp` sub_app of our `App`:
```rust
/// A [`SubApp`] contains its own [`Schedule`] and [`World`] separate from the main [`App`].
/// This is useful for situations where data and data processing should be kept completely separate
/// from the main application. The primary use of this feature in bevy is to enable pipelined rendering.
///
// -- snip --
pub struct SubApp {
    /// The [`SubApp`]'s instance of [`App`]
    pub app: App,

    /// A function that allows access to both the main [`App`] [`World`] and the [`SubApp`]. This is
    /// useful for moving data between the sub app and the main app.
    extract: Box<dyn Fn(&mut World, &mut App) + Send>,
}
```
So a `SubApp` is an `App` with it's own world and schedule. And the `RendereApp` sub app was created by the `RenderPlugin`. Sub apps are run in the Update schedule.

If we do have that sub app as we should, we then add one more system that is run on `ExtractSchedule` render_app schedule which is used to extract data from the main App's world to the render sub app`s world.

The system is `extract_gizmo_data` (definition above). It spawns entity on the render app by using the `LineGizmoUniform` bundle, the handle on the `LineGizmo` and a `GizmoMeshConfig` created from the `GizmoConfig` associated to each types of gizmo.

#
Here is the short version from Awwsmm:
So now, we finally understand what init_gizmo_group is doing

    we init a GizmoStorage<T> Resource for the specified T: GizmoConfigGroup
        GizmoStorage<T> contains list_positions / list_colors and strip_positions / strip_colors
        these positions and colors are used to render the Gizmos in the group
    we add an update_gizmo_meshes::<T> system to the Last Schedule of the main App
        this moves the latest positions and colors out of GizmoStorage<T> and into a new or existing LineGizmo Handle
        this is called on every Update, so we need to be constantly repopulating the storage in order for our Gizmos to rerender
    we add a GizmoConfigStore Resource to the World
        and we register the type T in the GizmoConfigStore's store field
        multiple GizmoConfigGroups can register with the same GizmoConfigStore
    we get a mutable reference to the RenderApp SubApp
    we add the extract_gizmo_data::<T> system to the ExtractSchedule in the RenderApp
        ExtractSchedule for a SubApp is called within the Update Schedule of the main App

Phew. What's next?
#

Now our app has a basic setup system on `Startup` which spawns the camera and some text to explain how to interact with the App.
Then we have two `Update` systems : `draw_example_collection, update_config`

The draw collection system creates/updates a few different gizmos (lines, rays, rectangles, circles, ellipses, arrows, arcs) all in 2d. They are split into two different groups: one for all the "linear" ones and one for the more elaborat ones likes circles, ellipses etc. The first group is going to use lists while the second one will use strips made of segments.
As for the config update, it takes keyboard input resource, time resource and the config_store where the gizmos are. One set of keys will interact with the basic gizmos and the rest with the gizmos in the `MyRoundGizmos` group.
It interact with them by changing the values inside the config (scaling line_width and toggling enabled) with min and max values on width.

So to say the first system draws the gizmos, while the other one change their configs depending on user inputs.