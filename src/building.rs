use bevy::prelude::*;

use crate::{
    bounds::Bounds2,
    loading::{BuildingAssets, TextureAssets},
    mouse_position::MousePosition,
    tiles::Tile,
    ui::GuiPluginLabels,
    GameState,
};

#[derive(Clone, Copy, PartialEq)]
pub enum BuildingType {
    TownCentre,
    Barracks,
    Farm,
    HouseOne,
    HouseTwo,
    House,
    Shade,
    Stall,
}

impl BuildingType {
    fn get_entity(&self, commands: &mut Commands, textures: &BuildingAssets) -> Option<Entity> {
        match self {
            BuildingType::TownCentre => Some(TownCentre::build(commands, textures)),
            _ => None,
        }
    }

    pub fn get_texture(&self, textures: &BuildingAssets) -> Handle<Image> {
        match self {
            BuildingType::TownCentre => textures.town_centre.clone(),
            BuildingType::Barracks => textures.barracks.clone(),
            BuildingType::Farm => textures.farm.clone(),
            BuildingType::HouseOne => textures.house_1.clone(),
            BuildingType::HouseTwo => textures.house_2.clone(),
            BuildingType::House => textures.house.clone(),
            BuildingType::Shade => textures.shade.clone(),
            BuildingType::Stall => textures.stall.clone(),
        }
    }
}

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
    pub building: Option<BuildingType>,
}

impl Default for BuildingState {
    fn default() -> Self {
        Self {
            mode_active: false,
            building: None,
        }
    }
}

// Start - Building Events
pub struct BuildingModeChange {
    pub state: bool,
    pub building: Option<BuildingType>,
}
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
                    .with_system(BuildingPlugin::cancel_build_mode)
                    .with_system(
                        BuildingPlugin::on_building_placed
                            .after(GuiPluginLabels::EnterBuildingMode),
                    ),
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
                state.mode_active = change.state;
                state.building = change.building;
            }
        }

        event.clear();
    }

    fn cancel_build_mode(keys: Res<Input<KeyCode>>, mut state: ResMut<BuildingState>) {
        if keys.just_pressed(KeyCode::Escape) {
            state.mode_active = false;
            state.building = None;
        }
    }

    fn on_building_placed(
        buttons: Res<Input<MouseButton>>,
        mut state: ResMut<BuildingState>,
        mut commands: Commands,
        textures: Res<BuildingAssets>,
        indicator_query: Query<&BuildingIndicator>,
        mut event: EventWriter<TownCentreBuilt>,
    ) {
        if buttons.just_pressed(MouseButton::Left) && state.mode_active {
            let indicator = indicator_query.single();

            if !indicator.valid_tile {
                return;
            }

            if let Some(tile_entity) = indicator.tile_entity {
                if let Some(building) = state.building {
                    let child = building.get_entity(&mut commands, &textures);

                    if let Some(child) = child {
                        commands.entity(tile_entity).push_children(&[child]);
                        commands.entity(tile_entity).remove::<Buildable>();

                        if building == BuildingType::TownCentre {
                            event.send(TownCentreBuilt);
                            state.mode_active = false;
                            state.building = None;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Component)]
pub struct BuildingIndicator {
    valid_tile: bool,
    tile_entity: Option<Entity>,
}

impl BuildingIndicator {
    fn spawn(mut commands: Commands, textures: Res<TextureAssets>) {
        commands.spawn((
            BuildingIndicator {
                valid_tile: false,
                tile_entity: None,
            },
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
        mut indicator_query: Query<(&mut Transform, &mut Handle<Image>, &mut BuildingIndicator)>,
        tile_query: Query<(Entity, &Bounds2, Option<&Buildable>), With<Tile>>,
        mouse: Res<MousePosition>,
        textures: Res<TextureAssets>,
    ) {
        let (mut transform, mut texture, mut indicator) = indicator_query.single_mut();

        for (entity, bound, buildable) in tile_query.iter() {
            if bound.in_bounds_centered(mouse.world) {
                transform.translation = Vec3::new(bound.position.x, bound.position.y, 5.0);

                match buildable {
                    Some(_) => {
                        *texture = textures.texture_selector.clone();
                        indicator.valid_tile = true;
                        indicator.tile_entity = Some(entity);
                    }
                    None => {
                        *texture = textures.texture_selector_err.clone();
                        indicator.valid_tile = false;
                        indicator.tile_entity = None;
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
