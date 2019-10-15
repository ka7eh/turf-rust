use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};

pub type Point = Vec<f64>;
pub type LineString = Vec<Point>;
pub type MultiLineString = Vec<LineString>;
pub type Polygon = Vec<LineString>;
pub type MultiPolygon = Vec<Polygon>;

pub fn point(coord: Point) -> GeoJson {
    GeoJson::Geometry(
        Geometry::new(
            Value::Point(coord)
        )
    )
}

pub fn line_string(coords: LineString) -> GeoJson {
    GeoJson::Geometry(
        Geometry::new(
            Value::LineString(coords)
        )
    )
}

pub fn multi_line_string(coords: MultiLineString) -> GeoJson {
    GeoJson::Geometry(
        Geometry::new(
            Value::MultiLineString(coords)
        )
    )
}

pub fn polygon(coords: Polygon) -> GeoJson {
    GeoJson::Geometry(
        Geometry::new(
            Value::Polygon(coords)
        )
    )
}

pub fn multi_polygon(coords: MultiPolygon) -> GeoJson {
    GeoJson::Geometry(
        Geometry::new(
            Value::MultiPolygon(coords)
        )
    )
}

pub fn feature_collection(features: Vec<GeoJson>) -> GeoJson {
    GeoJson::FeatureCollection(
        FeatureCollection {
            bbox: None,
            features: features.into_iter().map(
                |geometry| Feature {
                        bbox: None,
                        geometry: match geometry {
                            GeoJson::Geometry(g) => Some(g),
                            _ => None
                        },
                        id: None,
                        properties: None,
                        foreign_members: None
                    }
            ).collect(),
            foreign_members: None
        }
    )
}
