#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
pub trait GetGPUData {
    fn get_gpu_points(&self) -> Vec<[f32; 2]>;
    fn get_gpu_pivot(&self) -> [f32; 2];
    fn get_gpu_color(&self) -> [f32; 3];
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        let [px, py] = self.get_gpu_pivot();
        let points = self.get_gpu_points();
        let mut result: Vec<GPUVertex> = Vec::with_capacity(points.len());
        for [lx, ly] in self.get_gpu_points().iter() {
            result.push(GPUVertex {
                position: [px + lx, py + ly, 1.0],
                color: self.get_gpu_color(),
            })
        }
        result
    }
}
