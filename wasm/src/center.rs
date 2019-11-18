use wasm_bindgen::prelude::*;
use center::center;

use crate::helpers::Geometry;

#[wasm_bindgen]
pub fn wasm_center(geom: Geometry) -> JsValue {
    JsValue::from_serde(&center(&geom.geometry)).unwrap()
}
