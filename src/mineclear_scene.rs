use std::sync::{Arc, Mutex};

use application_context::{Scene, SceneState, SceneBuilder};
use renderer::Renderer;
use winit::window::Window;

pub struct MineclearSceneBuilder;

impl SceneBuilder for MineclearSceneBuilder {
    fn initialize(
        &mut self,
        window: Arc<Mutex<Window>>,
        renderer: Arc<Mutex<Renderer>>,
    ) -> Box<dyn Scene> {
        Box::new(MineclearScene::new(window, renderer))
    }

}

pub struct MineclearScene {
    window: Arc<Mutex<Window>>,
    renderer: Arc<Mutex<Renderer>>,
}

impl MineclearScene {
    pub fn new(window: Arc<Mutex<Window>>, renderer: Arc<Mutex<Renderer>>) -> Self {
        // renderer.lock().unwrap().insert_pipeline(configuration, render_function)


        Self {
            window,
            renderer
        }
    }
}

impl Scene for MineclearScene {
    fn process_events(&mut self, time: &application_context::TimingData, events: &application_context::EventBuffer) {

    }

    fn fixed_step_update(&mut self, time: &application_context::TimingData) -> SceneState {
        SceneState::Running
    }

    fn interpolate(&mut self, time: &application_context::TimingData) {

    }

    fn render(&mut self) {

    }
}

// Note:
// We could take interpolation to the next level by interpolating color animations on vertices

