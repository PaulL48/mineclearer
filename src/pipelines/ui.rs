use once_cell::sync::Lazy;
use renderer::{PipelineConfiguration, RenderFunction, PipelineUnit};
use wgpu::{CommandEncoder, TextureView, CommandBuffer};

use crate::ui_vertex::UiVertex;

pub static UI_BIND_GROUP_LAYOUTS: Lazy<Vec<Vec<wgpu::BindGroupLayoutEntry>>> = Lazy::new(|| {vec![
    vec![
        // Camera
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    ]
]});

pub static UI_PIPELINE: Lazy<PipelineConfiguration> = Lazy::new(|| {
    PipelineConfiguration {
        shader_path: "./resources/shaders/shader.wgsl".to_string(),
        vertex_shader_entrypoint: "vs_main".to_string(),
        vertex_buffer_layouts: vec![UiVertex::BUFFER_LAYOUT],
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: None,
        polygon_mode: wgpu::PolygonMode::Fill,
        depth_write_enabled: true,
        depth_compare_function: wgpu::CompareFunction::Less,
        fragment_shader_entrypoint: "fs_main".to_string(),
        fragment_shader_blend_mode: Some(wgpu::BlendState::ALPHA_BLENDING),
        fragment_shader_write_mask: wgpu::ColorWrites::ALL,
        bind_group_layouts: UI_BIND_GROUP_LAYOUTS.clone(),
    }
});

pub fn ui_render_function(
    mut encoder: CommandEncoder,
    view: &TextureView,
    depth_texture: &TextureView,
    pipeline: &PipelineUnit
) -> CommandBuffer {
    encoder.finish()
}

