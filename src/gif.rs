use std::{collections::HashMap, fs::read_dir, marker::PhantomData};

use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct GifPlugin<R, E> {
    _resource: PhantomData<fn() -> R>,
    _event: PhantomData<fn() -> E>,
}

//
impl<R, E> Plugin for GifPlugin<R, E>
where
    R: Resource + Default,
    E: Event + Default,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<R>()
            .add_event::<E>()
            .add_systems(Update, event_trgger::<E>);
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

#[derive(Event, Default)]
pub struct GifEvent;

#[derive(Resource)]
pub struct EventTriggerState {
    event_timer: Timer,
}

impl Default for EventTriggerState {
    fn default() -> Self {
        EventTriggerState {
            event_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

pub fn event_trgger<T: Event + Default>(
    time: Res<Time>,
    mut state: ResMut<EventTriggerState>,
    mut gif_event: EventWriter<T>,
) {
    if state.event_timer.tick(time.delta()).finished() {
        gif_event.send_default()
    }
}

pub fn gif_load<T: Component, E: Event + Default>(
    mut image_query: Query<&mut UiImage, With<T>>,
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
