use crate::components::{Motion, Position};
use specs::{Join, ReadStorage, System, WriteStorage};

const SPEED: i32 = 2;

#[derive(Debug)]
pub struct Physics {}

impl Physics {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Motion>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut pos, motions) = data;

        for (pos, motion) in (&mut pos, &motions).join() {
            pos.x += motion.velocity_x * SPEED;
            pos.y += motion.velocity_y * SPEED;
        }
    }
}
