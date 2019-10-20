use geojson::{GeoJson, Geometry, Position, Value};

fn match_geometry<F>(geometry: &Geometry, f: &F)
where
    F: Fn(&Position),
{
    match &geometry.value {
        Value::Point(point) => {
            f(&point);
        }
        Value::MultiPoint(points) => {
            for point in points {
                f(&point);
            }
        }
        Value::LineString(line_string) => {
            for point in line_string {
                f(&point);
            }
        }
        Value::MultiLineString(multi_line_string) => {
            for line in multi_line_string {
                for point in line {
                    f(&point);
                }
            }
        }
        Value::Polygon(polygon) => {
            for line in polygon {
                for point in line {
                    f(&point);
                }
            }
        }
        Value::MultiPolygon(multi_polygon) => {
            for polygon in multi_polygon {
                for line in polygon {
                    for point in line {
                        f(&point);
                    }
                }
            }
        }
        Value::GeometryCollection(geometry_collection) => {
            for geometry in geometry_collection {
                match_geometry(&geometry, f)
            }
        }
    }
}

pub fn each_coord<F>(geojson: &GeoJson, f: F)
where
    F: Fn(&Position),
{
    match geojson {
        GeoJson::FeatureCollection(collection) => {
            for feature in &collection.features {
                if let Some(geometry) = &feature.geometry {
                    match_geometry(&geometry, &f)
                }
            }
        }
        GeoJson::Feature(feature) => {
            if let Some(geometry) = &feature.geometry {
                match_geometry(&geometry, &f)
            }
        }
        GeoJson::Geometry(geometry) => match_geometry(&geometry, &f),
    }
}

fn match_mut_geometry<F>(geometry: &mut Geometry, f: &F)
where
    F: Fn(&mut Position),
{
    match &mut geometry.value {
        Value::Point(point) => {
            f(point);
        }
        Value::MultiPoint(points) => {
            for point in points {
                f(point);
            }
        }
        Value::LineString(line_string) => {
            for point in line_string {
                f(point);
            }
        }
        Value::MultiLineString(multi_line_string) => {
            for line in multi_line_string {
                for point in line {
                    f(point);
                }
            }
        }
        Value::Polygon(polygon) => {
            for line in polygon {
                for point in line {
                    f(point);
                }
            }
        }
        Value::MultiPolygon(multi_polygon) => {
            for polygon in multi_polygon {
                for line in polygon {
                    for point in line {
                        f(point);
                    }
                }
            }
        }
        Value::GeometryCollection(gc) => {
            for geometry in gc {
                match_mut_geometry(geometry, f)
            }
        }
    }
}

pub fn each_mut_coord<F>(geojson: &mut GeoJson, f: F)
where
    F: Fn(&mut Position),
{
    match geojson {
        GeoJson::FeatureCollection(collection) => {
            for feature in &mut collection.features {
                if let Some(geometry) = &mut feature.geometry {
                    match_mut_geometry(geometry, &f)
                }
            }
        }
        GeoJson::Feature(feature) => {
            if let Some(geometry) = &mut feature.geometry {
                match_mut_geometry(geometry, &f)
            }
        }
        GeoJson::Geometry(geometry) => match_mut_geometry(geometry, &f),
    }
}
