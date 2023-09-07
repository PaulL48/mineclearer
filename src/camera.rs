use glam::{Vec3, Mat4, Vec2, vec2};
use renderer::AsPod;

#[derive(Debug)]
pub struct Camera {
    size: Vec2,
    z_near: f32,
    z_far: f32,
}

impl Camera {
    // pub const BIND_GROUP_ENTRY_LAYOUT: BindGroupLayoutEntry = BindGroupLayoutEntry {
    //     binding: 0,
    //     visibility: wgpu::ShaderStages::,
    //     ty: todo!(),
    //     count: todo!(),
    // };
    
    pub fn new() -> Self {
        Self {
            size: vec2(800.0, 600.0),
            z_near: 0.1,
            z_far: 100.0,
        }
    }
}

impl AsPod for Camera {
    type Target = [[[f32; 4]; 4]; 2];

    fn as_pod(&self) -> Self::Target {
        let view = Mat4::look_at_rh(Vec3::Z, Vec3::ZERO, Vec3::Y);
        let projection = Mat4::orthographic_rh(
            -self.size.x / 2.0, 
            self.size.x / 2.0, 
            -self.size.y / 2.0, 
            self.size.y / 2.0, 
            self.z_near, 
            self.z_far
        );

        [view.to_cols_array_2d(), projection.to_cols_array_2d()]
    }
}
