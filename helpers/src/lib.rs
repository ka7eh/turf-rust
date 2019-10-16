use geojson::feature::Id;
use geojson::{Bbox, Feature, FeatureCollection, GeoJson, Geometry, Value};
use serde_json::{Map, Value as JsonValue};

pub struct FeatureOptions {
    bbox: Option<Bbox>,
    id: Option<Id>,
}

pub type FeatureProperties = Map<String, JsonValue>;

pub type Point = Vec<f64>;
pub type LineString = Vec<Point>;
pub type MultiLineString = Vec<LineString>;
pub type Polygon = Vec<LineString>;
pub type MultiPolygon = Vec<Polygon>;

pub fn feature(
    geometry: Geometry,
    properties: Option<FeatureProperties>,
    options: Option<FeatureOptions>,
) -> GeoJson {
    let (id, bbox) = match options {
        Some(options) => {
            let FeatureOptions { id, bbox } = options;
            (id, bbox)
        }
        None => (None, None),
    };
    GeoJson::Feature(Feature {
        bbox,
        geometry: Some(geometry),
        id,
        properties,
        foreign_members: None,
    })
}

pub fn point(
    coord: Point,
    properties: Option<FeatureProperties>,
    options: Option<FeatureOptions>,
) -> GeoJson {
    feature(Geometry::new(Value::Point(coord)), properties, options)
}

pub fn line_string(
    coords: LineString,
    properties: Option<FeatureProperties>,
    options: Option<FeatureOptions>,
) -> GeoJson {
    feature(Geometry::new(Value::LineString(coords)), properties, options)
}

pub fn multi_line_string(
    coords: MultiLineString,
    properties: Option<FeatureProperties>,
    options: Option<FeatureOptions>,
) -> GeoJson {
    feature(Geometry::new(Value::MultiLineString(coords)), properties, options)
}

pub fn polygon(
    coords: Polygon,
    properties: Option<FeatureProperties>,
    options: Option<FeatureOptions>,
) -> GeoJson {
    feature(Geometry::new(Value::Polygon(coords)), properties, options)
}

pub fn multi_polygon(
    coords: MultiPolygon,
    properties: Option<FeatureProperties>,
    options: Option<FeatureOptions>,
) -> GeoJson {
    feature(Geometry::new(Value::MultiPolygon(coords)), properties, options)
}

pub fn feature_collection(features: Vec<GeoJson>) -> GeoJson {
    GeoJson::FeatureCollection(FeatureCollection {
        bbox: None,
        features: features
            .into_iter()
            .map(|geojson| match geojson {
                GeoJson::Feature(feature) => feature,
                _ => panic!("Invalid feature")
            })
            .collect(),
        foreign_members: None,
    })
}
