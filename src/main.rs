mod board;
mod mine_clearer_scene;
mod components;
mod component_resources;
mod pipelines;
mod ui_vertex;
mod camera;

use std::time::Duration;

use application_context::SingleWindowApplicationContext;
use log::LevelFilter;
use mine_clearer_scene::MineclearSceneBuilder;
use simplelog::{CombinedLogger, WriteLogger, Config, TermLogger, TerminalMode, ColorChoice};
use winit::dpi::PhysicalSize;

const LOG_FILE: &str = "./graphics.log";

fn main() {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(LOG_FILE)
        .map_err(|e| e.to_string())
        .unwrap();

    CombinedLogger::init(vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), file),
            TermLogger::new(
                LevelFilter::Info,
                Config::default(),
                TerminalMode::Stdout,
                ColorChoice::Auto,
            ),
    ])
    .map_err(|e| e.to_string())
    .unwrap();
    

    let mut context = SingleWindowApplicationContext::new(
        PhysicalSize::new(800, 600), 
        Duration::from_millis(10).as_nanos() as u64,
        Duration::from_millis(250).as_nanos() as u64
    ).unwrap();

    context.queue_next_scene(Box::new(MineclearSceneBuilder{}));

    context.run();
}
