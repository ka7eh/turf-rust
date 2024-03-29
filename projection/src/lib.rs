use geojson::GeoJson;
use meta::each_mut_coord;
use std::f64::consts::{E, PI};

const D2R: f64 = PI / 180.0;
const R2D: f64 = 180.0 / PI;
// 900913 properties
const A: f64 = 6378137.0;
const MAXEXTENT: f64 = 20037508.342789244;

pub fn to_mercator(geojson: &GeoJson) -> GeoJson {
    let mut result = geojson.clone();

    each_mut_coord(&mut result, |lon_lat| {
        // compensate longitudes passing the 180th meridian
        // from https://github.com/proj4js/proj4js/blob/master/lib/common/adjust_lon.js
        let x_adjuster = if lon_lat[0] > 0.0 {
            360.0
        } else if lon_lat[0] < 0.0 {
            -360.0
        } else {
            0.0
        };

        let x_adjusted = if lon_lat[0].abs() <= 180.0 {
            lon_lat[0]
        } else {
            lon_lat[0] - x_adjuster
        };

        lon_lat[0] = (A * x_adjusted * D2R).min(MAXEXTENT).max(-MAXEXTENT);
        lon_lat[1] = (A * ((PI * 0.25) + (0.5 * lon_lat[1] * D2R)).tan().log(E))
            .min(MAXEXTENT)
            .max(-MAXEXTENT);
    });

    result
}

pub fn to_wgs84(geojson: &GeoJson) -> GeoJson {
    let mut result = geojson.clone();

    each_mut_coord(&mut result, |lon_lat| {
        lon_lat[0] = lon_lat[0] * R2D / A;
        lon_lat[1] = ((PI * 0.5) - 2.0 * E.powf(-lon_lat[1] / A).atan()) * R2D;
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::tests::{get_geojsons, GeoJsonFixture};

    #[test]
    fn test_to_mercator() {
        // TODO Fixtures are copied from turfjs and their accuracy is different and needs to be updated.
        let collection = get_geojsons();
        for item in collection {
            let GeoJsonFixture {
                file,
                input,
                output,
            } = item;
            let mut output = output;
            each_mut_coord(&mut output, |lon_lat| {
                lon_lat[0] = lon_lat[0].floor();
                lon_lat[1] = lon_lat[1].floor();
            });
            if file.starts_with("wgs84-") {
                let mut input_mercator = to_mercator(&input);
                each_mut_coord(&mut input_mercator, |lon_lat| {
                    lon_lat[0] = lon_lat[0].floor();
                    lon_lat[1] = lon_lat[1].floor();
                });
                assert_eq!(input_mercator, output, "{}", file);
            }
            if file.starts_with("mercator-") {
                let mut input_wgs84 = to_wgs84(&input);
                each_mut_coord(&mut input_wgs84, |lon_lat| {
                    lon_lat[0] = lon_lat[0].floor();
                    lon_lat[1] = lon_lat[1].floor();
                });
                assert_eq!(input_wgs84, output, "{}", file);
            }
        }
    }
}
