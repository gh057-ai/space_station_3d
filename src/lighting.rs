use glam::{Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub shininess: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LightingUBO {
    pub lights: [Light; 4],
    pub material: Material,
    pub view_pos: Vec3,
}

impl LightingUBO {
    pub fn new() -> Self {
        Self {
            lights: [
                Light {
                    position: Vec3::new(0.0, 0.0, 2.0),
                    color: Vec3::new(1.0, 1.0, 1.0),
                    intensity: 1.0,
                },
                Light {
                    position: Vec3::new(2.0, 2.0, 2.0),
                    color: Vec3::new(1.0, 0.0, 0.0),
                    intensity: 0.5,
                },
                Light {
                    position: Vec3::new(-2.0, 2.0, 2.0),
                    color: Vec3::new(0.0, 1.0, 0.0),
                    intensity: 0.5,
                },
                Light {
                    position: Vec3::new(0.0, -2.0, 2.0),
                    color: Vec3::new(0.0, 0.0, 1.0),
                    intensity: 0.5,
                },
            ],
            material: Material {
                ambient: Vec3::new(0.1, 0.1, 0.1),
                diffuse: Vec3::new(0.7, 0.7, 0.7),
                specular: Vec3::new(1.0, 1.0, 1.0),
                shininess: 32.0,
            },
            view_pos: Vec3::new(0.0, 0.0, -3.0),
        }
    }
}

pub struct LightManager {
    pub lighting_ubo: LightingUBO,
}

impl LightManager {
    pub fn new() -> Self {
        Self {
            lighting_ubo: LightingUBO::new(),
        }
    }

    pub fn add_light(&mut self, light: Light) -> bool {
        if self.lighting_ubo.lights.iter().any(|l| l.position == light.position) {
            return false;
        }

        for l in self.lighting_ubo.lights.iter_mut() {
            if l.position == Vec3::ZERO {
                *l = light;
                return true;
            }
        }

        false
    }

    pub fn clear_lights(&mut self) {
        for l in self.lighting_ubo.lights.iter_mut() {
            *l = Light {
                position: Vec3::ZERO,
                color: Vec3::ZERO,
                intensity: 0.0,
            };
        }
    }

    pub fn get_light(&self, index: usize) -> Option<Light> {
        if index >= self.lighting_ubo.lights.len() {
            None
        } else {
            Some(self.lighting_ubo.lights[index])
        }
    }

    pub fn update_light(&mut self, index: usize, light: Light) -> bool {
        if index >= self.lighting_ubo.lights.len() {
            false
        } else {
            self.lighting_ubo.lights[index] = light;
            true
        }
    }

    pub fn remove_light(&mut self, index: usize) -> bool {
        if index >= self.lighting_ubo.lights.len() {
            return false;
        }

        self.lighting_ubo.lights[index] = Light {
            position: Vec3::ZERO,
            color: Vec3::ZERO,
            intensity: 0.0,
        };

        true
    }
}
