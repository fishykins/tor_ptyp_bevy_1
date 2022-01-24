
/// A physical projectile, typically fired using gunpowder.
pub trait Projectile {
    fn diameter(&self) -> f32;
    fn length(&self) -> f32;
}