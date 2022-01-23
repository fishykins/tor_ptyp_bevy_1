use std::fmt::{Formatter, Display};

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use prima::geom::{Circle, Rect};

#[derive(Clone, Reflect, Debug, Inspectable)]
pub enum ColliderShape {
    None,
    Circle(f32),
    Rect(Vec2),
}

impl Display for ColliderShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColliderShape::None => write!(f, "None"),
            ColliderShape::Circle(r) => write!(f, "{}", Circle::new(Vec2::ZERO, *r)),
            ColliderShape::Rect(size) => write!(f, "{}", Rect::new_sized(*size)),
        }
    }
}

// impl Inspectable for ColliderShape {
//     type Attributes = ColliderShape;

//     fn ui(
//         &mut self,
//         ui: &mut bevy_inspector_egui::egui::Ui,
//         _: Self::Attributes,
//         _: &mut bevy_inspector_egui::Context,
//     ) -> bool {
//         let display: ColliderShape;
//         match self {
//             ColliderShape::None => {
//                 display = ColliderShape::None;
//             }
//             ColliderShape::Circle(circle) => {
//                 display = ColliderShape::Circle(*circle);
//             }
//             ColliderShape::Rect(rect) => {
//                 display = ColliderShape::Rect(*rect);
//             }
//         }
//         let hash = format!("{:?}", self);

//         bevy_inspector_egui::egui::ComboBox::from_id_source(hash)
//             .selected_text(format!("{}", display))
//             .show_ui(ui, |ui| {
//                 ui.heading(format!("{}", display));
//             });
//         true
//     }

//     fn setup(_: &mut App) {}
// }

impl Default for ColliderShape {
    fn default() -> Self {
        ColliderShape::None
    }
}
