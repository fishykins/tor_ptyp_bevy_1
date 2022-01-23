use super::normalize_angle;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Clone, Inspectable, Reflect, PartialEq)]
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

/// A rigidbody component
#[derive(Clone, Inspectable, Reflect, Component)]
pub struct Rigidbody<T>
where
    T: 'static + Sync + Send + Default + Inspectable + Reflect,
{
    #[inspectable(label = "type")]
    pub r#type: RigidbodyType,
    pub mass: f32,
    pub linear_velocity: Vec2,
    pub angular_velocity: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    pub linear_decay: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    pub linear_drag: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    pub angular_drag: f32,
    /// The amount of angular drag resulting from linear velocity (this is a fishics thing).
    #[inspectable(min = 0.0, max = 1.0)]
    pub linear_bleed: f32,
    /// The amount of linear drag resulting from angular velocity (this is a fishics thing).
    #[inspectable(min = 0.0, max = 1.0)]
    pub angular_bleed: f32,
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
    torques: Vec<f32>,

    #[inspectable(ignore)]
    #[reflect(ignore)]
    phantom: std::marker::PhantomData<T>,
}

impl<T> Default for Rigidbody<T>
where
    T: 'static + Sync + Send + Default + Inspectable + Reflect,
{
    fn default() -> Self {
        Self {
            r#type: RigidbodyType::Static,
            mass: 100.0,
            linear_velocity: Vec2::default(),
            angular_velocity: 0.0,
            linear_decay: 0.02,
            linear_drag: 0.07,
            linear_bleed: 0.0,
            angular_drag: 0.0,
            angular_bleed: 0.15,
            position: Vec2::default(),
            rotation: 0.0,
            auto_tracking: false,
            last_updated: 0,
            last_position: Vec2::default(),
            last_rotation: 0.0,
            forces: Vec::new(),
            torques: Vec::new(),
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
        let cached_lv = self.linear_velocity;
        for f in self.forces.iter_mut() {
            self.linear_velocity += *f / self.mass;
        }
        if self.linear_velocity.is_nan() {
            self.linear_velocity = cached_lv;
            bevy::log::warn!("NaN linear velocity: {:?}", self.forces);
        }
        self.forces.clear();

        // TODO: Add angular forces

        // Apply a drag force. This is not realistic, but it feels nice.
        let vel = self.linear_velocity.length();

        let drag: f32 = if vel >= 1.0 {
            vel / vel.powf(1.0 + self.linear_drag)
        } else {
            1.0
        };
        let decay = 1.0 - self.linear_decay;
        let bleed = 1.0 / (self.angular_velocity.abs() * self.angular_bleed + 1.0);
        self.linear_velocity = self.linear_velocity * drag.min(decay).min(bleed).clamp(0.0, 1.0);
        //bevy::log::info!("v: {}, drag: {:?}, av: {}, bleed: {}", vel, drag, self.angular_velocity, bleed);


        // Update position
        self.last_position = self.position;
        self.position += self.linear_velocity * delta_seconds;
        

        // Update rotation
        self.angular_velocity = (self.rotation - self.last_rotation) / delta_seconds;
        self.last_rotation = self.rotation;
        self.rotation = normalize_angle(self.rotation);

        // Update tick data
        if self.auto_tracking {
            self.last_updated += 1;
        }
    }

    /// Applies a force on the rigidbody in the direction of the force vector.
    /// The force is applied at the center of mass, and accounts for time elapsed.
    pub fn add_force(&mut self, f: Vec2) {
        if !f.is_nan(){
            self.forces.push(f);
        }
    }

    /// Applies a relative force on the rigidbody, accounting for local rotation.
    /// The force is applied at the center of mass, and accounts for time elapsed.
    pub fn add_local_force(&mut self, f: Vec2) {
        let dir = Quat::from_rotation_z(self.rotation);
        let v: Vec3 = dir * Vec3::new(f.x, f.y, 0.0);
        self.add_force(Vec2::new(v.x, v.y));
    }

    /// Adds a force in the forward direction.
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
}
