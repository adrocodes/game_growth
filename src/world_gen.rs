use crate::{bounds::Bounds2, global_state::GlobalState, loading::TextureAssets, GameState};
use bevy::prelude::*;
use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable, utils::*};
use rand::prelude::random;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(WorldGenerator::spawn_world),
        );
    }
}

pub enum TileType {
    DIRT,
    GRASS,
    SAND,
    STONE,
    WATER,
}

impl TileType {
    fn texture(&self, assets: &TextureAssets) -> Handle<Image> {
        match self {
            TileType::DIRT => assets.texture_dirt.clone(),
            TileType::GRASS => assets.texture_grass.clone(),
            TileType::SAND => assets.texture_sand.clone(),
            TileType::STONE => assets.texture_stone.clone(),
            TileType::WATER => assets.texture_water.clone(),
        }
    }

    fn from_perlin(value: &f32) -> Self {
        let as_percentage: i32 = (value.abs() * 100.) as i32;
        match as_percentage {
            0..=5 => TileType::WATER,
            6..=15 => TileType::SAND,
            16..=25 => TileType::DIRT,
            26..=90 => TileType::GRASS,
            91..=100 => TileType::STONE,
            _ => TileType::GRASS,
        }
    }
}

#[derive(Component)]
pub struct Tile(TileType);

struct WorldGenerator;

impl WorldGenerator {
    fn generate_perlin_atlas(global_state: &GlobalState) -> Vec<Vec<f32>> {
        let seed = random::<u32>();
        let hasher = PermutationTable::new(seed);
        let perlin_v2 = PlaneMapBuilder::new_fn(perlin_2d, &hasher)
            .set_size(global_state.world_cols, global_state.world_rows)
            .set_x_bounds(0.1, 1.0)
            .set_y_bounds(0.1, 1.0)
            .build();

        (0..global_state.world_cols)
            .map(|x| {
                (0..global_state.world_rows)
                    .map(|y| perlin_v2.get_value(x, y) as f32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn spawn_world(mut commands: Commands, state: Res<GlobalState>, textures: Res<TextureAssets>) {
        let atlas = WorldGenerator::generate_perlin_atlas(&state);
        let cols = state.world_cols;
        let rows = state.world_rows;
        let size = state.block_size as f32;
        let x_offset = (size * (cols as f32 / 2.)) - (size / 2.);
        let y_offset = (size * (rows as f32 / 2.)) - (size / 2.);

        let mut x = 0;
        for row in atlas {
            let mut y = 0;

            for col in row {
                let tile_type = TileType::from_perlin(&col);
                let texture = tile_type.texture(&textures);
                let position = Vec2::new(size * x as f32 - x_offset, size * y as f32 - y_offset);

                commands.spawn((
                    Tile(tile_type),
                    SpriteBundle {
                        texture,
                        transform: Transform::from_xyz(position.x, position.y, 0.0),
                        ..default()
                    },
                    Bounds2 {
                        position,
                        size: Vec2::splat(size),
                    },
                ));

                y += 1;
            }

            x += 1;
        }
    }
}
