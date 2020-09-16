use crate::arkanoid::{Ball, ARENA_WIDTH, ARENA_HEIGHT, ScoreBoard, ScoreText};
use amethyst::{
    ui::UiText,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage},
    core::ecs::{ReadExpect, Write},
};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (
        mut balls,
        mut locals,
        mut ui_text,
        mut scores,
        score_text
    ): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_y = transform.translation().y;

            let did_hit = if ball_y <= ball.radius {
                scores.score_left = (scores.score_left + 1)
                    .min(999);
                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };


            if did_hit {
                ball.velocity[0] = -ball.velocity[0]; // Reverse Direction
                transform.set_translation_x(ARENA_WIDTH / 2.0); // Reset Position
                transform.set_translation_y(ARENA_HEIGHT / 2.0); // Reset Position

                println!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_left, scores.score_right
                );
            }
        }
    }
}