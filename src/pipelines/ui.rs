use once_cell::sync::Lazy;
use renderer::{PipelineConfiguration, PipelineUnit, BufferBacked, ModelInstance, AsPod};
use wgpu::{CommandEncoder, TextureView, CommandBuffer, Queue};

use crate::{ui_vertex::UiVertex, camera::Camera};

pub static UI_BIND_GROUP_LAYOUTS: Lazy<Vec<Vec<wgpu::BindGroupLayoutEntry>>> = Lazy::new(|| {vec![
    vec![
        // Camera
        Camera::buffer_layout(0),
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
    queue: &mut Queue,
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
        render_pass.set_bind_group(0, pipeline.bind_groups().get(0).unwrap(), &[]);

        for (model, instances) in &pipeline.render_queue {
            if instances.len() == 0 {
                continue;
            }

            for mesh in model.meshes() {
                render_pass.set_bind_group(1, model.materials()[mesh.material_index()].bind_group(), &[]);
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
                render_pass.set_vertex_buffer(1, instances.buffer().slice(..(std::mem::size_of::<<ModelInstance as AsPod>::Target>() * instances.len()) as u64));
                render_pass.set_index_buffer(mesh.index_buffer().slice(..), mesh.index_type());
                render_pass.draw_indexed(0..mesh.index_count(), 0, 0..(instances.len() as u32));
            }
        }
    }
    encoder.finish()
}

