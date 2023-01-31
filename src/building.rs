use bevy::prelude::*;

use crate::{
    bounds::Bounds2,
    loading::{BuildingAssets, TextureAssets},
    mouse_position::MousePosition,
    tiles::Tile,
    GameState,
};

#[derive(Component)]
pub struct Buildable;

#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct TownCentre;

impl TownCentre {
    pub fn build(commands: &mut Commands, textures: &BuildingAssets) -> Entity {
        commands
            .spawn((
                Building,
                TownCentre,
                SpriteBundle {
                    texture: textures.town_centre.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                },
            ))
            .id()
    }
}

#[derive(Resource)]
pub struct BuildingState {
    pub mode_active: bool,
}

impl Default for BuildingState {
    fn default() -> Self {
        Self { mode_active: false }
    }
}

// Start - Building Events
pub struct BuildingModeChange(pub bool);
pub struct TownCentreBuilt;
// End - Building Events

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildingModeChange>()
            .add_event::<TownCentreBuilt>()
            .insert_resource(BuildingState::default())
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(BuildingIndicator::spawn),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(BuildingIndicator::track_position)
                    .with_system(BuildingIndicator::track_visibility)
                    .with_system(BuildingPlugin::listen_build_mode_event)
                    .with_system(BuildingPlugin::cancel_build_mode),
            );
    }
}

impl BuildingPlugin {
    fn listen_build_mode_event(
        mut event: EventReader<BuildingModeChange>,
        mut state: ResMut<BuildingState>,
    ) {
        if !event.is_empty() {
            for change in event.iter() {
                state.mode_active = change.0;
            }
        }

        event.clear();
    }

    fn cancel_build_mode(keys: Res<Input<KeyCode>>, mut state: ResMut<BuildingState>) {
        if keys.just_pressed(KeyCode::Escape) {
            state.mode_active = false;
        }
    }
}

#[derive(Component)]
pub struct BuildingIndicator;

impl BuildingIndicator {
    fn spawn(mut commands: Commands, textures: Res<TextureAssets>) {
        commands.spawn((
            BuildingIndicator,
            SpriteBundle {
                texture: textures.texture_selector.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 5.0),
                visibility: Visibility { is_visible: false },
                ..default()
            },
            Name::new("BuildingIndicator"),
        ));
    }

    fn track_position(
        mut indicator_query: Query<(&mut Transform, &mut Handle<Image>), With<BuildingIndicator>>,
        tile_query: Query<(&Bounds2, Option<&Buildable>), With<Tile>>,
        mouse: Res<MousePosition>,
        textures: Res<TextureAssets>,
    ) {
        let (mut transform, mut texture) = indicator_query.single_mut();

        for (bound, buildable) in tile_query.iter() {
            if bound.in_bounds_centered(mouse.world) {
                transform.translation = Vec3::new(bound.position.x, bound.position.y, 5.0);

                match buildable {
                    Some(_) => {
                        *texture = textures.texture_selector.clone();
                    }
                    None => {
                        *texture = textures.texture_selector_err.clone();
                    }
                };

                return;
            }
        }
    }

    fn track_visibility(
        mut query: Query<&mut Visibility, With<BuildingIndicator>>,
        state: Res<BuildingState>,
    ) {
        let mut visiblity = query.single_mut();
        visiblity.is_visible = state.mode_active;
    }
}
