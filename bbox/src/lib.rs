use geojson::{Bbox, GeoJson};
use std::f64::{INFINITY, NEG_INFINITY};
use std::sync::{Arc, Mutex};

use meta::coord_each;

pub fn bbox(geojson: &GeoJson) -> Bbox {
    let bbox = Arc::new(Mutex::new(vec![
        INFINITY,
        INFINITY,
        NEG_INFINITY,
        NEG_INFINITY,
    ]));

    coord_each(Some(&geojson), |coord| {
        let mut locked_bbox = bbox.lock().unwrap();
        if locked_bbox[0] > coord[0] {
            locked_bbox[0] = coord[0];
        }
        if locked_bbox[1] > coord[1] {
            locked_bbox[1] = coord[1];
        }
        if locked_bbox[2] < coord[0] {
            locked_bbox[2] = coord[0];
        }
        if locked_bbox[3] < coord[1] {
            locked_bbox[3] = coord[1];
        }
    });

    let result: Bbox = bbox.lock().unwrap().to_vec();

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers;
    use geojson::GeoJson;

    #[test]
    fn test_point() {
        let point = helpers::point(vec![102.0, 0.5]);
        assert_eq!(bbox(&point), vec![102.0, 0.5, 102.0, 0.5]);
    }

    #[test]
    fn test_line() {
        let line = helpers::line_string(vec![
            vec![102.0, -10.0],
            vec![103.0, 1.0],
            vec![104.0, 0.0],
            vec![130.0, 4.0],
        ]);
        assert_eq!(bbox(&line), vec![102.0, -10.0, 130.0, 4.0]);
    }

    #[test]
    fn test_multi_line() {
        let multi_line = helpers::multi_line_string(vec![
            vec![vec![100.0, 0.0], vec![101.0, 1.0]],
            vec![vec![102.0, 2.0], vec![103.0, 3.0]],
        ]);
        assert_eq!(bbox(&multi_line), vec![100.0, 0.0, 103.0, 3.0]);
    }

    #[test]
    fn test_polygon() {
        let polygon = helpers::polygon(vec![vec![
            vec![101.0, 0.0],
            vec![101.0, 1.0],
            vec![100.0, 1.0],
            vec![100.0, 0.0],
            vec![101.0, 0.0],
        ]]);
        assert_eq!(bbox(&polygon), vec![100.0, 0.0, 101.0, 1.0]);
    }

    #[test]
    fn test_multi_polygon() {
        let multi_polygon = helpers::multi_polygon(vec![
            vec![vec![
                vec![102.0, 2.0],
                vec![103.0, 2.0],
                vec![103.0, 3.0],
                vec![102.0, 3.0],
                vec![102.0, 2.0],
            ]],
            vec![
                vec![
                    vec![100.0, 0.0],
                    vec![101.0, 0.0],
                    vec![101.0, 1.0],
                    vec![100.0, 1.0],
                    vec![100.0, 0.0],
                ],
                vec![
                    vec![100.2, 0.2],
                    vec![100.8, 0.2],
                    vec![100.8, 0.8],
                    vec![100.2, 0.8],
                    vec![100.2, 0.2],
                ],
            ],
        ]);
        assert_eq!(bbox(&multi_polygon), vec![100.0, 0.0, 103.0, 3.0]);
    }

    #[test]
    fn test_feature_collection() {
        let feature_collection: GeoJson = helpers::feature_collection(vec![
            helpers::multi_polygon(vec![
                vec![vec![
                    vec![102.0, 2.0],
                    vec![103.0, 2.0],
                    vec![103.0, 3.0],
                    vec![102.0, 3.0],
                    vec![102.0, 2.0],
                ]],
                vec![
                    vec![
                        vec![100.0, 0.0],
                        vec![101.0, 0.0],
                        vec![101.0, 1.0],
                        vec![100.0, 1.0],
                        vec![100.0, 0.0],
                    ],
                    vec![
                        vec![100.2, 0.2],
                        vec![100.8, 0.2],
                        vec![100.8, 0.8],
                        vec![100.2, 0.8],
                        vec![100.2, 0.2],
                    ],
                ],
            ]),
            helpers::multi_line_string(vec![
                vec![vec![100.0, 0.0], vec![101.0, 1.0]],
                vec![vec![102.0, 2.0], vec![103.0, 3.0]],
            ]),
        ]);
        assert_eq!(bbox(&feature_collection), vec![100.0, 0.0, 103.0, 3.0]);
    }
}