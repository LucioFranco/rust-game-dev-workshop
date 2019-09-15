use crate::{
    components::{Direction, Motion, Player},
    resources::KeyEvents,
};
use sdl2::{event::Event, keyboard::Keycode};
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

#[derive(Debug, Default)]
pub struct Input {}

impl<'a> System<'a> for Input {
    type SystemData = (
        WriteStorage<'a, Motion>,
        ReadStorage<'a, Player>,
        ReadExpect<'a, KeyEvents>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut motion, players, events) = data;
        let events: &KeyEvents = &*events;

        let mut x = 0;
        let mut y = 0;
        let mut direction = None;

        for event in &events.0 {
            if let Event::KeyDown { keycode, .. } = event {
                if let Some(Keycode::Up) = keycode {
                    y -= 1;
                    direction = Some(Direction::North);
                }

                if let Some(Keycode::Down) = keycode {
                    y += 1;
                    direction = Some(Direction::South);
                }

                if let Some(Keycode::Right) = keycode {
                    x += 1;
                    direction = Some(Direction::East);
                }

                if let Some(Keycode::Left) = keycode {
                    x -= 1;
                    direction = Some(Direction::West);
                }
            }
        }

        for (motion, _) in (&mut motion, &players).join() {
            motion.velocity_x = x;
            motion.velocity_y = y;

            if let Some(dir) = direction {
                motion.direction = dir;
            }
        }
    }
}
