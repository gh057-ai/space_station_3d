use crate::lighting::{Light, Material, LightManager};
use crate::model::Model;
use glam::{Vec3, Mat4, Quat};
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;

#[derive(Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.position,
        )
    }
}

pub struct SceneObject {
    pub name: String,
    pub transform: Transform,
    pub model: Option<Arc<Model>>,
    pub material: Material,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
}

impl SceneObject {
    pub fn new(name: String, transform: Transform, model: Option<Arc<Model>>, material: Material) -> Self {
        Self {
            name,
            transform,
            model,
            material,
            children: Vec::new(),
            parent: None,
        }
    }

    pub fn world_matrix(&self, scene: &Scene) -> Mat4 {
        let local_matrix = self.transform.matrix();
        if let Some(parent_id) = self.parent {
            if let Some(parent) = scene.objects.get(parent_id) {
                return parent.world_matrix(scene) * local_matrix;
            }
        }
        local_matrix
    }
}

pub struct Scene {
    objects: Vec<SceneObject>,
    object_map: HashMap<String, usize>,
    light_manager: LightManager,
    root_objects: Vec<usize>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            object_map: HashMap::new(),
            light_manager: LightManager::new(),
            root_objects: Vec::new(),
        }
    }

    pub fn add_object(
        &mut self,
        name: String,
        transform: Transform,
        model: Option<Arc<Model>>,
        material: Material,
        parent: Option<&str>,
    ) -> Result<usize> {
        let object_id = self.objects.len();
        let mut object = SceneObject::new(name.clone(), transform, model, material);

        if let Some(parent_name) = parent {
            if let Some(&parent_id) = self.object_map.get(parent_name) {
                object.parent = Some(parent_id);
                self.objects[parent_id].children.push(object_id);
            } else {
                anyhow::bail!("Parent object '{}' not found", parent_name);
            }
        } else {
            self.root_objects.push(object_id);
        }

        self.objects.push(object);
        self.object_map.insert(name, object_id);
        Ok(object_id)
    }

    pub fn remove_object(&mut self, name: &str) -> Result<()> {
        if let Some(&object_id) = self.object_map.get(name) {
            // Remove from parent's children list
            if let Some(parent_id) = self.objects[object_id].parent {
                if let Some(pos) = self.objects[parent_id].children.iter().position(|&id| id == object_id) {
                    self.objects[parent_id].children.remove(pos);
                }
            }

            // Remove from root objects if it's a root
            if let Some(pos) = self.root_objects.iter().position(|&id| id == object_id) {
                self.root_objects.remove(pos);
            }

            // Recursively remove all children
            let children = self.objects[object_id].children.clone();
            for child_id in children {
                if let Some(child_name) = self.object_map.iter()
                    .find(|&(_, &id)| id == child_id)
                    .map(|(name, _)| name.clone())
                {
                    self.remove_object(&child_name)?;
                }
            }

            // Remove the object itself
            self.object_map.remove(name);
            Ok(())
        } else {
            anyhow::bail!("Object '{}' not found", name)
        }
    }

    pub fn get_object(&self, name: &str) -> Option<&SceneObject> {
        self.object_map.get(name).map(|&id| &self.objects[id])
    }

    pub fn get_object_mut(&mut self, name: &str) -> Option<&mut SceneObject> {
        self.object_map.get(name).map(|&id| &mut self.objects[id])
    }

    pub fn add_light(&mut self, light: Light) -> bool {
        self.light_manager.add_light(light)
    }

    pub fn update_transforms(&mut self) {
        let root_objects = self.root_objects.clone();
        for &root_id in &root_objects {
            self.update_object_transform(root_id, Mat4::IDENTITY);
        }
    }

    fn update_object_transform(&mut self, object_id: usize, parent_transform: Mat4) {
        let local_transform = self.objects[object_id].transform.matrix();
        let world_transform = parent_transform * local_transform;

        // Update children
        let children = self.objects[object_id].children.clone();
        for child_id in children {
            self.update_object_transform(child_id, world_transform);
        }
    }

    pub fn get_light_manager(&self) -> &LightManager {
        &self.light_manager
    }

    pub fn get_light_manager_mut(&mut self) -> &mut LightManager {
        &mut self.light_manager
    }

    pub fn traverse<F>(&self, f: F)
    where
        F: FnMut(&SceneObject),
    {
        self.traverse_internal(&self.root_objects, f);
    }

    fn traverse_internal<F>(&self, objects: &[usize], mut f: F)
    where
        F: FnMut(&SceneObject),
    {
        for &object_id in objects {
            let object = &self.objects[object_id];
            f(object);
            self.traverse_internal(&object.children, &mut f);
        }
    }
}
