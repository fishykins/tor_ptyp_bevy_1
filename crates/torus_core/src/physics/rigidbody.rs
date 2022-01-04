use super::normalize_angle;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Clone, Inspectable, Reflect)]
pub enum RigidbodyType {
    Static,
    Dynamic,
    Kinematic,
}

impl Default for RigidbodyType {
    fn default() -> Self {
        Self::Static
    }
}

#[derive(Clone, Inspectable, Reflect)]
pub struct Rigidbody<T>
where
    T: 'static + Sync + Send + Default + Inspectable + Reflect,
{
    #[inspectable(label="type")]
    pub r#type: RigidbodyType,
    pub mass: f32,
    pub linear_velocity: Vec2,
    pub angular_velocity: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    pub linear_drag: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    pub angular_drag: f32,
    pub position: Vec2,
    pub rotation: f32,
    pub auto_tracking: bool,

    #[inspectable(read_only)]
    last_updated: u64,
    #[inspectable(ignore)]
    last_position: Vec2,
    #[inspectable(ignore)]
    last_rotation: f32,

    #[inspectable(ignore)]
    forces: Vec<Vec2>,

    #[inspectable(ignore)]
    #[reflect(ignore)]
    phantom: std::marker::PhantomData<T>,
}

impl<T> Default for Rigidbody<T> where T: 'static + Sync + Send + Default + Inspectable + Reflect {
    fn default() -> Self {
        Self {
            r#type: RigidbodyType::Static,
            mass: 100.0,
            linear_velocity: Vec2::default(),
            angular_velocity: 0.0,
            linear_drag: 0.3,
            angular_drag: 0.01,
            position: Vec2::default(),
            rotation: 0.0,
            auto_tracking: false,
            last_updated: 0,
            last_position: Vec2::default(),
            last_rotation: 0.0,
            forces: Vec::new(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Rigidbody<T>
where
    T: 'static + Sync + Send + Default + Inspectable + Reflect,
{
    /// Applies all currently pending forces.
    pub fn update(&mut self, delta_seconds: f32) {
        // Add all new forces
        for f in self.forces.iter_mut() {
            self.linear_velocity += *f / self.mass;
        }
        self.forces.clear();

        // TODO: Add angular forces

        // Apply drag
        let drag = 1.0 - (self.linear_drag * delta_seconds * self.linear_velocity.length() * self.linear_velocity.length() * 0.01);
        let decay = 1.0 - (self.linear_drag * delta_seconds * 100.0);
        self.linear_velocity = self.linear_velocity * drag.min(decay);
        bevy::log::info!("drag: {:?}, decay: {}", drag, decay);

        // Update position
        self.last_position = self.position;
        self.last_rotation = self.rotation;
        self.position += self.linear_velocity * delta_seconds;

        if self.auto_tracking {
            self.last_updated += 1;
        }
    }

    pub fn update_system(time: Res<Time>, mut rbs: Query<&mut Rigidbody<T>>) {
        for mut rb in rbs.iter_mut() {
            rb.update(time.delta_seconds());
        }
    }

    /// Applies a force on the rigidbody in the direction of the force vector.
    /// The force is applied at the center of mass, and accounts for time elapsed.
    pub fn add_force(&mut self, f: Vec2) {
        self.forces.push(f);
    }

    /// Applies a relative force on the rigidbody, accounting for local rotation.
    /// The force is applied at the center of mass, and accounts for time elapsed.
    pub fn add_local_force(&mut self, f: Vec2) {
        let dir = Quat::from_rotation_z(self.rotation);
        let v: Vec3 = dir * Vec3::new(f.x, f.y, 0.0);
        self.forces.push(Vec2::new(v.x, v.y));
    }

    pub fn add_forward_force(&mut self, f: f32) {
        self.add_local_force(Vec2::new(0.0, f));
    }

    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            position: translation,
            ..Default::default()
        }
    }

    pub fn set_last_update(&mut self, frame: u64) {
        self.last_updated = frame;
    }

    pub fn last_updated(&self) -> u64 {
        self.last_updated
    }

    pub fn validate(&mut self) {
        self.rotation = normalize_angle(self.rotation);
    }
}
