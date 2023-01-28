use bevy::{
    prelude::{Color, Component, Query, Res, Vec2},
    sprite::Sprite,
};

use crate::mouse_position::WorldPosition;

#[derive(Debug, Clone, Copy, Component)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.position.x
            && coords.y >= self.position.y
            && coords.x <= self.position.x + self.size.x
            && coords.y <= self.position.y + self.size.y
    }

    pub fn in_bounds_centered(&self, coords: Vec2) -> bool {
        let half_size = self.size * Vec2::new(0.5, 0.5);
        let new_position = self.position - half_size;
        let bounds = Bounds2 {
            position: new_position,
            size: self.size,
        };

        return bounds.in_bounds(coords);
    }

    pub fn debug_mouse_over(mut query: Query<(&mut Sprite, &Bounds2)>, mouse: Res<WorldPosition>) {
        for (mut sprite, bounds) in query.iter_mut() {
            if bounds.in_bounds_centered(mouse.0) {
                sprite.color = Color::RED;
            } else {
                sprite.color = Color::WHITE;
            }
        }
    }
}
