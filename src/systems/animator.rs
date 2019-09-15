use crate::{
    components::{Animation, Motion, Sprite},
    resources::Delta,
};
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use std::time::Duration;

#[derive(Debug)]
pub struct Animator {}

impl Animator {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, Motion>,
        ReadExpect<'a, Delta>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut animations, mut sprites, motion, delta) = data;

        for (animation, sprite, motion) in (&mut animations, &mut sprites, &motion).join() {
            let next_frame = if let Some(dur) = animation.next_frame.checked_sub(delta.0) {
                animation.next_frame = dur;
                false
            } else {
                animation.next_frame = animation.step_delay;
                true
            };

            if next_frame {
                sprite.coordinates = animation.sprites[animation.current].into();

                if animation.current < animation.sprites.len() - 1 {
                    animation.current += 1;
                } else {
                    animation.current = 0;
                }
            }
        }
    }
}
