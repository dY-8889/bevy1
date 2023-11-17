use std::{collections::HashMap, thread};

use bevy::prelude::*;
use bevy_simple_text_input::{TextInput, TextInputPlugin, TextInputSubmitEvent};
use clipboard_macos::Clipboard;

use crate::{
    despawn_screen,
    server::{post, server, server_check, User},
    MainState, FONT_PATH, TEXT_COLOR,
};

const URL_MAP: [(&'static str, &'static str); 2] = [
    ("check", "http://127.0.0.1:8080/check"),
    ("connect", "http://127.0.0.1:8080/connect"),
];

pub struct ConnectPlugin;

impl Plugin for ConnectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin)
            .add_systems(OnEnter(MainState::Connect), main_connect_setup)
            .add_systems(
                OnExit(MainState::Connect),
                despawn_screen::<OnConnectScreen>,
            )
            .add_systems(
                Update,
                (connect_action, focus).run_if(in_state(MainState::Connect)),
            )
            .add_systems(OnEnter(ConnectResult::Ok), connect_ok)
            .add_systems(OnEnter(ConnectResult::Err), connect_err)
            .add_systems(Update, clipboard_input.run_if(in_state(MainState::Connect)));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, States)]
enum ConnectResult {
    Ok,
    Err,
    #[default]
    None,
}

#[derive(Component)]
enum ConnectAction {
    Confirm,
    Post,
}

#[derive(Component)]
struct OnConnectScreen;

//
const BACKGROUND_COLOR_ACTIVE: Color = Color::rgb(0.4, 0.4, 0.4);
const BACKGROUND_COLOR_INACTIVE: Color = Color::rgb(0.2, 0.2, 0.2);

fn main_connect_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load(FONT_PATH),
        font_size: 40.,
        color: TEXT_COLOR.into(),
    };
    


    let button = ButtonBundle {
        style: Style {
            width: Val::Px(150.),
            height: Val::Px(65.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::vertical(Val::Px(20.)),
            ..default()
        },
        ..default()
    };

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
            OnConnectScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        padding: UiRect::all(Val::Px(1.0)),
                        margin: UiRect::vertical(Val::Px(20.)),
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                },
                TextInput {
                    text_style: text_style.clone(),
                    ..default()
                },
            ));

            parent
                .spawn((button.clone(), ConnectAction::Confirm))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("サーバー", text_style.clone()));
                });
            parent
                .spawn((button, ConnectAction::Post))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("post", text_style));
                });
        });
}

fn connect_ok() {}
fn connect_err() {}

fn connect_action(
    interaction_query: Query<(&Interaction, &ConnectAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, connect_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            let url: HashMap<&str, &str> = HashMap::from(URL_MAP);
            match connect_action {
                ConnectAction::Confirm => {
                    thread::spawn(move || {
                        if !server_check(url.get("check").unwrap()) {
                            server().unwrap();
                        }
                    });
                }
                ConnectAction::Post => {
                    thread::spawn(|| {
                        let data = User::new("dai", 1, "");
                        let res_data = post("http://127.0.0.1:8080/connect", data);
                        println!("{:#?}", res_data);
                    });
                }
            }
        }
    }
}

// ペースト
fn clipboard_input(
    text_input_query: Query<(Entity, &mut TextInput)>,
    key_input: Res<Input<KeyCode>>,
    mut submit_events: ParamSet<(
        EventWriter<TextInputSubmitEvent>,
        EventReader<TextInputSubmitEvent>,
    )>,
) {
    let command = key_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    for (input_entity, input) in &text_input_query {
        if input.inactive {
            continue;
        }

        if command && key_input.any_just_pressed([KeyCode::V]) {
            let clipboard = Clipboard::new().expect("not clipboard");
            if let Ok(text) = clipboard.read() {
                submit_events.p0().send(TextInputSubmitEvent {
                    entity: input_entity,
                    value: text,
                });
            }
        }
    }
    for event in submit_events.p1().read() {
        info!("{}", event.value);
    }
}

//
fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInput, &mut BackgroundColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut text_input, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    text_input.inactive = false;
                    *border_color = BACKGROUND_COLOR_ACTIVE.into();
                } else {
                    text_input.inactive = true;
                    *border_color = BACKGROUND_COLOR_INACTIVE.into();
                }
            }
        }
    }
}
