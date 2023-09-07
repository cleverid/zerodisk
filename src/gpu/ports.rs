#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
pub trait GetVertices {
    fn get_vertices(&self) -> Vec<GPUVertex>;
}
