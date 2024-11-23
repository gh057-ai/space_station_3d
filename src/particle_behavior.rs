use glam::Vec3;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BehaviorType {
    Flock,
    Swarm,
    Vortex,
    Attractor,
    Repulsor,
    PathFollow,
    Obstacle,
    Leader,
    Predator,
    Prey,
}

#[derive(Debug, Clone)]
pub struct BehaviorParams {
    pub weight: f32,
    pub radius: f32,
    pub strength: f32,
    pub params: HashMap<String, f32>,
}

impl Default for BehaviorParams {
    fn default() -> Self {
        Self {
            weight: 1.0,
            radius: 5.0,
            strength: 1.0,
            params: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct FlockingBehavior {
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub perception_radius: f32,
    pub max_speed: f32,
    pub max_force: f32,
}

impl Default for FlockingBehavior {
    fn default() -> Self {
        Self {
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
            perception_radius: 5.0,
            max_speed: 10.0,
            max_force: 0.5,
        }
    }
}

impl FlockingBehavior {
    pub fn calculate_forces(&self, position: Vec3, velocity: Vec3, neighbors: &[(Vec3, Vec3)]) -> Vec3 {
        let mut separation = Vec3::ZERO;
        let mut alignment = Vec3::ZERO;
        let mut cohesion = Vec3::ZERO;
        let mut total = 0;

        for &(pos, vel) in neighbors {
            let distance = position.distance(pos);
            if distance > 0.0 && distance < self.perception_radius {
                // Separation
                let diff = (position - pos).normalize() / distance;
                separation += diff;

                // Alignment
                alignment += vel;

                // Cohesion
                cohesion += pos;

                total += 1;
            }
        }

        if total > 0 {
            // Average and apply weights
            separation = (separation / total as f32) * self.separation_weight;
            alignment = ((alignment / total as f32) - velocity) * self.alignment_weight;
            cohesion = ((cohesion / total as f32) - position) * self.cohesion_weight;

            // Limit forces
            separation = limit_vector(separation, self.max_force);
            alignment = limit_vector(alignment, self.max_force);
            cohesion = limit_vector(cohesion, self.max_force);
        }

        separation + alignment + cohesion
    }
}

#[derive(Debug)]
pub struct SwarmBehavior {
    pub attraction_point: Vec3,
    pub attraction_strength: f32,
    pub repulsion_radius: f32,
    pub swarm_radius: f32,
    pub noise_scale: f32,
    pub time_scale: f32,
}

impl SwarmBehavior {
    pub fn calculate_force(&self, position: Vec3, time: f32) -> Vec3 {
        let to_center = self.attraction_point - position;
        let distance = to_center.length();

        // Attraction to center
        let mut force = if distance > self.swarm_radius {
            to_center.normalize() * self.attraction_strength
        } else {
            Vec3::ZERO
        };

        // Repulsion from center
        if distance < self.repulsion_radius {
            force -= to_center.normalize() * (1.0 - distance / self.repulsion_radius);
        }

        // Add noise for natural movement
        let noise = simplex_noise_3d(
            position.x * self.noise_scale,
            position.y * self.noise_scale,
            time * self.time_scale,
        );
        force += Vec3::new(
            noise.0 as f32,
            noise.1 as f32,
            noise.2 as f32,
        ) * 0.5;

        force
    }
}

#[derive(Debug)]
pub struct VortexBehavior {
    pub center: Vec3,
    pub axis: Vec3,
    pub strength: f32,
    pub radius: f32,
    pub height_influence: f32,
    pub upward_force: f32,
}

impl VortexBehavior {
    pub fn calculate_force(&self, position: Vec3) -> Vec3 {
        let to_center = position - self.center;
        let distance = to_center.length_squared();
        
        if distance < self.radius * self.radius {
            let tangent = to_center.cross(self.axis).normalize();
            let vertical = self.axis * self.height_influence;
            
            // Calculate vortex force
            let force = tangent * self.strength / distance.sqrt() +
                       vertical * self.upward_force;
            
            force
        } else {
            Vec3::ZERO
        }
    }
}

#[derive(Debug)]
pub struct PathFollowBehavior {
    pub path: Vec<Vec3>,
    pub loop_path: bool,
    pub path_radius: f32,
    pub look_ahead: f32,
    pub arrival_threshold: f32,
}

impl PathFollowBehavior {
    pub fn calculate_force(&self, position: Vec3, velocity: Vec3) -> Vec3 {
        if self.path.is_empty() {
            return Vec3::ZERO;
        }

        // Find the closest point on the path
        let mut closest_point = self.path[0];
        let mut closest_dist = f32::MAX;
        let mut target_index = 0;

        for (i, &point) in self.path.iter().enumerate() {
            let dist = position.distance(point);
            if dist < closest_dist {
                closest_dist = dist;
                closest_point = point;
                target_index = i;
            }
        }

        // Look ahead on the path
        let look_ahead_index = (target_index + 1) % self.path.len();
        let target = if look_ahead_index < self.path.len() {
            self.path[look_ahead_index]
        } else if self.loop_path {
            self.path[0]
        } else {
            return Vec3::ZERO;
        };

        // Calculate desired velocity
        let to_target = target - position;
        let distance = to_target.length();

        if distance < self.arrival_threshold {
            // Slow down as we approach the target
            to_target * (distance / self.arrival_threshold)
        } else {
            to_target.normalize() * self.look_ahead
        }
    }
}

pub struct PredatorBehavior {
    pub perception_radius: f32,
    pub chase_speed: f32,
    pub attack_radius: f32,
    pub rest_time: f32,
    pub energy: f32,
}

impl PredatorBehavior {
    pub fn calculate_force(&self, position: Vec3, prey_positions: &[Vec3]) -> Vec3 {
        let mut closest_prey = None;
        let mut min_distance = f32::MAX;

        // Find closest prey
        for &prey_pos in prey_positions {
            let distance = position.distance(prey_pos);
            if distance < self.perception_radius && distance < min_distance {
                min_distance = distance;
                closest_prey = Some(prey_pos);
            }
        }

        if let Some(prey_pos) = closest_prey {
            let to_prey = prey_pos - position;
            if min_distance < self.attack_radius {
                // Attack speed
                to_prey.normalize() * self.chase_speed * 1.5
            } else {
                // Chase speed
                to_prey.normalize() * self.chase_speed
            }
        } else {
            // Wander when no prey is visible
            random_direction() * self.chase_speed * 0.5
        }
    }
}

// Helper functions
fn limit_vector(v: Vec3, max_length: f32) -> Vec3 {
    let length = v.length();
    if length > max_length {
        v * (max_length / length)
    } else {
        v
    }
}

fn random_direction() -> Vec3 {
    let theta = rand::random::<f32>() * std::f32::consts::TAU;
    let phi = rand::random::<f32>() * std::f32::consts::PI;
    Vec3::new(
        phi.sin() * theta.cos(),
        phi.sin() * theta.sin(),
        phi.cos()
    )
}

// Simplex noise implementation (simplified for example)
fn simplex_noise_3d(x: f32, y: f32, z: f32) -> (f64, f64, f64) {
    use noise::{NoiseFn, Simplex};
    let noise = Simplex::new(0);
    (
        noise.get([x as f64, y as f64, z as f64]),
        noise.get([x as f64 + 100.0, y as f64 + 100.0, z as f64]),
        noise.get([x as f64 + 200.0, y as f64 + 200.0, z as f64])
    )
}
