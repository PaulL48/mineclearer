use once_cell::sync::Lazy;
use renderer::{PipelineConfiguration, PipelineUnit};
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
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("UI Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_texture,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&pipeline.pipeline);

        // Here the buffer would need to be written to the queue
        // 

        render_pass.set_bind_group(0, bind_group, &[]);

        // Push a camera update
        // Get an orthographic camera

        for (model, instances) in &pipeline.render_queue {
            if instances.len() == 0 {
                continue;
            }

            for mesh in model.meshes() {
                render_pass.set_bind_group(index, bind_group, offsets)
            }
        }
    }

    encoder.finish()
}

