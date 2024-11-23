use glam::{Vec3, Vec4, Mat4};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ParticleEffect {
    pub effect_type: EffectType,
    pub start_time: Duration,
    pub duration: Duration,
    pub params: EffectParams,
}

#[derive(Debug, Clone, Copy)]
pub enum EffectType {
    Glow,
    Trail,
    Shockwave,
    ElectricArc,
    Distortion,
    VolumetricLight,
    Portal,
    BlackHole,
    TimeRift,
    HologramGlitch,
}

#[derive(Debug, Clone)]
pub struct EffectParams {
    pub color: Vec4,
    pub size: f32,
    pub intensity: f32,
    pub speed: f32,
    pub noise_scale: f32,
    pub distortion_strength: f32,
    pub transform: Mat4,
}

impl Default for EffectParams {
    fn default() -> Self {
        Self {
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            size: 1.0,
            intensity: 1.0,
            speed: 1.0,
            noise_scale: 1.0,
            distortion_strength: 0.0,
            transform: Mat4::IDENTITY,
        }
    }
}

#[derive(Debug)]
pub struct EffectRenderer {
    pub time: Duration,
    pub camera_position: Vec3,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
}

impl EffectRenderer {
    pub fn render_effect(&self, effect: &ParticleEffect, position: Vec3) -> EffectRenderData {
        let elapsed = (self.time - effect.start_time).as_secs_f32();
        let progress = (elapsed / effect.duration.as_secs_f32()).min(1.0);

        match effect.effect_type {
            EffectType::Glow => self.render_glow(effect, position, progress),
            EffectType::Trail => self.render_trail(effect, position, progress),
            EffectType::Shockwave => self.render_shockwave(effect, position, progress),
            EffectType::ElectricArc => self.render_electric_arc(effect, position, progress),
            EffectType::Distortion => self.render_distortion(effect, position, progress),
            EffectType::VolumetricLight => self.render_volumetric_light(effect, position, progress),
            EffectType::Portal => self.render_portal(effect, position, progress),
            EffectType::BlackHole => self.render_black_hole(effect, position, progress),
            EffectType::TimeRift => self.render_time_rift(effect, position, progress),
            EffectType::HologramGlitch => self.render_hologram_glitch(effect, position, progress),
        }
    }

    fn render_glow(&self, effect: &ParticleEffect, position: Vec3, progress: f32) -> EffectRenderData {
        let params = &effect.params;
        let size = params.size * (1.0 + progress * 0.5);
        let alpha = (1.0 - progress) * params.intensity;
        
        EffectRenderData {
            color: params.color * alpha,
            size,
            transform: Mat4::from_translation(position) * Mat4::from_scale(Vec3::splat(size)),
            uv_offset: Vec3::new(0.0, 0.0, 0.0),
            distortion: 0.0,
            noise: generate_noise(position, self.time.as_secs_f32(), params.noise_scale),
        }
    }

    fn render_trail(&self, effect: &ParticleEffect, position: Vec3, progress: f32) -> EffectRenderData {
        let params = &effect.params;
        let trail_length = params.size * (1.0 - progress);
        let alpha = (1.0 - progress) * params.intensity;
        
        EffectRenderData {
            color: params.color * alpha,
            size: params.size,
            transform: calculate_trail_transform(position, trail_length, params.speed),
            uv_offset: Vec3::new(progress, 0.0, 0.0),
            distortion: params.distortion_strength * progress,
            noise: generate_noise(position, self.time.as_secs_f32(), params.noise_scale),
        }
    }

    fn render_shockwave(&self, effect: &ParticleEffect, position: Vec3, progress: f32) -> EffectRenderData {
        let params = &effect.params;
        let wave_radius = params.size * progress * 2.0;
        let thickness = (1.0 - progress) * 0.2;
        let alpha = (1.0 - progress) * params.intensity;

        EffectRenderData {
            color: params.color * alpha,
            size: wave_radius,
            transform: Mat4::from_translation(position) * Mat4::from_scale(Vec3::splat(wave_radius)),
            uv_offset: Vec3::new(0.0, thickness, 0.0),
            distortion: params.distortion_strength * (1.0 - progress),
            noise: generate_noise(position, self.time.as_secs_f32(), params.noise_scale),
        }
    }

    fn render_electric_arc(&self, effect: &ParticleEffect, position: Vec3, progress: f32) -> EffectRenderData {
        let params = &effect.params;
        let arc_progress = (progress * std::f32::consts::TAU).sin() * 0.5 + 0.5;
        let intensity = (1.0 - progress) * params.intensity;
        
        EffectRenderData {
            color: params.color * intensity,
            size: params.size,
            transform: Mat4::from_translation(position),
            uv_offset: Vec3::new(arc_progress, 0.0, 0.0),
            distortion: params.distortion_strength * arc_progress,
            noise: generate_noise(position, self.time.as_secs_f32() * params.speed, params.noise_scale),
        }
    }

    fn render_distortion(&self, effect: &ParticleEffect, position: Vec3, progress: f32) -> EffectRenderData {
        let params = &effect.params;
        let distortion_size = params.size * (1.0 + progress * 0.5);
        let distortion_strength = params.distortion_strength * (1.0 - progress);
        
        EffectRenderData {
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            size: distortion_size,
            transform: Mat4::from_translation(position) * Mat4::from_scale(Vec3::splat(distortion_size)),
            uv_offset: Vec3::ZERO,
            distortion: distortion_strength,
            noise: generate_noise(position, self.time.as_secs_f32(), params.noise_scale),
        }
    }

    fn render_volumetric_light(&self, effect: &ParticleEffect, position: Vec3, progress: f32) -> EffectRenderData {
        let params = &effect.params;
        let light_intensity = params.intensity * (1.0 - progress);
        let light_size = params.size * (1.0 + progress * 0.3);
        
        EffectRenderData {
            color: params.color * light_intensity,
            size: light_size,
            transform: calculate_volumetric_transform(position, self.camera_position, light_size),
            uv_offset: Vec3::new(0.0, 0.0, progress),
            distortion: params.distortion_strength * progress,
            noise: generate_noise(position, self.time.as_secs_f32(), params.noise_scale),
        }
    }

    fn render_portal(&self, effect: &ParticleEffect, position: Vec3, progress: f32) -> EffectRenderData {
        let params = &effect.params;
        let portal_size = params.size * (1.0 + (progress * std::f32::consts::TAU).sin() * 0.1);
        let rotation = Mat4::from_rotation_z(progress * std::f32::consts::TAU);
        
        EffectRenderData {
            color: params.color,
            size: portal_size,
            transform: Mat4::from_translation(position) * rotation * Mat4::from_scale(Vec3::splat(portal_size)),
            uv_offset: Vec3::new(progress, 0.0, 0.0),
            distortion: params.distortion_strength * (1.0 - progress.powi(2)),
            noise: generate_noise(position, self.time.as_secs_f32() * params.speed, params.noise_scale),
        }
    }

    fn render_black_hole(&self, effect: &ParticleEffect, position: Vec3, progress: f32) -> EffectRenderData {
        let params = &effect.params;
        let hole_size = params.size * (1.0 - progress * 0.5);
        let distortion = params.distortion_strength * (1.0 + progress);
        
        EffectRenderData {
            color: Vec4::new(0.0, 0.0, 0.0, 1.0),
            size: hole_size,
            transform: Mat4::from_translation(position) * Mat4::from_scale(Vec3::splat(hole_size)),
            uv_offset: Vec3::ZERO,
            distortion,
            noise: generate_noise(position, self.time.as_secs_f32(), params.noise_scale),
        }
    }

    fn render_time_rift(&self, effect: &ParticleEffect, position: Vec3, progress: f32) -> EffectRenderData {
        let params = &effect.params;
        let rift_size = params.size * (1.0 + (progress * std::f32::consts::PI * 2.0).sin() * 0.2);
        let time_distortion = params.distortion_strength * (1.0 - progress);
        
        EffectRenderData {
            color: params.color * (1.0 - progress),
            size: rift_size,
            transform: calculate_time_rift_transform(position, rift_size, self.time.as_secs_f32()),
            uv_offset: Vec3::new(progress, 0.0, self.time.as_secs_f32() * params.speed),
            distortion: time_distortion,
            noise: generate_noise(position, self.time.as_secs_f32() * 2.0, params.noise_scale),
        }
    }

    fn render_hologram_glitch(&self, effect: &ParticleEffect, position: Vec3, progress: f32) -> EffectRenderData {
        let params = &effect.params;
        let glitch_intensity = ((progress * 20.0).sin() * 0.5 + 0.5) * params.intensity;
        let glitch_offset = Vec3::new(
            (progress * 7.0).sin() * 0.1,
            (progress * 5.0).cos() * 0.1,
            0.0
        );
        
        EffectRenderData {
            color: params.color * glitch_intensity,
            size: params.size,
            transform: Mat4::from_translation(position + glitch_offset) * params.transform,
            uv_offset: glitch_offset,
            distortion: params.distortion_strength * glitch_intensity,
            noise: generate_noise(position, self.time.as_secs_f32() * params.speed, params.noise_scale),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EffectRenderData {
    pub color: Vec4,
    pub size: f32,
    pub transform: Mat4,
    pub uv_offset: Vec3,
    pub distortion: f32,
    pub noise: f32,
}

// Helper functions
fn generate_noise(position: Vec3, time: f32, scale: f32) -> f32 {
    use noise::{NoiseFn, Perlin};
    let noise = Perlin::new(0);
    noise.get([
        position.x as f64 * scale as f64,
        position.y as f64 * scale as f64,
        time as f64
    ]) as f32
}

fn calculate_trail_transform(position: Vec3, length: f32, speed: f32) -> Mat4 {
    let scale = Vec3::new(length, 1.0, 1.0);
    Mat4::from_translation(position) * Mat4::from_scale(scale)
}

fn calculate_volumetric_transform(position: Vec3, camera_pos: Vec3, size: f32) -> Mat4 {
    let to_camera = (camera_pos - position).normalize();
    let right = to_camera.cross(Vec3::Y).normalize();
    let up = right.cross(to_camera);
    
    Mat4::from_cols(
        right.extend(0.0),
        up.extend(0.0),
        to_camera.extend(0.0),
        position.extend(1.0)
    ) * Mat4::from_scale(Vec3::splat(size))
}

fn calculate_time_rift_transform(position: Vec3, size: f32, time: f32) -> Mat4 {
    let rotation = Mat4::from_rotation_z(time * 0.5)
        * Mat4::from_rotation_y(time * 0.3)
        * Mat4::from_rotation_x(time * 0.2);
    
    Mat4::from_translation(position) 
        * rotation 
        * Mat4::from_scale(Vec3::splat(size))
}
