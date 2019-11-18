use geojson::{Bbox, GeoJson};
use helpers::{polygon, FeatureOptions, FeatureProperties};

pub fn bbox_polygon(
    bbox: &Bbox,
    properties: Option<FeatureProperties>,
    options: Option<FeatureOptions>,
) -> GeoJson {
    let west = bbox[0];
    let south = bbox[1];
    let east = bbox[2];
    let north = bbox[3];

    let low_left = vec![west.to_owned(), south.to_owned()];
    let top_left = vec![west.to_owned(), north.to_owned()];
    let top_right = vec![east.to_owned(), north.to_owned()];
    let low_right = vec![east.to_owned(), south.to_owned()];

    polygon(
        vec![vec![
            low_left.to_owned(),
            low_right,
            top_right,
            top_left,
            low_left,
        ]],
        properties,
        options,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use geojson::Value;

    #[test]
    fn test_bbox_polygon() {
        let poly = bbox_polygon(&vec![0.0, 0.0, 10.0, 10.0], None, None);

        match poly {
            GeoJson::Feature(feature) => match feature.geometry {
                Some(geometry) => match geometry.value {
                    Value::Polygon(p) => {
                        assert_eq!(p[0][0], vec![0.0, 0.0]);
                        assert_eq!(p[0][1], vec![10.0, 0.0]);
                        assert_eq!(p[0][2], vec![10.0, 10.0]);
                        assert_eq!(p[0][3], vec![0.0, 10.0]);
                    }
                    _ => panic!("Wrong geometry"),
                },
                _ => panic!("Missing geometry"),
            },
            _ => panic!("Wrong feature")
        }
    }
}
