use crate::global_state::GlobalState;
use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_pancam::PanCam;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_play_button))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu));
    }
}

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb_u8(53, 186, 243),
            hovered: Color::rgb_u8(255, 255, 225),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    global_state: Res<GlobalState>,
) {
    let offset = global_state.block_size * 0;
    let max_vertical_offset = global_state.block_size * 0;
    let max_vertical = ((global_state.world_rows * global_state.block_size) / 2) + offset;
    let max_horizontal = ((global_state.world_cols * global_state.block_size) / 2) + offset;

    commands.spawn((
        Camera2dBundle::default(),
        PanCam {
            grab_buttons: vec![MouseButton::Middle],
            min_scale: 1.,
            min_x: Some(-(max_horizontal as f32)),
            max_x: Some(max_horizontal as f32),
            min_y: Some(-(max_vertical as f32 + max_vertical_offset as f32)),
            max_y: Some(max_vertical as f32),
            ..default()
        },
    ));

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(75.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Button>>) {
    commands.entity(button.single()).despawn_recursive();
}
