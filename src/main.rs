#![allow(dead_code)]

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
};
use std::time::{Duration, Instant};

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SLEEP_DURATION: u32 = 1_000_000_000u32 / 60;

mod components;
mod game;
mod resources;
mod systems;

fn main() -> Result<(), crate::Error> {
    let ctx = sdl2::init()?;

    sdl2::image::init(InitFlag::PNG)?;

    let video = ctx.video()?;

    let window = video
        .window("Rust Gamedev Workshop", WIDTH, HEIGHT)
        .position_centered()
        .build()?;

    let canvas = window.into_canvas().build()?;

    let texture_creator = canvas.texture_creator();

    let textures = vec![
        texture_creator.load_texture("assets/bardo_2x.png")?,
        texture_creator.load_texture("assets/reaper_blade_2x.png")?,
        texture_creator.load_texture("assets/pinktrees_2x.png")?,
    ];

    // Create the world and all the default entities.
    let mut world = game::world();
    // Create the systems that will _drive_ the game.
    let mut systems = game::systems(canvas, textures)?;

    // Fetch the events from the window.
    let mut events = ctx.event_pump()?;

    // Used to track the delta time between frames.
    let mut start = Instant::now();

    loop {
        let events = events.poll_iter().collect::<Vec<_>>();

        if events.iter().any(quit_or_esc) {
            break;
        }

        let elapsed = start.elapsed();
        start = Instant::now();

        world.insert::<resources::Delta>(elapsed.into());
        world.insert::<resources::KeyEvents>(events.into());

        // Run the game!
        systems.dispatch(&mut world);

        std::thread::sleep(Duration::new(0, SLEEP_DURATION));
    }

    Ok(())
}

fn quit_or_esc(event: &Event) -> bool {
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => true,
        _ => false,
    }
}
