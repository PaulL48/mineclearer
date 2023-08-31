mod board;
mod mineclear_scene;
mod render_details;

use std::time::Duration;

use application_context::SingleWindowApplicationContext;
use mineclear_scene::MineclearSceneBuilder;
use render_details::BIND_GROUP_LAYOUT;
use winit::dpi::PhysicalSize;

fn main() {
    let mut context = SingleWindowApplicationContext::new(
        PhysicalSize::new(800, 600), 
        Duration::from_millis(10).as_nanos() as u64,
        Duration::from_millis(250).as_nanos() as u64
    ).unwrap();

    BIND_GROUP_LAYOUT;
    context.queue_next_scene(Box::new(MineclearSceneBuilder{}));

    context.run();
}
