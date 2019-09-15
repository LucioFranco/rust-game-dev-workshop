use sdl2::rect::Rect;
use specs::{storage::*, Component};
use std::time::Duration;

/// Marks an entity as the player.
#[derive(Debug, Component, Default)]
#[storage(NullStorage)]
pub struct Player;

/// An entity that can be positioned somewhere.
#[derive(Debug, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// An entity that has some bound box.
#[derive(Debug, Component)]
pub struct BoundingBox {
    pub width: u32,
    pub height: u32,
}

/// An entity that can move.
#[derive(Debug, Component)]
pub struct Motion {
    pub velocity_x: i32,
    pub velocity_y: i32,
    pub direction: Direction,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            velocity_x: 0,
            velocity_y: 0,
            direction: Direction::North,
        }
    }
}

/// A direction. Ordered in clockwise fasion.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North = 0,
    East,
    South,
    West,
}

/// Represents a Sprite sheet item.
#[derive(Debug, Component)]
pub struct Sprite {
    pub texture_id: usize,
    pub coordinates: Option<Rect>,
}

/// Represents an animated enitity.
#[derive(Debug, Component)]
pub struct Animation {
    pub sprites: Vec<Rect>,
    pub current: usize,
    pub step_delay: Duration,
    pub next_frame: Duration,
}

impl Animation {
    pub fn new(sprites: Vec<Rect>, step_delay: Duration) -> Self {
        Self {
            sprites,
            step_delay,
            current: 0,
            next_frame: Duration::from_millis(0),
        }
    }
}
