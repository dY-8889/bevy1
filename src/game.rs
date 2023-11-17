use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use crate::gif::{gif_load, Gif, GifPlugin};
use crate::{despawn_screen, MainState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(MainState::Game), setup_game)
            .add_systems(Update, gif_load::<LoadScreen, LoadEvent>)
            .add_systems(OnExit(MainState::Game), despawn_screen::<OnGameScreen>);
        // .add_plugins(GifPlugin::<EventTrigger, LoadEvent>);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Desabled,
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct LoadScreen;

#[derive(Event, Default)]
pub struct LoadEvent;

#[derive(Resource)]
pub struct EventTrigger {
    timer: Timer,
}
impl Default for EventTrigger {
    fn default() -> Self {
        EventTrigger {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

fn setup_game(mut commands: Commands, gif: Res<Gif>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(gif.get("load", 1)),
                    ..default()
                },
                LoadScreen,
            ));
        });
}
