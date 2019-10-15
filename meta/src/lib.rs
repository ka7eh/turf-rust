use geojson::{Geometry, GeoJson, Position, Value};

fn match_geometry<F>(geometry: &Geometry, f: &F)
    where F: Fn(&Position) {
    match geometry.value {
        Value::Point(ref point) => {
            f(point);
        }
        Value::MultiPoint(ref points) => {
            for point in points {
                f(point);
            }
        }
        Value::LineString(ref line_string) => {
            for point in line_string {
                f(point);
            }
        }
        Value::MultiLineString(ref multi_line_string) => {
            for line in multi_line_string {
                for point in line {
                    f(point);
                }
            }
        }
        Value::Polygon(ref polygon) => {
            for line in polygon {
                for point in line {
                    f(point);
                }
            }
        }
        Value::MultiPolygon(ref multi_polygon) => {
            for polygon in multi_polygon {
                for line in polygon {
                    for point in line {
                        f(point);
                    }
                }
            }
        }
        Value::GeometryCollection(ref gc) => {
            for geometry in gc {
                match_geometry(geometry, f)
            }
        }
    }
}

pub fn coord_each<F>(geojson: Option<&GeoJson>, f: F)
    where F: Fn(&Position) {
    match geojson {
        Some(gj) => {
            match gj {
                GeoJson::FeatureCollection(ref collection) => {
                    for feature in &collection.features {
                        if let Some(ref geometry) = feature.geometry {
                            match_geometry(geometry, &f)
                        }
                    }
                }
                GeoJson::Feature(ref feature) => {
                    if let Some(ref geometry) = feature.geometry {
                        match_geometry(geometry, &f)
                    }
                }
                GeoJson::Geometry(ref geometry) => {
                    match_geometry(geometry, &f)
                }
            }
        }
        None => ()
    }
}
