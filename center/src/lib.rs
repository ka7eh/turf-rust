use bbox::bbox;
use geojson::GeoJson;
use helpers::point;
use serde_json::Map;

pub fn center(geojson: &GeoJson) -> GeoJson {
    let bb = bbox(geojson);
    let x = (bb[0] + bb[2]) / 2.0;
    let y = (bb[1] + bb[3]) / 2.0;
    return point(vec![x, y], Some(Map::new()), None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path;
    use utils::tests::{get_geojsons, GeoJsonFixture};

    #[test]
    fn test_center() {
        let collection = get_geojsons();
        for item in collection {
            let GeoJsonFixture {
                file,
                input,
                output,
            } = item;
            assert_eq!(center(&input), output, "{}", file);
        }
    }
}
