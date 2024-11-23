use ash::vk;
use glam::{Vec3, Vec4};
use gpu_allocator::vulkan::{Allocation, AllocationCreateDesc, AllocationScheme, Allocator};
use std::sync::Arc;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LightUBO {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

#[derive(Debug)]
pub struct LightBuffer {
    buffer: vk::Buffer,
    allocation: Option<Allocation>,
    device: Arc<ash::Device>,
}

impl LightBuffer {
    pub fn new(
        device: Arc<ash::Device>,
        allocator: &mut Allocator,
        buffer_size: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let buffer_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size: buffer_size as u64,
            usage: vk::BufferUsageFlags::UNIFORM_BUFFER,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: std::ptr::null(),
        };

        let buffer = unsafe { device.create_buffer(&buffer_info, None)? };

        let requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

        let allocation = allocator.allocate(&AllocationCreateDesc {
            name: "Light Buffer",
            requirements,
            location: gpu_allocator::MemoryLocation::CpuToGpu,
            linear: true,
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })?;

        unsafe {
            device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())?;
        }

        Ok(Self {
            buffer,
            allocation: Some(allocation),
            device,
        })
    }

    pub fn cleanup(&mut self, allocator: &mut Allocator) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(allocation) = self.allocation.take() {
            allocator.free(allocation)?;
        }
        unsafe {
            self.device.destroy_buffer(self.buffer, None);
        }
        Ok(())
    }
}

impl Drop for LightBuffer {
    fn drop(&mut self) {
        if self.allocation.is_some() {
            eprintln!("Warning: LightBuffer dropped without calling cleanup()");
        }
    }
}

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    buffer: Option<LightBuffer>,
    device: Arc<ash::Device>,
}

impl Light {
    pub fn new(
        device: Arc<ash::Device>,
        position: Vec3,
        color: Vec3,
        intensity: f32,
    ) -> Self {
        Self {
            position,
            color,
            intensity,
            buffer: None,
            device,
        }
    }

    pub fn create_point_light(position: Vec3, color: Vec3, intensity: f32, device: Arc<ash::Device>) -> Self {
        Self::new(position, color, intensity, device)
    }

    pub fn create_white_light(position: Vec3, intensity: f32, device: Arc<ash::Device>) -> Self {
        Self::new(position, Vec3::new(1.0, 1.0, 1.0), intensity, device)
    }

    pub fn create_ambient_light(device: Arc<ash::Device>) -> Self {
        Self::new(
            Vec3::ZERO,
            Vec3::new(1.0, 1.0, 1.0),
            0.1, // Low intensity for ambient light
            device,
        )
    }

    pub fn to_ubo(&self) -> LightUBO {
        LightUBO {
            position: self.position,
            color: self.color,
            intensity: self.intensity,
        }
    }

    pub fn update(&mut self, allocator: &mut Allocator) -> Result<(), Box<dyn std::error::Error>> {
        if self.buffer.is_none() {
            let buffer = LightBuffer::new(
                self.device.clone(),
                allocator,
                std::mem::size_of::<LightUBO>(),
            )?;
            self.buffer = Some(buffer);
        }

        if let Some(buffer) = &self.buffer {
            if let Some(allocation) = &buffer.allocation {
                let light_ubo = self.to_ubo();
                unsafe {
                    let data_ptr = allocation.mapped_ptr().unwrap().as_ptr() as *mut LightUBO;
                    data_ptr.write(light_ubo);
                }
            }
        }

        Ok(())
    }

    pub fn cleanup(&mut self, allocator: &mut Allocator) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mut buffer) = self.buffer.take() {
            buffer.cleanup(allocator)?;
        }
        Ok(())
    }

    pub fn get_buffer(&self) -> Option<vk::Buffer> {
        self.buffer.as_ref().map(|b| b.buffer)
    }

    pub fn update_position(&mut self, position: Vec3) {
        self.position = position;
        self.update(self.device.clone());
    }
}

impl Drop for Light {
    fn drop(&mut self) {
        if self.buffer.is_some() {
            eprintln!("Warning: Light dropped without calling cleanup()");
        }
    }
}
