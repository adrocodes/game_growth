use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalState {
    pub world_rows: usize,
    pub world_cols: usize,
    pub block_size: usize,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            world_rows: 32,
            world_cols: 48,
            block_size: 64,
        }
    }
}
