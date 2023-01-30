use bevy::prelude::*;

use crate::{loading::BuildingAssets, GameState};

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
pub struct BuildingModeChange(bool);
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
                    .with_system(BuildingIndicator::track_visibility),
            );
    }
}

#[derive(Component)]
pub struct BuildingIndicator;

impl BuildingIndicator {
    fn spawn(mut commands: Commands) {}

    fn track_position() {
        todo!();
    }

    fn track_visibility() {
        todo!();
    }
}
