use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

use crate::{
    gpu_types::GpuUniform,
    sim::{WORLD_EDGE_SEGMENTS, WORLD_RADIUS},
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WorldUniform {
    pub radius: f32,
    pub boundary_segments: u32,
    _padding: [f32; 2],
}

impl Default for WorldUniform {
    fn default() -> Self {
        Self {
            radius: WORLD_RADIUS,
            boundary_segments: WORLD_EDGE_SEGMENTS,
            _padding: [f32::default(), f32::default()],
        }
    }
}

unsafe impl bytemuck::Pod for WorldUniform {}
unsafe impl bytemuck::Zeroable for WorldUniform {}

impl GpuUniform for WorldUniform {
    fn bind(
        &self,
        device: &Device,
    ) -> (Buffer, Vec<u8>, BindGroup, BindGroupLayout) {
        let layout = create_wradius_bind_group_layout(device);
        let buffer_contents = get_wradius_buffer_contents();
        let buffer = create_wradius_buffer(device, &buffer_contents);
        let bind_group = create_world_bind_group(&buffer, &layout, device);
        (buffer, buffer_contents, bind_group, layout)
    }
}

fn get_wradius_buffer_contents() -> Vec<u8> {
    let uniform = WorldUniform {
        radius: WORLD_RADIUS,
        boundary_segments: WORLD_EDGE_SEGMENTS,
        _padding: [0.0, 0.0],
    };
    bytemuck::cast_slice(&[uniform]).to_vec()
}

fn create_world_bind_group(
    buffer: &Buffer,
    layout: &BindGroupLayout,
    device: &Device,
) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
        label: Some("World Radius Bind Group"),
    })
}

fn create_wradius_buffer(device: &Device, buffer_contents: &[u8]) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("World Radius Buffer"),
        contents: buffer_contents,
        usage: wgpu::BufferUsages::UNIFORM
            | wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST,
    })
}

fn create_wradius_bind_group_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("World Radius Bind Group Layout"),
    })
}
