pub trait GpuPrimitive: bytemuck::Pod + bytemuck::Zeroable {
    fn data(&self) -> Vec<u8>;
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

mod vertex;
pub use vertex::GpuVertex;

mod triangle;
pub use triangle::GpuTriangle;