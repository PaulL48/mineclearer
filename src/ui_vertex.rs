use renderer::AsPod;


#[derive(Debug, Copy, Clone)]
pub struct UiVertex {
    position: [f32; 3],
    texture_coordinates: [f32; 2]
}

impl UiVertex {
    pub const BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                shader_location: 1
            }
        ],
    };
}

impl AsPod for UiVertex {
    type Target = [f32; 5];

    fn as_pod(&self) -> Self::Target {
        [
            self.position[0],
            self.position[1],
            self.position[2],
            self.texture_coordinates[0],
            self.texture_coordinates[1]
        ]
    }
}

