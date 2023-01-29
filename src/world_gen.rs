use crate::{bounds::Bounds2, global_state::GlobalState, loading::TextureAssets, GameState};
use bevy::prelude::*;
use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable, utils::*};
use rand::{prelude::random, Rng};

pub struct WorldGenPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
enum GeneratorSteps {
    World,
}

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(WorldGenerator::spawn_world.label(GeneratorSteps::World)),
        );
    }
}

#[derive(PartialEq, Debug)]
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
            0..=3 => TileType::WATER,
            4..=10 => TileType::SAND,
            11..=15 => TileType::DIRT,
            16..=95 => TileType::GRASS,
            96..=100 => TileType::STONE,
            _ => TileType::GRASS,
        }
    }
}

#[derive(Component)]
pub struct Tile(TileType);

#[derive(Component)]
pub struct ResourceTile;

pub trait ResourceItem {
    fn texture(&self, textures: &TextureAssets) -> Handle<Image>;
    fn yieldAmount(&self) -> usize;
}

#[derive(Component)]
pub struct ResourceTree {
    pub lvl: usize,
}

impl Default for ResourceTree {
    fn default() -> Self {
        ResourceTree {
            lvl: rand::thread_rng().gen_range(1..=4),
        }
    }
}

impl ResourceItem for ResourceTree {
    fn texture(&self, textures: &TextureAssets) -> Handle<Image> {
        match self.lvl {
            1 => textures.texture_tree_1.clone(),
            2 => textures.texture_tree_2.clone(),
            3 => textures.texture_tree_3.clone(),
            4 => textures.texture_tree_4.clone(),
            _ => textures.texture_tree_1.clone(),
        }
    }

    fn yieldAmount(&self) -> usize {
        match self.lvl {
            1 => 1,
            2 => 3,
            3 => 5,
            4 => 7,
            _ => 0,
        }
    }
}

#[derive(Component)]
pub struct ResourceStone {
    pub lvl: usize,
}

impl Default for ResourceStone {
    fn default() -> Self {
        Self {
            lvl: rand::thread_rng().gen_range(1..=3),
        }
    }
}

impl ResourceItem for ResourceStone {
    fn texture(&self, textures: &TextureAssets) -> Handle<Image> {
        match self.lvl {
            1 => textures.texture_stone_1.clone(),
            2 => textures.texture_stone_2.clone(),
            3 => textures.texture_stone_3.clone(),
            _ => textures.texture_stone_1.clone(),
        }
    }

    fn yieldAmount(&self) -> usize {
        match self.lvl {
            1 => 1,
            2 => 3,
            3 => 5,
            _ => 0,
        }
    }
}

#[derive(Component)]
pub struct ResourceBerry {
    pub lvl: usize,
}

impl Default for ResourceBerry {
    fn default() -> Self {
        Self {
            lvl: rand::thread_rng().gen_range(1..=2),
        }
    }
}

impl ResourceItem for ResourceBerry {
    fn texture(&self, textures: &TextureAssets) -> Handle<Image> {
        match self.lvl {
            1 => textures.texture_berry_1.clone(),
            2 => textures.texture_berry_2.clone(),
            _ => textures.texture_berry_1.clone(),
        }
    }

    fn yieldAmount(&self) -> usize {
        self.lvl
    }
}

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
                let is_grass = tile_type == TileType::GRASS;
                let texture = tile_type.texture(&textures);
                let position = Vec2::new(size * x as f32 - x_offset, size * y as f32 - y_offset);
                let transform = Transform::from_xyz(position.x, position.y, 0.0);

                let id = commands
                    .spawn((
                        Tile(tile_type),
                        Bounds2 {
                            position,
                            size: Vec2::splat(size),
                        },
                    ))
                    .id();

                if is_grass {
                    WorldGenerator::spawn_grass_resources(
                        &mut commands,
                        &id,
                        texture,
                        transform,
                        &textures,
                    );
                } else {
                    commands.entity(id).insert(SpriteBundle {
                        texture,
                        transform,
                        ..default()
                    });
                }

                y += 1;
            }

            x += 1;
        }
    }

    fn spawn_grass_resources(
        commands: &mut Commands,
        entity: &Entity,
        grass_texture: Handle<Image>,
        transform: Transform,
        textures: &TextureAssets,
    ) {
        let mut rng = rand::thread_rng();
        let roll = rng.gen::<f32>();
        let percentage = (roll * 100.) as i32;

        match percentage {
            0..=15 => {
                let tree = ResourceTree::default();
                let texture = tree.texture(textures);

                commands
                    .entity(*entity)
                    .insert(ResourceTile)
                    .insert(tree)
                    .insert(SpriteBundle {
                        texture,
                        transform,
                        ..default()
                    });
            }
            16..=21 => {
                let stone = ResourceStone::default();
                let texture = stone.texture(textures);

                commands
                    .entity(*entity)
                    .insert(ResourceTile)
                    .insert(stone)
                    .insert(SpriteBundle {
                        texture,
                        transform,
                        ..default()
                    });
            }
            22..=30 => {
                let berry = ResourceBerry::default();
                let texture = berry.texture(textures);

                commands
                    .entity(*entity)
                    .insert(ResourceTile)
                    .insert(berry)
                    .insert(SpriteBundle {
                        texture,
                        transform,
                        ..default()
                    });
            }
            _ => {
                commands.entity(*entity).insert(SpriteBundle {
                    texture: grass_texture,
                    transform,
                    ..default()
                });
            }
        };
    }
}
