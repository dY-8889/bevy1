use bevy::prelude::*;

// use serde::{Deserialize, Serialize};

use crate::gif::{Gif, GifPlugin, Trigger};
use crate::{despawn_screen, MainState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(MainState::Game), setup_game)
            .add_systems(OnExit(MainState::Game), despawn_screen::<OnGameScreen>)
            .add_plugins(GifPlugin::<LoadScreen, Event>::default());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Desabled,
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component, Default)]
struct LoadScreen;

#[derive(Event, Resource)]
struct Event(Timer);

impl Default for Event {
    fn default() -> Self {
        Event(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

impl Trigger<Event> for Event {
    fn event_trigger(time: Res<Time>, mut state: ResMut<Event>, mut gif_event: EventWriter<Event>) {
        if state.0.tick(time.delta()).finished() {
            gif_event.send_default();
            println!("event_trigger");
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
