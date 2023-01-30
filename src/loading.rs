use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<PersonAssets>()
                .with_collection::<BuildingAssets>()
                .continue_to_state(GameState::Menu),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
    #[asset(path = "world/dirt.png")]
    pub texture_dirt: Handle<Image>,
    #[asset(path = "world/grass.png")]
    pub texture_grass: Handle<Image>,
    #[asset(path = "world/sand.png")]
    pub texture_sand: Handle<Image>,
    #[asset(path = "world/stone.png")]
    pub texture_stone: Handle<Image>,
    #[asset(path = "world/water.png")]
    pub texture_water: Handle<Image>,

    #[asset(path = "world/tree_1.png")]
    pub texture_tree_1: Handle<Image>,
    #[asset(path = "world/tree_2.png")]
    pub texture_tree_2: Handle<Image>,
    #[asset(path = "world/tree_3.png")]
    pub texture_tree_3: Handle<Image>,
    #[asset(path = "world/tree_4.png")]
    pub texture_tree_4: Handle<Image>,

    #[asset(path = "world/stone_1.png")]
    pub texture_stone_1: Handle<Image>,
    #[asset(path = "world/stone_2.png")]
    pub texture_stone_2: Handle<Image>,
    #[asset(path = "world/stone_3.png")]
    pub texture_stone_3: Handle<Image>,

    #[asset(path = "world/berry_1.png")]
    pub texture_berry_1: Handle<Image>,
    #[asset(path = "world/berry_2.png")]
    pub texture_berry_2: Handle<Image>,

    #[asset(path = "textures/selector.png")]
    pub texture_selector: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct PersonAssets {
    #[asset(path = "workers/person.png")]
    pub person: Handle<Image>,
    #[asset(path = "workers/person_1.png")]
    pub person_1: Handle<Image>,
    #[asset(path = "workers/person_2.png")]
    pub person_2: Handle<Image>,
    #[asset(path = "workers/person_3.png")]
    pub person_3: Handle<Image>,
    #[asset(path = "workers/person_4.png")]
    pub person_4: Handle<Image>,
    #[asset(path = "workers/person_5.png")]
    pub person_5: Handle<Image>,
    #[asset(path = "workers/person_6.png")]
    pub person_6: Handle<Image>,
    #[asset(path = "workers/person_7.png")]
    pub person_7: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct BuildingAssets {
    #[asset(path = "buildings/barracks.png")]
    pub barracks: Handle<Image>,
    #[asset(path = "buildings/farm.png")]
    pub farm: Handle<Image>,
    #[asset(path = "buildings/house.png")]
    pub house: Handle<Image>,
    #[asset(path = "buildings/house_1.png")]
    pub house_1: Handle<Image>,
    #[asset(path = "buildings/house_2.png")]
    pub house_2: Handle<Image>,
    #[asset(path = "buildings/shade.png")]
    pub shade: Handle<Image>,
    #[asset(path = "buildings/stall.png")]
    pub stall: Handle<Image>,
    #[asset(path = "buildings/town_centre.png")]
    pub town_centre: Handle<Image>,
}
