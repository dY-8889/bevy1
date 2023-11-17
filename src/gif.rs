use std::fmt::Debug;
use std::{collections::HashMap, fs::read_dir};

use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub trait Trigger<ER: Event + Resource> {
    fn event_trigger(time: Res<Time>, state: ResMut<ER>, gif_event: EventWriter<ER>);
}

pub trait Status {
    fn exe() -> Self;
}

enum LoadStatu {
    Standby,
}

impl Status for LoadStatu {
    fn exe() -> Self {
        LoadStatu::Standby
    }
}

/// # Example
/// ```
/// #[drive(Component, Default)]
/// struct Screen;
///
/// #[drive(Event, Resource)]
/// struct Event(Timer)
///
/// impl Default for Event {
///     fn default() -> Self {
///          Event(Timer::from_seconds(0.5, TimerMode::Repeating))
///     }
/// }
///
/// impl Trigger<Event> for Event {
///     fn event_trigger(
///         time: Res<Time>,
///         mut state: ResMut<Event>,
///         mut gif_event: EventWriter<Event>
///     ) {
///         if state.0.tick(time.delta()).finished() {
///             gif_event.send_default();
///         }
///     }
/// }
///
/// fn main() {
///     App::new().add_plugins(GifPlugin::<Screen, Event>::default()).run();
/// }
///
/// fn setup(commands: Commands) {
///     commands.spawn((
///         ImageBundle {
///             image: UiImage::new(gif.get("load", 1)),
///             ..default()
///         },
///         Screen,
///     ));
/// }
/// ```
pub struct GifPlugin<C, ER>(pub C, pub ER);

impl<C, ER> Plugin for GifPlugin<C, ER>
where
    C: Component,
    ER: Event + Resource + Default + Trigger<ER>,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<ER>()
            .add_event::<ER>()
            .add_systems(Update, ER::event_trigger)
            .add_systems(Update, gif_load::<C, ER>);
    }
}

impl<C: Default, ER: Default> Default for GifPlugin<C, ER> {
    fn default() -> Self {
        GifPlugin(C::default(), ER::default())
    }
}

#[derive(Debug, Resource)]
pub struct Gif(pub HashMap<String, Vec<Handle<Image>>>);

impl Gif {
    pub fn new(asset_server: Res<AssetServer>) -> Self {
        let mut map: HashMap<String, Vec<Handle<Image>>> = HashMap::new();

        let folder = get_folder("assets/images/");

        for entry in folder {
            let mut images: Vec<Handle<Image>> = Vec::new();

            for img_path in get_folder(&entry) {
                let path = img_path[7..].to_string();
                images.push(asset_server.load(path));
            }

            map.insert(entry[14..].to_string(), images);
        }

        Gif(map)
    }

    pub fn get(&self, key: &str, frame: usize) -> Handle<Image> {
        self.0.get(key).expect("キーが存在しません")[frame].clone()
    }

    fn _sort() {}
}

pub fn gif_load<C: Component, E: Event>(
    mut image_query: Query<&mut UiImage, With<C>>,
    gif: Res<Gif>,
    mut gif_event: EventReader<E>,
) {
    for _ in gif_event.read() {
        for mut image in &mut image_query {
            image.texture = gif.get("load", thread_rng().gen_range(0..5));
        }
    }
}

fn get_folder(target_path: &str) -> Vec<String> {
    let mut folder: Vec<String> = Vec::new();

    if let Ok(folder_path) = read_dir(target_path) {
        for dir_entry in folder_path {
            let path = dir_entry.unwrap().path().to_string_lossy().into_owned();
            folder.push(path);
        }
    }

    folder
}
