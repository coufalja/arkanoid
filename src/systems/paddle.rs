use amethyst::{
    core::{Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

// You'll have to mark PADDLE_HEIGHT as public in arkanoid
use crate::arkanoid::{Paddle, ARENA_WIDTH, PADDLE_WIDTH};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (_paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = input.axis_value("paddle");
            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_x = transform.translation().x;
                transform.set_translation_x(
                    (paddle_x + scaled_amount)
                        .min(ARENA_WIDTH - PADDLE_WIDTH * 0.5)
                        .max(PADDLE_WIDTH * 0.5),
                );
            }
        }
    }
}