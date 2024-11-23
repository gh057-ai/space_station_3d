use glam::{Vec3, Vec2, Mat4};
use crate::vertex::Vertex;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn create_cylinder(radius: f32, height: f32, segments: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Create vertices for top and bottom circles
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * 2.0 * PI;
            let x = angle.cos() * radius;
            let z = angle.sin() * radius;

            // Bottom vertex
            vertices.push(Vertex::new(
                Vec3::new(x, 0.0, z).into(),
                Vec3::new(x, 0.0, z).normalize().into(),
                Vec2::new(i as f32 / segments as f32, 0.0).into(),
            ));

            // Top vertex
            vertices.push(Vertex::new(
                Vec3::new(x, height, z).into(),
                Vec3::new(x, 0.0, z).normalize().into(),
                Vec2::new(i as f32 / segments as f32, 1.0).into(),
            ));
        }

        // Create indices for the cylinder wall
        for i in 0..segments {
            let next = (i + 1) % segments;
            let base = i * 2;
            let next_base = next * 2;

            // First triangle
            indices.push(base);
            indices.push(base + 1);
            indices.push(next_base);

            // Second triangle
            indices.push(next_base);
            indices.push(base + 1);
            indices.push(next_base + 1);
        }

        // Create vertices and indices for top and bottom caps
        let center_bottom = vertices.len() as u32;
        vertices.push(Vertex::new(
            Vec3::new(0.0, 0.0, 0.0).into(),
            Vec3::new(0.0, -1.0, 0.0).into(),
            Vec2::new(0.5, 0.5).into(),
        ));

        let center_top = vertices.len() as u32;
        vertices.push(Vertex::new(
            Vec3::new(0.0, height, 0.0).into(),
            Vec3::new(0.0, 1.0, 0.0).into(),
            Vec2::new(0.5, 0.5).into(),
        ));

        // Create indices for caps
        for i in 0..segments {
            let next = (i + 1) % segments;

            // Bottom cap
            indices.push(center_bottom);
            indices.push(i * 2);
            indices.push(next * 2);

            // Top cap
            indices.push(center_top);
            indices.push((i * 2) + 1);
            indices.push((next * 2) + 1);
        }

        Self { vertices, indices }
    }

    pub fn create_sphere(radius: f32, segments: u32, rings: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Create vertices
        for ring in 0..=rings {
            let phi = (ring as f32 / rings as f32) * PI;
            for segment in 0..=segments {
                let theta = (segment as f32 / segments as f32) * 2.0 * PI;

                let x = phi.sin() * theta.cos();
                let y = phi.cos();
                let z = phi.sin() * theta.sin();

                let position = Vec3::new(x, y, z) * radius;
                let normal = Vec3::new(x, y, z).normalize();
                let uv = Vec2::new(
                    segment as f32 / segments as f32,
                    ring as f32 / rings as f32,
                );

                vertices.push(Vertex::new(position.into(), normal.into(), uv.into()));
            }
        }

        // Create indices
        for ring in 0..rings {
            for segment in 0..segments {
                let current = ring * (segments + 1) + segment;
                let next = current + (segments + 1);

                indices.push(current);
                indices.push(current + 1);
                indices.push(next);

                indices.push(current + 1);
                indices.push(next + 1);
                indices.push(next);
            }
        }

        Self { vertices, indices }
    }

    pub fn create_corridor_section(width: f32, length: f32, segments: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let height = width * 1.5; // Slightly taller than wide
        let corner_radius = width * 0.2; // Rounded corners
        
        // Create the main corridor shape with rounded corners
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * 2.0 * PI;
            
            // Create vertices for both ends of the corridor
            for z in &[0.0, length] {
                // Main rectangular part
                let base_x = width / 2.0 * angle.cos().signum();
                let base_y = height / 2.0 * angle.sin().signum();
                
                // Add rounded corners
                let corner_x = corner_radius * angle.cos();
                let corner_y = corner_radius * angle.sin();
                
                let x = if angle.cos().abs() > 0.707 {
                    base_x
                } else {
                    width / 2.0 - corner_radius + corner_x
                };
                
                let y = if angle.sin().abs() > 0.707 {
                    base_y
                } else {
                    height / 2.0 - corner_radius + corner_y
                };

                let position = Vec3::new(x, y, *z);
                let normal = Vec3::new(x, y, 0.0).normalize();
                let uv = Vec2::new(i as f32 / segments as f32, *z / length);
                
                vertices.push(Vertex::new(position.into(), normal.into(), uv.into()));
            }
        }

        // Create indices for the walls
        for i in 0..segments {
            let base = i * 2;
            let next_base = ((i + 1) % segments) * 2;
            
            // First triangle
            indices.push(base);
            indices.push(base + 1);
            indices.push(next_base);
            
            // Second triangle
            indices.push(next_base);
            indices.push(base + 1);
            indices.push(next_base + 1);
        }

        // Create end caps
        let center_front = vertices.len() as u32;
        vertices.push(Vertex::new(
            Vec3::new(0.0, 0.0, 0.0).into(),
            Vec3::new(0.0, 0.0, -1.0).into(),
            Vec2::new(0.5, 0.5).into(),
        ));

        let center_back = vertices.len() as u32;
        vertices.push(Vertex::new(
            Vec3::new(0.0, 0.0, length).into(),
            Vec3::new(0.0, 0.0, 1.0).into(),
            Vec2::new(0.5, 0.5).into(),
        ));

        // Create indices for end caps
        for i in 0..segments {
            let next = ((i + 1) % segments) * 2;
            
            // Front cap
            indices.push(center_front);
            indices.push(i * 2);
            indices.push(next);
            
            // Back cap
            indices.push(center_back);
            indices.push(i * 2 + 1);
            indices.push(next + 1);
        }

        Self { vertices, indices }
    }

    pub fn create_octagonal_room(width: f32, height: f32, depth: f32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let segments = 8; // Octagonal shape
        let _corner_ratio = 0.3; // How much of each wall is the corner segment

        // Create vertices for the main room
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * 2.0 * PI;
            let next_angle = ((i + 1) as f32 / segments as f32) * 2.0 * PI;
            
            // Calculate corner positions
            let corner_x = width / 2.0 * angle.cos();
            let corner_z = depth / 2.0 * angle.sin();
            
            let next_x = width / 2.0 * next_angle.cos();
            let next_z = depth / 2.0 * next_angle.sin();
            
            // Create vertices for the wall segment
            for y in &[0.0, height] {
                // First vertex of the wall
                vertices.push(Vertex::new(
                    Vec3::new(corner_x, *y, corner_z).into(),
                    Vec3::new(angle.cos(), 0.0, angle.sin()).normalize().into(),
                    Vec2::new(i as f32 / segments as f32, *y / height).into(),
                ));
                
                // Second vertex of the wall
                vertices.push(Vertex::new(
                    Vec3::new(next_x, *y, next_z).into(),
                    Vec3::new(next_angle.cos(), 0.0, next_angle.sin()).normalize().into(),
                    Vec2::new((i + 1) as f32 / segments as f32, *y / height).into(),
                ));
            }
        }

        // Create indices for the walls
        for i in 0..segments {
            let base = i * 4;
            let _next_base = ((i + 1) % segments) * 4;
            
            // First triangle
            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            
            // Second triangle
            indices.push(base + 2);
            indices.push(base + 1);
            indices.push(base + 3);
        }

        // Create floor and ceiling vertices
        let center_floor = vertices.len() as u32;
        vertices.push(Vertex::new(
            Vec3::new(0.0, 0.0, 0.0).into(),
            Vec3::new(0.0, -1.0, 0.0).into(),
            Vec2::new(0.5, 0.5).into(),
        ));

        let center_ceiling = vertices.len() as u32;
        vertices.push(Vertex::new(
            Vec3::new(0.0, height, 0.0).into(),
            Vec3::new(0.0, 1.0, 0.0).into(),
            Vec2::new(0.5, 0.5).into(),
        ));

        // Create indices for floor and ceiling
        for i in 0..segments {
            let base = i * 4;
            let _next_base = ((i + 1) % segments) * 4;
            
            // Floor triangles
            indices.push(center_floor);
            indices.push(base);
            indices.push(_next_base);
            
            // Ceiling triangles
            indices.push(center_ceiling);
            indices.push(base + 2);
            indices.push(_next_base + 2);
        }

        Self { vertices, indices }
    }

    pub fn create_door(width: f32, height: f32) -> Self {
        let vertices = vec![
            // Front face
            Vertex::new(
                Vec3::new(-width/2.0, 0.0, 0.0).into(),
                Vec3::new(0.0, 0.0, 1.0).into(),
                Vec2::new(0.0, 0.0).into(),
            ),
            Vertex::new(
                Vec3::new(width/2.0, 0.0, 0.0).into(),
                Vec3::new(0.0, 0.0, 1.0).into(),
                Vec2::new(1.0, 0.0).into(),
            ),
            Vertex::new(
                Vec3::new(width/2.0, height, 0.0).into(),
                Vec3::new(0.0, 0.0, 1.0).into(),
                Vec2::new(1.0, 1.0).into(),
            ),
            Vertex::new(
                Vec3::new(-width/2.0, height, 0.0).into(),
                Vec3::new(0.0, 0.0, 1.0).into(),
                Vec2::new(0.0, 1.0).into(),
            ),
        ];

        let indices = vec![
            0, 1, 2,
            2, 3, 0,
        ];

        Self { vertices, indices }
    }

    pub fn transform(&mut self, transform: &Mat4) {
        for vertex in &mut self.vertices {
            let transformed_vertex = transform_vertex(vertex, *transform);
            *vertex = transformed_vertex;
        }
    }
}

fn transform_vertex(vertex: &Vertex, transform: Mat4) -> Vertex {
    let transformed_pos = transform.transform_point3(vertex.position.into());
    let transformed_normal = transform.transform_vector3(vertex.normal.into()).normalize();
    
    Vertex {
        position: transformed_pos.into(),
        normal: transformed_normal.into(),
        tex_coord: vertex.tex_coord,
    }
}
