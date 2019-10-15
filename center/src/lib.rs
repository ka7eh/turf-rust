use bbox::bbox;
use helpers::point;
use geojson::GeoJson;

pub fn center(geojson: &GeoJson) -> GeoJson {
    let bb = bbox(geojson);
    let x = (bb[0] + bb[2]) / 2.0;
    let y = (bb[1] + bb[3]) / 2.0;
    return point(vec![x, y]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path;
    use utils::tests::get_geojsons;

    #[test]
    fn test_center() {
        let collection = get_geojsons();
        for (input, output) in collection {
            assert_eq!(center(&input), output);
        }
    }
}
