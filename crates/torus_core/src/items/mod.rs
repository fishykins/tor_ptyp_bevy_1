pub mod attire;
pub mod components;
pub mod containers;

mod manakin;
mod holdable;
mod item;
mod weapons;
mod material;

pub use manakin::Manakin;
pub use holdable::Holdable;
pub use item::{Item, Physical, Wrappable};
pub use weapons::Weapon;
pub use material::Material;

pub type DefaultFloat = f32;
pub const PI: DefaultFloat = std::f32::consts::PI;

#[cfg(test)]
mod tests {
    use crate::items::{attire::*, attire::AttireSlots::*, components::*, containers::*, Wrappable};

    #[test]
    fn atire_test() {
        let mut hoodie = Layer::new(
            "hoodie".to_string(),
            vec![Midlayer],
            vec![Pocket::new(10.0f32)]
        );
        let pocket = hoodie.pocket_mut(0).unwrap();
        let duck = Animal::new("QUACK".to_string(), 1.0, 2.0);
        let bunny = Animal::new("HOP".to_string(), 2.5, 3.0);
        let bear = Animal::new("GROWL".to_string(), 100.0, 6.0);
        let duck_index = pocket
            .try_add(duck.wrap())
            .expect("Could not add duck to pocket :(");
        println!(
            "After adding the duck, the pocket is {}/{} full",
            pocket.fill(),
            pocket.max_capacity()
        );
        let bunny_index = pocket
            .try_add(bunny.wrap())
            .expect("Could not add bunny to pocket :(");
        println!(
            "After adding the bunny, the pocket is {}/{} full",
            pocket.fill(),
            pocket.max_capacity()
        );
        let bear = pocket
            .try_add(bear.wrap())
            .expect_err("Bear should not fit in pocket but it went in anyway");
        pocket
            .try_remove(bunny_index)
            .expect("Could not remove bunny from the pocket?! Oh no...");
        println!(
            "After removing the bunny, the pocket is {}/{} full",
            pocket.fill(),
            pocket.max_capacity()
        );
        let bear_index = pocket
            .try_add(bear)
            .expect("Could not add bear to pocket :(");

        let duck_ref = pocket
            .item(duck_index)
            .expect("No duck found but it should be there!");
        let bear_ref = pocket
            .item(bear_index)
            .expect("no bear found but it should be there!");
        let duck = Animal::try_unwrap(duck_ref).expect("Could not make duck from item :(");
        let bear = Animal::try_unwrap(bear_ref).expect("Could not make bear from item :(");
        duck.make_sound();
        bear.make_sound();
    }
}
