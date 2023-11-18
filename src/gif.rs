use std::fmt::Debug;
use std::{collections::HashMap, fs::read_dir};

use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct GifPlugin<T>(T);

impl<T> Plugin for GifPlugin<T>
where
    T: Event + Resource + Component + Default,
{
    fn build(&self, app: &mut App) {
        app.add_event::<T>()
            .init_resource::<T>()
            .add_systems(Update, event_trigger::<T>);
    }
}

#[derive(Resource)]
pub struct GifResource {
    timer: Timer,
    key: String,
    frame: usize,
}

impl GifResource {
    pub fn new(time: f32, key: String, frame: usize) -> Self {
        GifResource {
            timer: Timer::from_seconds(time, TimerMode::Repeating),
            key,
            frame,
        }
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

    pub fn get(&self, key: String, frame: usize) -> Handle<Image> {
        self.0.get(&key).expect("キーが存在しません")[frame].clone()
    }

    fn _sort() {}
}

fn event_trigger<T: Event + Default>(
    time: Res<Time>,
    mut state: ResMut<GifResource>,
    mut gif_event: EventWriter<T>,
) {
    if state.timer.tick(time.delta()).finished() {
        gif_event.send_default()
    }
}
fn gif_load<T: Component + Event>(
    mut gif_screen_query: Query<&mut UiImage, With<T>>,
    gif_res: Res<Gif>,
    mut events: EventReader<T>,
) {
    for _ in events.read() {
        for mut image in &mut gif_screen_query {
            image.texture = gif_res.get("load".to_string(), thread_rng().gen_range(0..10))
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
