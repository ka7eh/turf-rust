use crate::utils;
use geojson::GeoJson;
use helpers::{line_string, multi_line_string, multi_polygon, point, polygon};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Geometry {
    #[wasm_bindgen(skip)]
    pub geometry: GeoJson,
}

#[wasm_bindgen]
impl Geometry {
    #[wasm_bindgen(constructor)]
    pub fn new(geom_type: &str, js_coords: JsValue) -> Self {
        utils::set_panic_hook();
        Geometry {
            geometry: match geom_type {
                "point" => {
                    let coords = js_coords.into_serde().unwrap();
                    point(coords)
                }
                "line" | "line_string" => {
                    let coords = js_coords.into_serde().unwrap();
                    line_string(coords)
                }
                "multi_line" | "multi_line_string" => {
                    let coords = js_coords.into_serde().unwrap();
                    multi_line_string(coords)
                }
                "polygon" => {
                    console::log_1(&format!("{:?}", js_coords).into());
                    let coords = js_coords.into_serde().unwrap();
                    polygon(coords)
                }
                "multi_polygon" => {
                    let coords = js_coords.into_serde().unwrap();
                    multi_polygon(coords)
                }
                _ => panic!("Unsupported type!"),
            },
        }
    }
}
