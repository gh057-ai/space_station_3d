use std::sync::Arc;
use glam::{Vec3, Quat, Mat4, Vec4};
use crate::geometry::Mesh;
use crate::material::Material;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModuleType {
    Corridor,
    Hub,
    Airlock,
    LivingQuarters,
    CommandCenter,
    Laboratory,
    Storage,
    PowerPlant,
}

#[derive(Debug)]
pub enum InteractionType {
    None,
    Door,
    Console,
    Light,
    Window,
    Button,
    Terminal,
    PowerControl,
    LifeSupport,
    Experiment,
    Storage,
    MainComputer,
    Communications,
    StationControl,
    ResearchStation,
    LabEquipment,
    AirlockControl,
    PressureControl,
    EnvironmentControl,
}

#[derive(Debug)]
pub enum ElementState {
    Inactive,
    Active,
    Transitioning(f32), // Progress from 0.0 to 1.0
    Locked,
    Warning,
    Emergency,
    Malfunction,
}

#[derive(Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.position,
        )
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }

    pub fn rotate(&mut self, axis: Vec3, angle: f32) {
        self.rotation *= Quat::from_axis_angle(axis.normalize(), angle);
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.scale *= scale;
    }

    pub fn from_position(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

#[derive(Debug)]
pub struct StationModule {
    pub module_type: ModuleType,
    pub transform: Transform,
    pub mesh: Mesh,
    pub material: Material,
    pub connected_modules: Vec<usize>,
    pub structural_integrity: f32,
    pub power_consumption: f32,
    pub power_generation: f32,
    pub atmosphere_sealed: bool,
    pub interactive_elements: Vec<InteractiveElement>,
}

#[derive(Debug)]
pub struct InteractiveElement {
    pub element_type: InteractionType,
    pub state: ElementState,
    pub position: Vec3,
    pub power_draw: f32,
}

impl StationModule {
    pub fn new(module_type: ModuleType, position: Vec3) -> Self {
        let (mesh, material) = Self::generate_module_geometry(&module_type);
        let mut module = Self {
            module_type,
            transform: Transform::from_position(position),
            mesh,
            material,
            connected_modules: Vec::new(),
            structural_integrity: 1.0,
            power_consumption: 0.0,
            power_generation: 0.0,
            atmosphere_sealed: true,
            interactive_elements: Vec::new(),
        };

        // Configure module-specific properties
        match module_type {
            ModuleType::PowerPlant => {
                module.power_generation = 100.0;
                module.power_consumption = 10.0;
                module.add_interactive_elements(&[
                    (InteractionType::PowerControl, Vec3::new(2.0, 0.0, 0.0)),
                    (InteractionType::EmergencyShutoff, Vec3::new(-2.0, 0.0, 0.0)),
                ]);
            }
            ModuleType::LivingQuarters => {
                module.power_consumption = 15.0;
                module.add_interactive_elements(&[
                    (InteractionType::LightControl, Vec3::new(1.0, 2.0, 0.0)),
                    (InteractionType::EnvironmentControl, Vec3::new(-1.0, 2.0, 0.0)),
                ]);
            }
            ModuleType::CommandCenter => {
                module.power_consumption = 25.0;
                module.add_interactive_elements(&[
                    (InteractionType::MainComputer, Vec3::ZERO),
                    (InteractionType::Communications, Vec3::new(2.0, 0.0, 2.0)),
                    (InteractionType::StationControl, Vec3::new(-2.0, 0.0, 2.0)),
                ]);
            }
            ModuleType::Laboratory => {
                module.power_consumption = 20.0;
                module.add_interactive_elements(&[
                    (InteractionType::ResearchStation, Vec3::new(2.0, 0.0, 0.0)),
                    (InteractionType::LabEquipment, Vec3::new(-2.0, 0.0, 0.0)),
                ]);
            }
            ModuleType::Airlock => {
                module.power_consumption = 5.0;
                module.add_interactive_elements(&[
                    (InteractionType::AirlockControl, Vec3::ZERO),
                    (InteractionType::PressureControl, Vec3::new(0.0, 2.0, 0.0)),
                ]);
            }
            ModuleType::Storage => {
                module.power_consumption = 5.0;
                module.add_interactive_elements(&[
                    (InteractionType::StorageAccess, Vec3::new(0.0, 0.0, 2.0)),
                ]);
            }
            ModuleType::Corridor => {
                module.power_consumption = 2.0;
                module.add_interactive_elements(&[
                    (InteractionType::LightControl, Vec3::new(0.0, 2.0, 0.0)),
                ]);
            }
            ModuleType::Hub => {
                module.power_consumption = 8.0;
                module.add_interactive_elements(&[
                    (InteractionType::LightControl, Vec3::new(0.0, 2.0, 0.0)),
                    (InteractionType::EnvironmentControl, Vec3::new(2.0, 0.0, 0.0)),
                ]);
            }
        }

        module
    }

    fn add_interactive_elements(&mut self, elements: &[(InteractionType, Vec3)]) {
        for (element_type, position) in elements {
            self.interactive_elements.push(InteractiveElement {
                element_type: *element_type,
                state: ElementState::Inactive,
                position: *position,
                power_draw: match element_type {
                    InteractionType::MainComputer => 5.0,
                    InteractionType::Communications => 3.0,
                    InteractionType::StationControl => 4.0,
                    InteractionType::PowerControl => 2.0,
                    InteractionType::EnvironmentControl => 2.0,
                    InteractionType::LightControl => 1.0,
                    _ => 0.5,
                },
            });
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update interactive elements
        for element in &mut self.interactive_elements {
            match element.state {
                ElementState::Active => {
                    self.power_consumption += element.power_draw * delta_time;
                }
                ElementState::Inactive => {}
                ElementState::Malfunction => {
                    self.structural_integrity -= 0.01 * delta_time;
                }
            }
        }

        // Clamp structural integrity
        self.structural_integrity = self.structural_integrity.clamp(0.0, 1.0);
    }

    fn generate_module_geometry(module_type: &ModuleType) -> (Mesh, Material) {
        match module_type {
            ModuleType::Corridor => {
                let mesh = Mesh::create_cylinder(2.0, 8.0, 32);
                let material = Material::new(
                    Vec4::new(0.7, 0.7, 0.7, 1.0),
                    0.8,
                    0.2,
                    1.0,
                );
                (mesh, material)
            }
            ModuleType::Hub => {
                let mesh = Mesh::create_octagonal_room(8.0, 4.0, 8.0);
                let material = Material::new(
                    Vec4::new(0.75, 0.75, 0.8, 1.0),
                    0.8,
                    0.3,
                    1.0,
                );
                (mesh, material)
            }
            ModuleType::Airlock => {
                let mesh = Mesh::create_octagonal_room(4.0, 3.0, 4.0);
                let material = Material::new(
                    Vec4::new(0.6, 0.6, 0.65, 1.0),
                    0.9,
                    0.2,
                    1.0,
                );
                (mesh, material)
            }
            ModuleType::LivingQuarters => {
                let mesh = Mesh::create_octagonal_room(10.0, 4.0, 10.0);
                let material = Material::new(
                    Vec4::new(0.8, 0.75, 0.7, 1.0),
                    0.6,
                    0.4,
                    1.0,
                );
                (mesh, material)
            }
            ModuleType::CommandCenter => {
                let mesh = Mesh::create_octagonal_room(12.0, 5.0, 12.0);
                let material = Material::new(
                    Vec4::new(0.6, 0.65, 0.7, 1.0),
                    0.85,
                    0.2,
                    1.0,
                );
                (mesh, material)
            }
            ModuleType::Laboratory => {
                let mesh = Mesh::create_octagonal_room(9.0, 4.0, 9.0);
                let material = Material::new(
                    Vec4::new(0.85, 0.85, 0.9, 1.0),
                    0.7,
                    0.3,
                    1.0,
                );
                (mesh, material)
            }
            ModuleType::Storage => {
                let mesh = Mesh::create_octagonal_room(10.0, 6.0, 15.0);
                let material = Material::new(
                    Vec4::new(0.6, 0.6, 0.6, 1.0),
                    0.7,
                    0.5,
                    1.0,
                );
                (mesh, material)
            }
            ModuleType::PowerPlant => {
                let mesh = Mesh::create_octagonal_room(12.0, 8.0, 12.0);
                let material = Material::new(
                    Vec4::new(0.5, 0.5, 0.55, 1.0),
                    0.9,
                    0.2,
                    1.0,
                );
                (mesh, material)
            }
        }
    }
}

#[derive(Debug)]
pub struct SpaceStation {
    modules: Vec<StationModule>,
    power_grid: PowerGrid,
    life_support: LifeSupport,
    structural_integrity: f32,
}

impl SpaceStation {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            power_grid: PowerGrid::new(),
            life_support: LifeSupport::new(),
            structural_integrity: 1.0,
        }
    }

    pub fn create_default_layout() -> Self {
        let mut station = Self::new();

        // Create command center as the central hub
        let command_center_idx = station.add_module(ModuleType::CommandCenter, Vec3::ZERO);

        // Add surrounding corridors in cardinal directions
        let north_corridor = station.add_module(
            ModuleType::Corridor,
            Vec3::new(0.0, 0.0, -8.0)
        );
        let east_corridor = station.add_module(
            ModuleType::Corridor,
            Vec3::new(8.0, 0.0, 0.0)
        );
        let south_corridor = station.add_module(
            ModuleType::Corridor,
            Vec3::new(0.0, 0.0, 8.0)
        );
        let west_corridor = station.add_module(
            ModuleType::Corridor,
            Vec3::new(-8.0, 0.0, 0.0)
        );

        // Connect corridors to command center
        station.connect_modules(command_center_idx, north_corridor);
        station.connect_modules(command_center_idx, east_corridor);
        station.connect_modules(command_center_idx, south_corridor);
        station.connect_modules(command_center_idx, west_corridor);

        // Add modules at the ends of corridors
        let lab_idx = station.add_module(
            ModuleType::Laboratory,
            Vec3::new(0.0, 0.0, -16.0)
        );
        station.connect_modules(north_corridor, lab_idx);

        let living_idx = station.add_module(
            ModuleType::LivingQuarters,
            Vec3::new(16.0, 0.0, 0.0)
        );
        station.connect_modules(east_corridor, living_idx);

        let storage_idx = station.add_module(
            ModuleType::Storage,
            Vec3::new(0.0, 0.0, 16.0)
        );
        station.connect_modules(south_corridor, storage_idx);

        let power_idx = station.add_module(
            ModuleType::PowerPlant,
            Vec3::new(-16.0, 0.0, 0.0)
        );
        station.connect_modules(west_corridor, power_idx);

        // Add an airlock off the laboratory
        let airlock_idx = station.add_module(
            ModuleType::Airlock,
            Vec3::new(0.0, 0.0, -24.0)
        );
        station.connect_modules(lab_idx, airlock_idx);

        station
    }

    pub fn add_module(&mut self, module_type: ModuleType, position: Vec3) -> usize {
        let module = StationModule::new(module_type, position);
        self.modules.push(module);
        self.modules.len() - 1
    }

    pub fn connect_modules(&mut self, module1_idx: usize, module2_idx: usize) -> bool {
        if module1_idx >= self.modules.len() || module2_idx >= self.modules.len() {
            return false;
        }

        // Check if modules are close enough to connect
        let pos1 = self.modules[module1_idx].transform.position;
        let pos2 = self.modules[module2_idx].transform.position;
        let distance = (pos2 - pos1).length();

        // Maximum connection distance (should be sum of module radii)
        let max_distance = 10.0;

        if distance > max_distance {
            return false;
        }

        // Add connection references
        self.modules[module1_idx].connected_modules.push(module2_idx);
        self.modules[module2_idx].connected_modules.push(module1_idx);

        // Update structural integrity
        self.update_structural_integrity();

        true
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update power distribution
        self.power_grid.update(delta_time);

        // Update life support systems
        self.life_support.update(delta_time);

        // Update all modules
        for module in &mut self.modules {
            module.update(delta_time);
        }

        // Update structural integrity
        self.update_structural_integrity();
    }

    fn update_structural_integrity(&mut self) {
        // Base integrity starts at 1.0
        let mut total_integrity = 1.0;

        // Check each module's individual integrity
        for module in &self.modules {
            total_integrity = total_integrity.min(module.structural_integrity);
        }

        // Check connection stresses
        for (i, module) in self.modules.iter().enumerate() {
            for &connected_idx in &module.connected_modules {
                let stress = self.calculate_connection_stress(i, connected_idx);
                total_integrity = total_integrity.min(1.0 - stress);
            }
        }

        self.structural_integrity = total_integrity;
    }

    fn calculate_connection_stress(&self, module1_idx: usize, module2_idx: usize) -> f32 {
        let pos1 = self.modules[module1_idx].transform.position;
        let pos2 = self.modules[module2_idx].transform.position;
        
        // Calculate stress based on distance and angle
        let distance = (pos2 - pos1).length();
        let optimal_distance = 8.0; // Optimal connection distance
        
        // Distance stress increases quadratically with deviation from optimal
        let distance_stress = ((distance - optimal_distance) / optimal_distance).powi(2) * 0.5;
        
        // Add other stress factors (could include module mass, vibration, etc.)
        distance_stress
    }
}

#[derive(Debug)]
struct PowerGrid {
    total_output: f32,
    total_consumption: f32,
    grid_stability: f32,
}

impl PowerGrid {
    fn new() -> Self {
        Self {
            total_output: 0.0,
            total_consumption: 0.0,
            grid_stability: 1.0,
        }
    }

    fn update(&mut self, delta_time: f32) {
        // Update power generation and consumption
        // This would be expanded based on active modules and systems
    }
}

#[derive(Debug)]
struct LifeSupport {
    oxygen_level: f32,
    temperature: f32,
    pressure: f32,
}

impl LifeSupport {
    fn new() -> Self {
        Self {
            oxygen_level: 1.0,
            temperature: 293.15, // 20°C in Kelvin
            pressure: 1.0, // 1 atm
        }
    }

    fn update(&mut self, delta_time: f32) {
        // Update life support parameters
        // This would be expanded based on module states and crew activities
    }
}
