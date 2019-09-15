use crate::{
    components::{Animation, BoundingBox, Motion, Player, Position, Sprite},
    systems::{Animator, Input, Physics, Renderer},
};
use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};
use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};
use std::time::Duration;

/// Create a new world with default entities.
pub fn world() -> World {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Motion>();
    world.register::<BoundingBox>();
    world.register::<Player>();
    world.register::<Sprite>();
    world.register::<Animation>();

    // Player
    let player_starting_animations = gen_locations(&[(0, 0)], 52, 72);
    let player_walking = gen_locations(&[(0, 3), (1, 3), (2, 3)], 52, 72);
    world
        .create_entity()
        .with(Player::default())
        .with(Position { x: 0, y: 0 })
        .with(Motion::default())
        .with(Sprite {
            texture_id: 0,
            coordinates: None,
        })
        .with(Animation::new(player_walking, Duration::from_millis(270)))
        .build();

    world
        .create_entity()
        .with(Position { x: -100, y: -200 })
        .with(Sprite {
            texture_id: 2,
            coordinates: Some(Rect::new(0, 0, 128, 128)),
        })
        .build();

    world
}

pub fn systems(
    canvas: Canvas<Window>,
    textures: Vec<Texture<'_>>,
) -> Result<Dispatcher<'_, '_>, crate::Error> {
    let dispatcher = DispatcherBuilder::new()
        .with(Input::default(), "input", &[])
        .with(Physics::new(), "physics", &["input"])
        .with(Animator::new(), "animator", &["physics"])
        // The renderer is not Send or Sync so we need to run it on the
        // dispatchers thread.
        .with_thread_local(Renderer::new(canvas, textures))
        .build();

    Ok(dispatcher)
}

fn gen_locations(locs: &[(u32, u32)], width: u32, height: u32) -> Vec<Rect> {
    let mut sprites = Vec::new();

    for (x, y) in locs {
        let x = x * width;
        let y = y * height;
        sprites.push(Rect::new(x as i32, y as i32, width, height));
    }

    sprites
}
// 624 x 576 12 x 8
// 624 / 12 = 52
// 576 / 8 = 72
