mod connect;
mod game;
mod gif;
mod menu;
mod server;

use bevy::prelude::*;

use gif::Gif;

fn main() {
    App::new()
        .add_state::<MainState>()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "connection app".to_string(),
                resolution: (1000., 600.).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Update, button_system)
        .add_systems(Startup, setup)
        .add_plugins((menu::MenuPlugin, game::GamePlugin, connect::ConnectPlugin))
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const _PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const FONT_PATH: &str = "font/ZenMaruGothicMedium.ttf";
pub const TEXT_COLOR: Color = Color::WHITE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, States)]
pub enum MainState {
    Game,
    Connect,
    #[default]
    Menu,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gif = Gif::new(asset_server);
    println!("{:#?}", gif);

    commands.insert_resource(gif);

    commands.spawn(Camera2dBundle::default());
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => *color = HOVERED_BUTTON.into(),
            Interaction::Hovered => *color = NORMAL_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
