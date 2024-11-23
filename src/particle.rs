use std::time::Duration;
use glam::Vec3;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ParticleType {
    #[default]
    Debris,
    Smoke,
    Fire,
    Spark,
    Glow,
    Flash,
    Shockwave,
    ElectricArc,
    TimeDistortion,
    PlasmaFlow,
    IonicDischarge,
    QuantumFluctuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ParticleEffectType {
    #[default]
    Fade,
    ColorShift,
    Scale,
    Glow,
    Flash,
    Trail,
    Shockwave,
    ElectricArc,
    TimeDistortion,
}

#[derive(Debug, Clone, Default)]
pub struct ParticleEffect {
    pub effect_type: ParticleEffectType,
    pub duration: Duration,
    pub elapsed: Duration,
    pub parameters: HashMap<String, f32>,
}

impl ParticleEffect {
    pub fn update(&mut self, dt: f32) {
        self.elapsed += Duration::from_secs_f32(dt);
        match self.effect_type {
            ParticleEffectType::Fade => {
                // Implement fade effect
            }
            ParticleEffectType::ColorShift => {
                // Implement color shifting based on elapsed time
            }
            ParticleEffectType::Scale => {
                // Implement scaling based on elapsed time
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParticleConfig {
    pub position: Vec3,
    pub direction: Vec3,
    pub spread_angle: f32,
    pub speed: f32,
    pub size: f32,
    pub color: Vec3,
    pub particle_lifetime: Duration,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            direction: Vec3::new(0.0, 1.0, 0.0),
            spread_angle: 45.0,
            speed: 1.0,
            size: 1.0,
            color: Vec3::ONE,
            particle_lifetime: Duration::from_secs(1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub size: f32,
    pub color: Vec3,
    pub opacity: f32,
    pub rotation: f32,
    pub lifetime: Duration,
    pub age: Duration,
    pub particle_type: ParticleType,
    pub effects: Vec<ParticleEffect>,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            size: 1.0,
            color: Vec3::ONE,
            opacity: 1.0,
            rotation: 0.0,
            lifetime: Duration::from_secs(1),
            age: Duration::from_secs(0),
            particle_type: ParticleType::Debris,
            effects: Vec::new(),
        }
    }
}

impl Particle {
    pub fn new(config: ParticleConfig) -> Self {
        Self {
            position: config.position,
            velocity: config.direction * config.speed,
            acceleration: Vec3::ZERO,
            size: config.size,
            color: config.color,
            opacity: 1.0,
            rotation: 0.0,
            lifetime: config.particle_lifetime,
            age: Duration::ZERO,
            particle_type: ParticleType::Debris, // Default type
            effects: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self.particle_type {
            ParticleType::Debris => {
                self.position += self.velocity * dt;
                self.velocity *= 0.99; // Apply drag
                self.lifetime = self.lifetime.saturating_sub(Duration::from_secs_f32(dt));
            }
            ParticleType::Smoke => {
                self.position += self.velocity * dt;
                self.velocity.y += 0.1 * dt; // Smoke rises
                self.size *= 1.01; // Smoke expands
                self.lifetime = self.lifetime.saturating_sub(Duration::from_secs_f32(dt));
            }
            ParticleType::Fire => {
                self.position += self.velocity * dt;
                self.velocity.y += 0.2 * dt; // Fire rises faster
                self.size *= 0.99; // Fire shrinks
                self.lifetime = self.lifetime.saturating_sub(Duration::from_secs_f32(dt));
            }
            ParticleType::Spark => {
                self.position += self.velocity * dt;
                self.velocity.y -= 0.5 * dt; // Gravity
                self.size *= 0.98; // Sparks shrink
                self.lifetime = self.lifetime.saturating_sub(Duration::from_secs_f32(dt));
            }
            ParticleType::Glow => {
                self.position += self.velocity * dt;
                self.size *= 0.95; // Glow fades quickly
                self.lifetime = self.lifetime.saturating_sub(Duration::from_secs_f32(dt));
            }
            ParticleType::Flash => {
                self.position += self.velocity * dt;
                self.size *= 0.95; // Flash fades quickly
                self.lifetime = self.lifetime.saturating_sub(Duration::from_secs_f32(dt));
            }
            ParticleType::Shockwave => {
                self.position += self.velocity * dt;
                self.size *= 1.1; // Shockwave expands
                self.lifetime = self.lifetime.saturating_sub(Duration::from_secs_f32(dt));
            }
            ParticleType::ElectricArc => {
                self.position += self.velocity * dt;
                self.rotation += dt * 10.0; // Arc rotates
                self.lifetime = self.lifetime.saturating_sub(Duration::from_secs_f32(dt));
            }
            ParticleType::TimeDistortion => {
                self.position += self.velocity * dt * 0.5; // Slower movement
                self.size = 1.0 + (self.lifetime.as_secs_f32() * 2.0).sin(); // Pulsing effect
                self.lifetime = self.lifetime.saturating_sub(Duration::from_secs_f32(dt));
            }
            _ => {
                // Default update behavior for other particle types
                self.position += self.velocity * dt;
                self.lifetime = self.lifetime.saturating_sub(Duration::from_secs_f32(dt));
            }
        }

        // Update effects
        for effect in &mut self.effects {
            effect.update(dt);
        }

        // Remove expired effects
        self.effects.retain(|effect| effect.elapsed < effect.duration);
    }
}

#[derive(Debug, Clone, Default)]
pub enum EmissionPattern {
    #[default]
    Point,
    Sphere { radius: f32 },
    Cone { radius: f32, height: f32 },
    Ring { radius: f32, count: u32 },
    Spiral { radius: f32, height: f32, rotations: f32 },
    Burst { radius: f32, angle_offset: f32 },
}

pub struct ParticleEmitter {
    pub position: Vec3,
    pub direction: Vec3,
    pub spread_angle: f32,
    pub emission_rate: f32,
    pub particle_type: ParticleType,
    pub particles: Vec<Particle>,
    pub emission_pattern: EmissionPattern,
    pub age: Duration,
    pub max_particles: usize,
    pub initial_velocity: f32,
    pub particle_size: f32,
    pub particle_lifetime: Duration,
    pub emit_timer: Duration,
    pub emission_interval: Duration,
}

impl ParticleEmitter {
    pub fn builder() -> ParticleEmitterBuilder {
        ParticleEmitterBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct ParticleEmitterBuilder {
    position: Vec3,
    direction: Vec3,
    spread_angle: f32,
    emission_rate: f32,
    particle_type: ParticleType,
    emission_pattern: EmissionPattern,
    initial_velocity: f32,
    particle_size: f32,
    particle_lifetime: Duration,
}

impl ParticleEmitterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn direction(mut self, direction: Vec3) -> Self {
        self.direction = direction;
        self
    }

    pub fn spread_angle(mut self, spread_angle: f32) -> Self {
        self.spread_angle = spread_angle;
        self
    }

    pub fn emission_rate(mut self, emission_rate: f32) -> Self {
        self.emission_rate = emission_rate;
        self
    }

    pub fn particle_type(mut self, particle_type: ParticleType) -> Self {
        self.particle_type = particle_type;
        self
    }

    pub fn emission_pattern(mut self, emission_pattern: EmissionPattern) -> Self {
        self.emission_pattern = emission_pattern;
        self
    }

    pub fn initial_velocity(mut self, initial_velocity: f32) -> Self {
        self.initial_velocity = initial_velocity;
        self
    }

    pub fn particle_size(mut self, particle_size: f32) -> Self {
        self.particle_size = particle_size;
        self
    }

    pub fn particle_lifetime(mut self, particle_lifetime: Duration) -> Self {
        self.particle_lifetime = particle_lifetime;
        self
    }

    pub fn build(self) -> ParticleEmitter {
        ParticleEmitter {
            position: self.position,
            direction: self.direction,
            spread_angle: self.spread_angle,
            emission_rate: self.emission_rate,
            particle_type: self.particle_type,
            emission_pattern: self.emission_pattern,
            initial_velocity: self.initial_velocity,
            particle_size: self.particle_size,
            particle_lifetime: self.particle_lifetime,
            particles: Vec::new(),
            age: Duration::from_secs(0),
            max_particles: 100,
            emit_timer: Duration::from_secs(0),
            emission_interval: Duration::from_secs_f32(1.0),
        }
    }
}

impl ParticleEmitter {
    pub fn update(&mut self, dt: f32) {
        // Update emission timer
        self.emit_timer += Duration::from_secs_f32(dt);
        if self.emit_timer >= self.emission_interval {
            self.emit_timer = Duration::ZERO;
            self.emit();
        }

        // Update age
        self.age += Duration::from_secs_f32(dt);

        // Update emitter behavior based on particle type
        match self.particle_type {
            ParticleType::Fire => {
                self.emission_rate *= 1.0 + 0.2 * (self.age.as_secs_f32() * 5.0).sin();
            }
            ParticleType::Smoke => {
                self.direction += Vec3::new(
                    0.1 * (self.age.as_secs_f32() * 0.5).sin(),
                    0.1 * (self.age.as_secs_f32() * 0.7).cos(),
                    0.1 * (self.age.as_secs_f32() * 0.3).sin(),
                );
                self.direction = self.direction.normalize();
            }
            _ => {}
        }

        // Update all particles
        self.particles.retain_mut(|particle| {
            particle.update(dt);
            particle.age < particle.lifetime
        });
    }

    pub fn emit(&mut self) {
        if self.particles.len() >= 100 {
            return;
        }

        let spawn_pos = match &self.emission_pattern {
            EmissionPattern::Point => self.position,
            EmissionPattern::Sphere { radius } => {
                let direction = random_direction();
                self.position + direction * *radius
            }
            EmissionPattern::Cone { radius, height } => {
                let t = self.age.as_secs_f32();
                let angle = t * std::f32::consts::TAU;
                let r = radius * (1.0 - t.cos() * 0.2);
                let x = angle.cos() * r;
                let y = height * t;
                let z = angle.sin() * r;
                self.position + Vec3::new(x, y, z)
            }
            EmissionPattern::Ring { radius, count } => {
                let index = (self.particles.len() % *count as usize) as f32;
                let angle = index * std::f32::consts::TAU / *count as f32;
                self.position + Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius)
            }
            EmissionPattern::Spiral { radius, height, rotations } => {
                let t = (self.age.as_secs_f32() % 10.0) / 10.0;
                let angle = t * std::f32::consts::TAU * rotations;
                let r = radius * t;
                let x = angle.cos() * r;
                let y = height * t;
                let z = angle.sin() * r;
                self.position + Vec3::new(x, y, z)
            }
            EmissionPattern::Burst { radius, angle_offset } => {
                self.position + Vec3::new(
                    angle_offset.cos() * radius,
                    0.0,
                    angle_offset.sin() * radius
                )
            }
        };

        let particle = Particle::new(ParticleConfig {
            position: spawn_pos,
            direction: self.direction,
            spread_angle: self.spread_angle,
            speed: self.initial_velocity,
            size: self.particle_size,
            color: Vec3::ONE,
            particle_lifetime: self.particle_lifetime,
        });

        self.particles.push(particle);
    }
}

fn random_direction() -> Vec3 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let theta = rng.gen_range(0.0..std::f32::consts::TAU);
    let phi = rng.gen_range(0.0..std::f32::consts::PI);
    Vec3::new(
        theta.cos() * phi.sin(),
        phi.cos(),
        theta.sin() * phi.sin()
    )
}
