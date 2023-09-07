use application_context::Nanoseconds;
use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct Time {
    total_simulated_time: Nanoseconds,
    fixed_step: f32,
    alpha: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            total_simulated_time: 0,
            fixed_step: 0.0,
            alpha: 0.0
        }
    }

    pub fn fixed_step(&self) -> f32 {
        self.fixed_step
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    pub fn fixed_step_mut(&mut self) -> &mut f32 {
        &mut self.fixed_step
    }

    pub fn alpha_mut(&mut self) -> &mut f32 {
        &mut self.alpha
    }
}
