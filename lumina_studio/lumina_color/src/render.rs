use wgpu;
use std::borrow::Cow;

pub struct ColorComputePipeline {
    pub pipeline: wgpu::ComputePipeline,
}

impl ColorComputePipeline {
    pub fn new(device: &wgpu::Device) -> Self {
        // We embed the WGSL at compile time so it is always packaged in the binary
        let shader_src = include_str!("shaders/color_grade.wgsl");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Color Grading Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_src)),
        });

        // We must define the bind group layout that matches our WGSL bindings:
        // @binding(0) texture_2d
        // @binding(1) texture_storage_2d
        // @binding(2) uniform GradingParameters
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Color Grading Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(80), // 5 vec4<f32> = 5 * 16 bytes = 80 bytes
                    },
                    count: None,
                }
            ],
        });

        let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Color Grading Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Color Grading Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: "color_grade_cs",
        });

        Self { pipeline }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_compilation() {
        pollster::block_on(async {
            let instance = wgpu::Instance::default();
            let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: None,
            }).await;

            if let Some(adapter) = adapter {
                let (device, _queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();
                let _pipeline = ColorComputePipeline::new(&device);
            }
        });
    }
}
