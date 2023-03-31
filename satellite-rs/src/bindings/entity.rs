use wasm_bindgen::prelude::*;

use super::{
    cartesian3::JSCartesian3, graphics::point_graphics::PointGraphics,
    position_property::PositionProperty,
};

#[wasm_bindgen(module = "cesium")]
extern "C" {
    #[derive(Debug)]
    pub type Entity;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Entity;

    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &Entity, val: String);

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &Entity) -> JSCartesian3;

    #[wasm_bindgen(method, setter)]
    pub fn set_position(this: &Entity, val: &PositionProperty);

    #[wasm_bindgen(method, setter)]
    pub fn set_point(this: &Entity, val: PointGraphics);
}

pub struct EntityAdapter<T: AsRef<PositionProperty>> {
    entity: Entity,
    position_property: T,
}

impl<T: AsRef<PositionProperty>> EntityAdapter<T> {
    pub fn new(entity: Entity, position_property: T) -> EntityAdapter<T> {
        EntityAdapter {
            entity,
            position_property,
        }
    }
}

impl<T: AsRef<PositionProperty>> AsRef<T> for EntityAdapter<T> {
    fn as_ref(&self) -> &T {
        &self.position_property
    }
}

impl<T: AsRef<PositionProperty>> AsRef<Entity> for EntityAdapter<T> {
    fn as_ref(&self) -> &Entity {
        &self.entity
    }
}
