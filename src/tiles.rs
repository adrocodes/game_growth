use crate::loading::TextureAssets;
use bevy::prelude::*;
use rand::Rng;

#[derive(PartialEq, Debug)]
pub enum TileType {
    DIRT,
    GRASS,
    SAND,
    STONE,
    WATER,
}

impl TileType {
    pub fn texture(&self, assets: &TextureAssets) -> Handle<Image> {
        match self {
            TileType::DIRT => assets.texture_dirt.clone(),
            TileType::GRASS => assets.texture_grass.clone(),
            TileType::SAND => assets.texture_sand.clone(),
            TileType::STONE => assets.texture_stone.clone(),
            TileType::WATER => assets.texture_water.clone(),
        }
    }

    pub fn from_perlin(value: &f32) -> Self {
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
pub struct Tile(pub TileType);

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
