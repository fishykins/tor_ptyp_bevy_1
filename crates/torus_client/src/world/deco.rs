use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;
use torus_core::physics::Collider;


pub fn spawn_deco(commands: &mut Commands) {

    let mut rng = thread_rng();

    let shape = shapes::RegularPolygon {
        sides: rng.gen_range(3..8),
        feature: shapes::RegularPolygonFeature::Radius(rng.gen_range(4.0..16.0)),
        ..shapes::RegularPolygon::default()
    };

    let mut entity = commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::rgb(rng.gen(), rng.gen(), rng.gen())),
            outline_mode: StrokeMode::new(Color::BLACK, 2.0),
        },
        Transform::from_translation(Vec3::new(rng.gen_range(0.0..1200.0), rng.gen_range(0.0..1200.0), -10.0))
    ));

    entity.insert(Collider::default());
}