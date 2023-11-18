use bevy::prelude::*;

use crate::gif::{Gif, GifResource};
use crate::{despawn_screen, MainState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(MainState::Game), setup_game)
            .add_systems(OnExit(MainState::Game), despawn_screen::<OnGameScreen>);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Desabled,
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component, Event, Resource)]
struct Screen(GifResource);

impl Default for Screen {
    fn default() -> Self {
        Screen(GifResource::new(0.2, "load".to_string(), 1))
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
            parent.spawn((ImageBundle {
                image: UiImage::new(gif.get("load".to_string(), 1)),
                ..default()
            },));
        });
}
