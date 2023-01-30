use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
    loading::{BuildingAssets, FontAssets},
    GameState,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(UiPlugin::spawn))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(ui_reveal_toggle));
    }
}

#[derive(Component)]
struct Panel;

impl UiPlugin {
    fn root() -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexEnd,
                position_type: PositionType::Relative,
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..default()
        }
    }

    fn main_panel_border() -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.), Val::Px(400.)),
                border: UiRect::all(Val::Px(5.0)),
                margin: UiRect::all(Val::Px(10.)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            focus_policy: FocusPolicy::Block,
            ..default()
        }
    }

    fn main_panel() -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(10.)),
                flex_wrap: FlexWrap::Wrap,
                align_items: AlignItems::FlexStart,
                align_content: AlignContent::FlexStart,
                justify_content: JustifyContent::FlexStart,
                position_type: PositionType::Relative,
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        }
    }

    fn toggle_help(fonts: &FontAssets) -> TextBundle {
        TextBundle {
            text: Text::from_section(
                "<q> to toggle",
                TextStyle {
                    font: fonts.fira_sans.clone(),
                    font_size: 14.,
                    color: Color::hex("666666").unwrap(),
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Px(20.), Val::Undefined, Val::Undefined, Val::Px(10.)),
                ..default()
            },
            ..default()
        }
    }

    fn build_building_button(handle: Handle<Image>) -> ButtonBundle {
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(64.), Val::Px(64.)),
                margin: UiRect::all(Val::Px(10.)),
                ..default()
            },
            image: UiImage(handle),
            ..default()
        }
    }

    fn spawn_town_centre_btn(textures: &BuildingAssets) -> ButtonBundle {
        UiPlugin::build_building_button(textures.town_centre.clone())
    }

    fn spawn(mut commands: Commands, textures: Res<BuildingAssets>, fonts: Res<FontAssets>) {
        commands.spawn(UiPlugin::root()).with_children(|parent| {
            parent
                .spawn((Panel, UiPlugin::main_panel_border()))
                .with_children(|parent| {
                    parent
                        .spawn(UiPlugin::main_panel())
                        .with_children(|parent| {
                            parent.spawn(UiPlugin::toggle_help(&fonts));
                            parent.spawn(UiPlugin::spawn_town_centre_btn(&textures));
                        });
                });
        });
    }
}

struct PanelState {
    active: bool,
}

impl Default for PanelState {
    fn default() -> Self {
        Self { active: true }
    }
}

fn ui_reveal_toggle(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Style, With<Panel>>,
    mut panel_state: Local<PanelState>,
) {
    if keys.just_pressed(KeyCode::Q) {
        let mut style = query.single_mut();
        let goal: f32 = match panel_state.active {
            true => -450.0,
            false => 0.0,
        };

        style.position.bottom = Val::Px(goal);
        panel_state.active = !panel_state.active;
    }
}
