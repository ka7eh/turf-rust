use wasm_bindgen::prelude::*;
use bbox::bbox;

use crate::helpers::Geometry;

#[wasm_bindgen]
pub fn wasm_bbox(geom: Geometry) -> JsValue {
    JsValue::from_serde(&bbox(&geom.geometry)).unwrap()
}
