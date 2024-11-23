use glam::Vec3;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        BoundingBox { min, max }
    }

    pub fn from_points(points: &[Vec3]) -> Self {
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        for point in points {
            min = min.min(*point);
            max = max.max(*point);
        }

        Self { min, max }
    }

    pub fn contains_point(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        point.clamp(self.min, self.max)
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn intersects_line_segment(&self, start: Vec3, end: Vec3) -> bool {
        let dir = end - start;
        let dir_inv = Vec3::new(
            1.0 / dir.x,
            1.0 / dir.y,
            1.0 / dir.z,
        );

        let t1 = (self.min - start) * dir_inv;
        let t2 = (self.max - start) * dir_inv;

        let tmin = t1.min(t2);
        let tmax = t1.max(t2);

        let t_min = tmin.x.max(tmin.y).max(tmin.z);
        let t_max = tmax.x.min(tmax.y).min(tmax.z);

        t_max >= t_min && t_min <= 1.0 && t_max >= 0.0
    }

    pub fn normal_at_point(&self, point: Vec3) -> Vec3 {
        let center = self.center();
        let half_size = (self.max - self.min) * 0.5;
        let local_point = point - center;

        let mut normal = Vec3::ZERO;
        let mut min_dist = f32::MAX;

        // Check each face
        let faces = [
            (Vec3::X, half_size.x),
            (Vec3::NEG_X, half_size.x),
            (Vec3::Y, half_size.y),
            (Vec3::NEG_Y, half_size.y),
            (Vec3::Z, half_size.z),
            (Vec3::NEG_Z, half_size.z),
        ];

        for (axis, size) in faces {
            let dist = (local_point.dot(axis) - size).abs();
            if dist < min_dist {
                min_dist = dist;
                normal = axis;
            }
        }

        normal
    }
}
