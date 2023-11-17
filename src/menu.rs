use bevy::{app::AppExit, prelude::*};

use crate::{despawn_screen, MainState, FONT_PATH, TEXT_COLOR};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(OnEnter(MainState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(Update, menu_action.run_if(in_state(MenuState::Main)))
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMenuScreen>);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum MenuState {
    Main,
    #[default]
    Disabled,
}

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    Game,
    Connect,
    Quit,
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(15.)),
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
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        ..default()
                    },
                    MenuButtonAction::Game,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.,
                            color: TEXT_COLOR.into(),
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        ..default()
                    },
                    MenuButtonAction::Connect,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Connect",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.,
                            color: TEXT_COLOR.into(),
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style,
                        ..default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.,
                            color: TEXT_COLOR.into(),
                        },
                    ));
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut main_state: ResMut<NextState<MainState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Game => {
                    main_state.set(MainState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Connect => {
                    main_state.set(MainState::Connect);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Quit => app_exit.send(AppExit),
            }
        }
    }
}
