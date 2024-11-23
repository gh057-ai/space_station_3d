use ash::vk;
use gpu_allocator::vulkan::{Allocation, AllocationCreateDesc, AllocationScheme, Allocator};
use glam::{Vec3, Vec4};

#[derive(Debug)]
pub struct Material {
    pub albedo: Vec4,
    pub metallic: f32,
    pub roughness: f32,
    pub alpha: f32,
    pub emissive: Vec3,
    pub normal_scale: f32,
    pub occlusion_strength: f32,
    pub alpha_cutoff: f32,
    pub double_sided: bool,
    pub buffer: Option<vk::Buffer>,
    pub allocation: Option<Allocation>,
}

impl Material {
    pub fn new(albedo: Vec4, metallic: f32, roughness: f32, alpha: f32) -> Self {
        Self {
            albedo,
            metallic,
            roughness,
            alpha,
            emissive: Vec3::ZERO,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            alpha_cutoff: 0.5,
            double_sided: false,
            buffer: None,
            allocation: None,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: Vec4::ONE,
            metallic: 0.0,
            roughness: 0.5,
            alpha: 1.0,
            emissive: Vec3::ZERO,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            alpha_cutoff: 0.5,
            double_sided: false,
            buffer: None,
            allocation: None,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MaterialUBO {
    pub albedo: Vec4,
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: Vec3,
    pub normal_scale: f32,
    pub occlusion_strength: f32,
    pub alpha_cutoff: f32,
    pub double_sided: u32,
}

impl Material {
    pub fn create_metal(color: Vec3) -> Self {
        Self {
            albedo: Vec4::new(color.x, color.y, color.z, 1.0),
            metallic: 1.0,
            roughness: 0.1,
            alpha: 1.0,
            emissive: Vec3::ZERO,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            alpha_cutoff: 0.5,
            double_sided: false,
            buffer: None,
            allocation: None,
        }
    }

    pub fn create_plastic(color: Vec3) -> Self {
        Self {
            albedo: Vec4::new(color.x, color.y, color.z, 1.0),
            metallic: 0.0,
            roughness: 0.5,
            alpha: 1.0,
            emissive: Vec3::ZERO,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            alpha_cutoff: 0.5,
            double_sided: false,
            buffer: None,
            allocation: None,
        }
    }

    pub fn create_glass(color: Vec3, alpha: f32) -> Self {
        Self {
            albedo: Vec4::new(color.x, color.y, color.z, alpha),
            metallic: 0.0,
            roughness: 0.1,
            alpha,
            emissive: Vec3::ZERO,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            alpha_cutoff: 0.5,
            double_sided: false,
            buffer: None,
            allocation: None,
        }
    }

    pub fn create_buffer(
        &mut self,
        device: &ash::Device,
        allocator: &mut Allocator,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let buffer_size = std::mem::size_of::<MaterialUBO>();

        let buffer_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            size: buffer_size as u64,
            usage: vk::BufferUsageFlags::UNIFORM_BUFFER,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };

        let buffer = unsafe { device.create_buffer(&buffer_info, None)? };
        let requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

        let allocation = allocator.allocate(&AllocationCreateDesc {
            name: "Material Buffer",
            requirements,
            location: gpu_allocator::MemoryLocation::CpuToGpu,
            linear: true,
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })?;

        unsafe {
            device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())?;
        }

        // Update buffer contents
        let ubo = MaterialUBO {
            albedo: self.albedo,
            metallic: self.metallic,
            roughness: self.roughness,
            emissive: self.emissive,
            normal_scale: self.normal_scale,
            occlusion_strength: self.occlusion_strength,
            alpha_cutoff: self.alpha_cutoff,
            double_sided: self.double_sided as u32,
        };

        let data_ptr = allocation.mapped_ptr().unwrap().as_ptr() as *mut MaterialUBO;
        unsafe {
            data_ptr.write(ubo);
        }

        self.buffer = Some(buffer);
        self.allocation = Some(allocation);

        Ok(())
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Self {
            albedo: self.albedo,
            metallic: self.metallic,
            roughness: self.roughness,
            alpha: self.alpha,
            emissive: self.emissive,
            normal_scale: self.normal_scale,
            occlusion_strength: self.occlusion_strength,
            alpha_cutoff: self.alpha_cutoff,
            double_sided: self.double_sided,
            buffer: self.buffer,
            allocation: None, // We don't clone the allocation
        }
    }
}

impl Drop for Material {
    fn drop(&mut self) {
        if let (Some(_buffer), Some(_allocation)) = (self.buffer.take(), self.allocation.take()) {
            // Buffer and allocation cleanup should be handled by the renderer
        }
    }
}
