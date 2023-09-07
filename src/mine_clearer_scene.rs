use std::sync::{Arc, Mutex};
use application_context::{Scene, SceneState, SceneBuilder, Nanoseconds};
use bevy_ecs::{world::World, schedule::Schedule, system::Resource};
use renderer::Renderer;
use winit::window::Window;
use crate::{component_resources::Time, pipelines::{UI_PIPELINE, ui_render_function}};

pub struct MineclearSceneBuilder;

impl SceneBuilder for MineclearSceneBuilder {
    fn initialize(
        &mut self,
        window: Arc<Mutex<Window>>,
        renderer: Arc<Mutex<Renderer>>,
    ) -> Box<dyn Scene> {
        Box::new(MineClearerScene::new(window, renderer))
    }
}

struct MineClearerComponents {
    set: World,
    fixed_step_schedule: Schedule,
    pre_render_schedule: Schedule,
}

impl MineClearerComponents {
    pub fn new() -> Self {
        let mut set = World::new();

        set.insert_resource(Time::new());
        
        let fixed_step_schedule = Schedule::default();
        let pre_render_schedule = Schedule::default();

        Self {
            set,
            fixed_step_schedule,
            pre_render_schedule
        }
    }

    pub fn run_fixed_step(&mut self, dt: f32) {
        *self.set.get_resource_mut::<Time>().unwrap().fixed_step_mut() = dt;
        self.fixed_step_schedule.run(&mut self.set);
    }

    pub fn run_pre_render_step(&mut self, alpha: f32) {
        *self.set.get_resource_mut::<Time>().unwrap().alpha_mut() = alpha;
        self.pre_render_schedule.run(&mut self.set);
    }
}

pub struct MineClearerScene {
    window: Arc<Mutex<Window>>,
    renderer: Arc<Mutex<Renderer>>,
    components: MineClearerComponents,
}

impl MineClearerScene {
    pub fn new(window: Arc<Mutex<Window>>, renderer: Arc<Mutex<Renderer>>) -> Self {
        {
            let mut renderer = renderer.lock().unwrap();
            renderer.insert_pipeline(&UI_PIPELINE, ui_render_function);
        }

        
        Self {
            window,
            renderer,
            components: MineClearerComponents::new(),
        }
    }
}

impl Scene for MineClearerScene {
    fn process_events(&mut self, time: &application_context::TimingData, events: &application_context::EventBuffer) {

    }

    fn fixed_step_update(&mut self, time: &application_context::TimingData) -> SceneState {
        self.components.run_fixed_step(time.dt_s());
        SceneState::Running
    }

    fn interpolate(&mut self, time: &application_context::TimingData) {
        self.components.run_pre_render_step(time.alpha());
    }

    fn render(&mut self) {

    }
}

// Note:
// We could take interpolation to the next level by interpolating color animations on vertices

